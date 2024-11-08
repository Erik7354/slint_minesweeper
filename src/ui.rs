use std::{rc::{Rc}, sync::{Arc, Mutex}};
use slint::{ModelRc, SharedString, VecModel, Weak};

use crate::minesweeper::{Minesweeper, Field, GameStatus};

slint::include_modules!();

/// Runs a Minesweeper game with the corresponding ui.
///
/// This function sets up the main window, initializes the Minesweeper game state, and registers
/// callbacks for tile clicks, game restarts, and game ticks.
pub fn run(ms: Minesweeper) -> Result<(), slint::PlatformError> {
    let mw = MainWindow::new().unwrap();
    let msx: Arc<Mutex<Minesweeper>> = Arc::from(Mutex::from(ms));
    
    // callback: tile clicked
    mw.global::<TileLogic>().on_tile_clicked(tile_clicked_callback(
        mw.as_weak(), 
        msx.clone()
    ));

    // callback: tile right clicked
    mw.global::<TileLogic>().on_tile_right_clicked(tile_right_clicked_callback(
        mw.as_weak(), 
        msx.clone()
    ));

    // callback: restart
    mw.global::<GameLogic>().on_restart(restart_callback(
        mw.as_weak(), 
        msx.clone()
    ));

    // callback: tick
    mw.global::<GameLogic>().on_tick(tick_callback(
        mw.as_weak(), 
        msx.clone()
    ));
    
    // initial settings
    let ms = msx.lock().unwrap();
    mw.set_bombs_text(SharedString::from(format!("{:0>3}", ms.mine_count())));
    mw.set_tiles(board_as_model(ms.board_clone()));
    drop(ms);
    
    // run
    return mw.run();
}

fn board_as_model(board: Vec<Vec<Field>>) -> ModelRc<ModelRc<TileData>> {
    let tiles: ModelRc<ModelRc<TileData>> = Rc::new(VecModel::from(
        board.iter().map(|row| -> ModelRc<TileData> {
            return Rc::new(VecModel::from(
                row.iter().map(|col| -> TileData {
                    let mut td = TileData::default();
                    td.is_flagged = col.is_flagged();
                    td.revealed = col.is_revealed();
                    if td.revealed {
                        td.is_mine = col.is_mine();
                        td.adjacent_mines = col.adjacent_mines() as i32;
                    }
                    return td;
                }).collect::<Vec<TileData>>()
            )).clone().into();
        }).collect::<Vec<ModelRc<TileData>>>()
    )).clone().into();

    return tiles;
}

/// Creates a callback function to handle tile clicks.
///
/// # Arguments
///
/// * `weak_handle` - A weak reference to the main window handle.
/// * `ms` - Rc reference to the Minesweeper game state.
///
/// # Returns
///
/// * A closure that handles the tile click logic, taking the x and y coordinates of the clicked tile as arguments.
fn tile_clicked_callback(weak_handle: Weak<MainWindow>, ms: Arc<Mutex<Minesweeper>>) -> impl Fn(i32, i32) {
    return move |x: i32, y: i32| {
        let mut ms = ms.lock().unwrap();
    
        ms.reveal(x as usize, y as usize);
        let ms_status = ms.status();
        let board = ms.board_clone();
    
        weak_handle.upgrade_in_event_loop( move |handle| {
            handle.set_status(match ms_status {
                GameStatus::Running => UIGameStatus::Running,
                GameStatus::Win => UIGameStatus::Win,
                GameStatus::GameOver => UIGameStatus::GameOver,
            });
    
            handle.set_tiles(board_as_model(board));
        }).unwrap();
    };
}

/// Creates a callback function to handle tile right clicks.
///
/// # Arguments
///
/// * `weak_handle` - A weak reference to the main window handle.
/// * `ms` - Rc reference to the Minesweeper game state.
///
/// # Returns
///
/// * A closure that handles the tile right click logic, taking the x and y coordinates of the clicked tile as arguments.
fn tile_right_clicked_callback(weak_handle: Weak<MainWindow>, ms: Arc<Mutex<Minesweeper>>) -> impl Fn(i32, i32) {
    return move |x: i32, y: i32| {
        let mut ms = ms.lock().unwrap();
    
        ms.flag(x as usize, y as usize);
        let mines = ms.mine_count() - ms.flagged_count();
        let ms_status = ms.status();
        let board = ms.board_clone();
    
        weak_handle.upgrade_in_event_loop( move |handle| {
            handle.set_status(match ms_status {
                GameStatus::Running => UIGameStatus::Running,
                GameStatus::Win => UIGameStatus::Win,
                GameStatus::GameOver => UIGameStatus::GameOver,
            });

            handle.set_bombs_text(SharedString::from(format!("{:0>3}", mines)));
    
            handle.set_tiles(board_as_model(board));
        }).unwrap();
    };
}

/// Creates a callback function to handle game restarts.
///
/// # Arguments
///
/// * `weak_handle` - A weak reference to the main window handle.
/// * `ms` - Rc reference to the Minesweeper game state.
///
/// # Returns
///
/// * A closure that handles the game restart logic.
fn restart_callback(weak_handle: Weak<MainWindow>, ms: Arc<Mutex<Minesweeper>>) -> impl Fn() {
    return move || {
        let mut ms = ms.lock().unwrap();

        ms.restart();
        let mine_count = ms.mine_count();
        let ms_status = ms.status();
        let board = ms.board_clone();

        weak_handle.upgrade_in_event_loop( move |handle| {
            handle.set_timer_running(true);
            handle.set_bombs_text(SharedString::from(format!("{:0>3}", mine_count)));
            handle.set_time_text(SharedString::from("000"));
            handle.set_status(match ms_status {
                GameStatus::Running => UIGameStatus::Running,
                GameStatus::Win => UIGameStatus::Win,
                GameStatus::GameOver => UIGameStatus::GameOver,
            });

            handle.set_tiles(board_as_model(board));
        }).unwrap();
    }
}

/// Creates a callback function to handle game ticks.
///
/// # Arguments
///
/// * `weak_handle` - A weak reference to the main window handle.
/// * `ms` - Rc reference to the Minesweeper game state.
///
/// # Returns
///
/// * A closure that handles the game tick logic.
fn tick_callback(weak_handle: Weak<MainWindow>, ms: Arc<Mutex<Minesweeper>>) -> impl Fn() {
    return move || {
        let ms = ms.lock().unwrap();
        let secs = ms.seconds_running();
        let status = ms.status();

        weak_handle.upgrade_in_event_loop(move |handle| {
            if status != GameStatus::Running {
                handle.set_timer_running(false);
                return
            }

            handle.set_time_text(SharedString::from(format!("{:0>3}", secs)));
        }).unwrap();
    }
}