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
    } else if let Some(dir) = state_dir() {
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
    state_dir().map(|dir| dir.join(constants::CONFIG_PATH))
}

/// The folder that holds `cmdjewel/config.toml` (settings and saves):
/// `$XDG_STATE_HOME` when it is set, else `~/.local/state` on Linux and the
/// BSDs, else the OS config folder as before (macOS, Windows). A config.toml
/// left in the old folder (`~/.config/cmdjewel`) is copied over the first time.
/// Tests get no folder, so they never touch a real save.
fn state_dir() -> Option<PathBuf> {
    #[cfg(test)]
    return None;
    #[cfg(not(test))]
    {
        static DIR: std::sync::OnceLock<Option<PathBuf>> = std::sync::OnceLock::new();
        DIR.get_or_init(|| {
            let xdg = std::env::var_os("XDG_STATE_HOME")
                .map(PathBuf::from)
                .filter(|p| p.is_absolute());
            #[cfg(all(unix, not(target_os = "macos")))]
            let dir = xdg.or_else(dirs::state_dir);
            #[cfg(not(all(unix, not(target_os = "macos"))))]
            let dir = xdg.or_else(dirs::config_local_dir);
            if let (Some(new), Some(old)) = (&dir, dirs::config_local_dir()) {
                let (new, old) = (new.join(constants::CONFIG_PATH), old.join(constants::CONFIG_PATH));
                if new != old && !new.exists() && old.exists() {
                    if let Some(parent) = new.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    match std::fs::copy(&old, &new) {
                        Ok(_) => log::info!("copied {} to {}", old.display(), new.display()),
                        Err(e) => log::warn!("could not copy {}: {}", old.display(), e),
                    }
                }
            }
            log::info!("state folder: {:?}", dir.as_ref().map(|d| d.join("cmdjewel")));
            dir
        })
        .clone()
    }
}

/// Saves a board to an existing Save. Requires get_save() to return a valid save
pub fn save_board(board: &Board, is_game_over: bool) {
    log::info!(
        "save {}: score {} level {} game over {}",
        board.config_ref().name,
        board.get_score(),
        board.get_level() + 1,
        is_game_over
    );
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
    if let Some(dir) = state_dir() {
        let dir = dir.join(constants::CONFIG_PATH);
        let mut doc = toml_edit::ser::to_document(cfg).unwrap();
        let mut visitor = hacks::HackyFormatter;
        // Write save
        {
            let mut folder = dir.clone();
            folder.pop();
            std::fs::create_dir_all(folder).unwrap_or_default();
        }
        visitor.visit_document_mut(&mut doc);
        std::fs::write(dir, doc.to_string()).unwrap();
    }
}

/// Deletes the config file, if it exists.
pub fn reset_config() {
    log::info!("settings reset: config file deleted");
    if let Some(dir) = state_dir() {
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
