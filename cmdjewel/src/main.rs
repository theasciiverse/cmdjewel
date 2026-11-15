mod animations;
mod config;
mod constants;
mod session_log;
mod ui;
mod view;

use cursive::{theme::ColorStyle, traits::With};

/// `--mute`: no music, and no audio device is opened.
static MUTE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() {
    // RUST_LOG=debug logs the game; cursive's own layout messages stay at info
    let mut logger = env_logger::Builder::new();
    if std::env::var_os("RUST_LOG").is_some() {
        logger.filter_module("cursive_core", log::LevelFilter::Info);
    }
    logger.parse_default_env().init();
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--mute" => MUTE.store(true, std::sync::atomic::Ordering::Relaxed),
            "-h" | "--help" => {
                println!("Usage: cmdjewel [--mute]\n\n  --mute  play without music");
                return;
            }
            _ => log::warn!("unknown option {arg}, ignored"),
        }
    }
    log::info!(
        "cmdjewel {} starting, mute {}",
        env!("CARGO_PKG_VERSION"),
        MUTE.load(std::sync::atomic::Ordering::Relaxed)
    );
    match std::panic::catch_unwind(|| start()) {
        Ok(_) => println!("{}", constants::strings::GOODBYE[rand::random_range(0..10)]),
        Err(err) => {
            println!("\x1b[31m{}\x1b[0m", constants::strings::CRASH); // Print the "oh no!" string
            if let Some(err_str) = err.downcast_ref::<&str>() {
                println!("\n{}", err_str)
            }
        }
    }
    cleanup();
}

fn cleanup() {
    // Cleaning up: Reset the background (again on terminals that support it)
    // Konsole (KDE) doesn't support this, but Alacritty and XTerm do. You might find it's a hit-or-miss with your terminal.
    print!("\x1b]111;\x07")
}

fn start() {
    let mut siv = cursive::default();
    // Set a theme (nord) (I copy and pasted this from the git repo and changed some colors)
    let theme = config::get_config().settings.theme.unwrap_or_default();
    siv.set_theme(cursive::theme::Theme {
        shadow: false,
        borders: cursive::theme::BorderStyle::Simple,
        palette: cursive::theme::Palette::retro().with(|palette| {
            use cursive::style::PaletteColor::*;
            palette[Background] = theme.background.clone().into();
            palette[View] = palette[Background];
            palette[Primary] = theme.foreground.into();
            palette[HighlightText] = theme.highlight_text.into();
            palette[TitlePrimary] = theme.title_primary.into();
            palette[Secondary] = theme.inactive_fg.into();
            palette[Tertiary] = theme.inactive_bg.into();
            palette[Highlight] = theme.highlight.into();
            palette[HighlightInactive] = theme.highlight_inactive.into();
            palette[cursive::style::PaletteStyle::EditableText].color = ColorStyle::primary();
        }),
    });
    // Sets the terminal background--uses an ANSI escape sequence to run an Operating System Command (OSC)
    // that gets picked up by your terminal emulator. XTerm (and other emulators) implemented this
    // as a command to change the background.
    println!("\x1b]11;{}\x07", theme.background);
    // Set up music
    #[cfg(feature = "sound")]
    let _stream = init_audio();

    // Set title
    siv.set_window_title("cmdjewel");
    // show the start screen
    ui::show_menu_splash(&mut siv);
    // set up commands
    ui::init_commands(&mut siv);
    // Set the refresh rate to 30 FPS and run
    siv.set_autorefresh(true);
    // Run Cursive, with its input logged (session_log.rs)
    siv.run_with(|| {
        session_log::wrap(
            cursive::backends::crossterm::Backend::init().expect("could not start the terminal"),
        )
    });
    log::info!("quit");
}

/// Loads the soundtrack and opens the audio output. The song is always
/// loaded, so the music calls elsewhere stay valid; with `--mute`, or when no
/// usable audio device is found, no stream is opened and the game is silent.
#[cfg(feature = "sound")]
fn init_audio() -> Option<cpal::Stream> {
    use cpal::traits::StreamTrait;
    it2play_rs::load_bytes(
        Vec::from(include_bytes!("../cmdjewel.it")),
        it2play_rs::IT2Driver::HQ,
    );
    it2play_rs::play(0);
    if MUTE.load(std::sync::atomic::Ordering::Relaxed) {
        return None;
    }
    // it2play_rs panics when there is no device; keep playing without sound
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|info| log::warn!("no sound: {info}")));
    let stream = std::panic::catch_unwind(|| {
        let stream = it2play_rs::generate_stream();
        stream.play().map(|_| stream)
    });
    std::panic::set_hook(hook);
    match stream {
        Ok(Ok(stream)) => Some(stream),
        Ok(Err(e)) => {
            log::warn!("no sound: {e}");
            None
        }
        Err(_) => None,
    }
}
