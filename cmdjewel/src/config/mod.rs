use crate::constants;
use cmdjewel_core::board::{Board, BoardConfig, Gamemode};
use cmdjewel_core::gems::Gem;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::RwLock;
use std::sync::atomic::AtomicBool;
use toml_edit::visit_mut::VisitMut;

pub mod data;
pub mod data_theme;
mod hacks;

static CONFIG_MODIFIED: AtomicBool = AtomicBool::new(false);

/// Loads a config file and returns a corresponding Config struct.
/// If:
///     1. There is no config file
///     2. There is a config file, but it can't be parsed
/// return data::Config::default()
pub fn get_config() -> data::Config {
    /// Check if there's a cached config file
    static CONFIG: RwLock<Option<data::Config>> = RwLock::new(None);
    if CONFIG.read().unwrap().is_some()
        && !CONFIG_MODIFIED.load(std::sync::atomic::Ordering::Relaxed)
    {
        CONFIG.read().unwrap().clone().unwrap()
    } else if let Some(dir) = dirs::config_local_dir() {
        // Load cmdjewel config file
        let dir = dir.join(constants::CONFIG_PATH);
        // If it exists, read it.
        if dir.exists() {
            let file = File::open(dir);
            let mut data = String::new();
            file.unwrap().read_to_string(&mut data).unwrap();
            // Reads the config file, returns data::Config::default() if it can't deserialize
            // TODO made an edit here; maybe we should crash instead lol
            let config = match toml::from_str::<data::Config>(&data) {
                Ok(res) => res,
                Err(err) => panic!("{} {}", constants::strings::ERR_PARSE_CONFIG, err.message()),
            };
            *CONFIG.write().unwrap() = Some(config.clone()); // <- Update cached config
            CONFIG_MODIFIED.store(true, std::sync::atomic::Ordering::Relaxed); // <- Reset modified check; we cached a new config
            config
        } else {
            // Config file doesn't exist
            data::Config::default()
        }
    } else {
        // Unable to get config directory
        data::Config::default()
    }
}

/// Returns the full path of a config file
pub fn config_path() -> Option<PathBuf> {
    dirs::config_local_dir().map(|dir| dir.join(constants::CONFIG_PATH))
}

/// Saves a board to an existing Save. Requires get_save() to return a valid save
pub fn save_board(board: &Board, is_game_over: bool) {
    CONFIG_MODIFIED.store(true, std::sync::atomic::Ordering::Relaxed);
    // Load the config (or the default config)
    let mut cfg = get_config();
    // Create a game save from the board
    let gs = if !is_game_over {
        Some(data::GameSave::new(board))
    } else {
        None
    };
    // Update the save, store
    match board.config_ref().gamemode {
        Gamemode::ZEN => cfg.save.zen = gs,
        Gamemode::CLASSIC => cfg.save.classic = gs,
    };
    // Write to config file
    save_config(&cfg);
}

/// Writes a Config struct to a config file.
/// If the write fails, no exception is thrown. (TODO: probably throw one?)
pub fn save_config(cfg: &data::Config) {
    if let Some(dir) = dirs::config_local_dir() {
        let dir = dir.join(constants::CONFIG_PATH);
        let mut doc = toml_edit::ser::to_document(cfg).unwrap();
        let mut visitor = hacks::HackyFormatter;
        // Write save
        {
            let mut folder = dir.clone();
            folder.pop();
            std::fs::create_dir(folder).unwrap_or_default();
        }
        visitor.visit_document_mut(&mut doc);
        std::fs::write(dir, doc.to_string()).unwrap();
    }
}

/// Deletes the config file, if it exists.
pub fn reset_config() {
    if let Some(dir) = dirs::config_local_dir() {
        let dir = dir.join(constants::CONFIG_PATH);
        if dir.exists() {
            std::fs::remove_file(dir).unwrap();
        }
    }
}

/// Creates a new Board. If a save exists for its gamemode, loads the save. Otherwise, creates a new Board.
pub fn new_board(config: BoardConfig) -> Board {
    // Load the config (or the default config)
    let cfg = get_config();
    // Get game save
    let gs = match config.gamemode {
        Gamemode::ZEN => cfg.save.zen,
        Gamemode::CLASSIC => cfg.save.classic,
    };
    // Create Board
    if let Some(save) = gs {
        // TODO: wow this code sucks
        let mut data = [Gem::Empty; 64];
        let save_data = save.data.iter().map(|g| g.0).collect::<Vec<Gem>>();
        data.copy_from_slice(save_data.as_slice());
        Board::new_controlled(config, data, save.score, save.level, save.level_progress)
    } else {
        Board::new(config)
    }
}
