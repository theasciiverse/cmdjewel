mod animations;
mod config;
mod constants;
mod ui;
mod view;

use cursive::{theme::ColorStyle, traits::With};

fn main() {
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
    let stream;
    #[cfg(feature = "sound")]
    {
        use cpal::traits::StreamTrait;
        stream = init_audio();
        stream.play().unwrap();
    }

    // Set title
    siv.set_window_title("cmdjewel");
    // show the start screen
    ui::show_menu_splash(&mut siv);
    // set up commands
    ui::init_commands(&mut siv);
    // Set the refresh rate to 30 FPS and run
    siv.set_autorefresh(true);
    // Run Cursive
    siv.run()
}

#[cfg(feature = "sound")]
fn init_audio() -> cpal::Stream {
    it2play_rs::load_bytes(
        Vec::from(include_bytes!("../cmdjewel.it")),
        it2play_rs::IT2Driver::HQ,
    );
    it2play_rs::play(0);
    it2play_rs::generate_stream()
}
