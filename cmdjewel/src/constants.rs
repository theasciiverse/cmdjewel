pub const CONFIG_PATH: &str = "cmdjewel/config.toml";

pub mod strings {
    pub const CMDJEWEL_LOGO: &str = "
               ,   .                _.
  __  ,   ,  _.| __.  __  ,   ,  __  |
 /  ' |\\ /| /  |   | /__' | , | /__' |
 \\__, | ' | \\_,|   , \\__, \\/ \\/ \\__, ',_
                 -'
";
    pub const LOGO_GEMS: &str = "
       ●   ◆   ⬟   ▼   ■   ⬢   ▲
";
    pub const CLASSIC: &str = "Classic";
    pub const CLASSIC_DESC: &str =
        "A classic game of cmdjewel. Match 3 (or more) gems in a row until you run out of moves.";
    pub const ZEN: &str = "Zen";
    pub const ZEN_DESC: &str = "Like Classic, but you can't run out of moves.";
    pub const MSG_WELCOME: &str =
        "Welcome to cmdjewel!\nUse the arrow keys and enter to move around.";
    pub const MAIN_MENU: &str = "Main Menu";
    pub const QUIT: &str = "Quit";
    pub const OK: &str = "Ok";
    pub const PLAY: &str = "Play Game";
    pub const HELP: &str = "Help";
    pub const LEVEL: &str = "Level";
    pub const SCORE: &str = "Score";
    pub const HINT: &str = "Hint";
    pub const COMMAND: &str = "Command";
    pub const CMD_NOT_FOUND: &str =
        "Command not found. Available commands are main/m, play/p [classic/zen], q[a/!], hint/h";
    pub const KEY_NOT_FOUND: &str =
        "Key not recognized. Use the arrow keys to move and the enter key to enter SWAP mode.";
    pub const SETTINGS: &str = "Settings";
    pub const BACK: &str = "Back";
    pub const RESET: &str = "Reset";
    pub const YES: &str = "Yes";
    pub const NO: &str = "No";
    pub const MUSIC_VOL: &str = "Music volume";
    pub const TROUBLESHOOTING: &str = "Troubleshooting";
    pub const TUTORIAL: &str = "How to play";
    pub const TUTORIAL_1: &str = "Score as many points as possible until there are no more moves.";
    pub const TUTORIAL_2: &str = "Swap adjacent gems to make rows of three.";
    pub const TUTORIAL_3: &str = "Match 4 or more gems to create Special Gems.";
    pub const TUTORIAL_4: &str = "Make an L or T match to create a Star Gem!";
    pub const HELP_TUTORIAL: &str = "Press the tutorial button to learn how to play cmdjewel.";
    pub const HELP_CMDJEWEL: &str =
        "cmdjewel is a match-3 game for terminal emulators, inspired by Bejeweled.";
    pub const HELP_CONTROLS: &str = "You can use your mouse with cmdjewel. Try clicking on buttons! Click and swipe to match gems. If you're using the keyboard, press SPACE or ENTER in-game to enter swap mode - where directional keys will swap gems. You can even use Vim keys (h, j, k, and l) as directional keys.";
    pub const WARN_RESET: &str =
        "This will delete your config file, including all your saved games.";
    pub const ARE_SURE: &str = "Are you sure?";
    pub const GOODBYE: [&str; 10] = [
        "Goodbye!",
        "Thanks for playing cmdjewel!",
        "See you again soon!",
        "Thanks for all your support <3",
        "Aw man, you quit *this* early?",
        "[INSERT WITTY GOODBYE ONE-LINER.]",
        "Bye for now! Maybe you should mess with your save file in ~/.config/cmdjewel/. I mean what? I didn't say anything.",
        "Seeya round!",
        "▼ Goodby--Aw whups I dropped this gem...",
        "Stick around for Bejeweled 4, coming in the year 3000! Now with gambling and in-app purchases!",
    ];
    pub const CRASH: &str = "Oh dear! cmdjewel crashed! Look at the error to get some clues, or open a bug at https://codeberg.org/pastthepixels/cmdjewel/issues";

    // Errors :(
    pub const ERR_PARSE_CONFIG: &str = "[ERROR] Unable to parse config file!";

    pub fn first_save(path: &str) -> String {
        format!(
            "cmdjewel just created a save file at {}. When you return, it'll load your game from that path.",
            path
        )
    }

    pub fn game_over(score: u32, level: u8) -> String {
        format!(
            "Game over! You scored {} points and got to level {}.",
            score, level
        )
    }
}

pub mod gems {
    use cmdjewel_core::gems::{Gem, GemColor};
    use cursive::style::{Color, ColorStyle};

    use crate::config;

    /// Gets a printable string from a game::Gems.
    /// This doesn't belong in board as that file only contains game logic and nothing user-facing.
    pub fn gem_string(gem: Gem) -> String {
        match gem {
            Gem::Empty => "•",
            Gem::Normal(x) => match x {
                GemColor::Blue => "▼",
                GemColor::White => "●",
                GemColor::Red => "■",
                GemColor::Yellow => "◆",
                GemColor::Green => "⬟",
                GemColor::Orange => "⬢",
                GemColor::Purple => "▲",
            },
            Gem::Flame(x) => match x {
                GemColor::Blue => "▽",
                GemColor::White => "○",
                GemColor::Red => "□",
                GemColor::Yellow => "◇",
                GemColor::Green => "⬠",
                GemColor::Orange => "⬡",
                GemColor::Purple => "△",
            },
            Gem::Star(_) => "★",
            Gem::Supernova(_) => "☆",
            Gem::Hypercube(_) => "◩",
        }
        .into()
    }

    /// Gets a ColorStyle given a game::Gems
    pub fn gem_color(gem: Gem) -> ColorStyle {
        let theme = config::get_config().settings.theme.unwrap_or_default();
        match gem {
            Gem::Empty => ColorStyle::new(
                Into::<Color>::into(theme.grey),
                Into::<Color>::into(theme.background),
            ),
            Gem::Normal(x) => colorstyle_from_gemcolor(x),
            Gem::Flame(x) => colorstyle_from_gemcolor(x),
            Gem::Star(x) => colorstyle_from_gemcolor(x),
            Gem::Supernova(x) => colorstyle_from_gemcolor(x),
            Gem::Hypercube(_) => ColorStyle::new(
                Into::<Color>::into(theme.highlight_text),
                Into::<Color>::into(theme.grey),
            ),
        }
    }

    /// Returns a ColorStyle from a game::GemColor
    fn colorstyle_from_gemcolor(gem_color: GemColor) -> ColorStyle {
        let theme = config::get_config().settings.theme.unwrap_or_default();
        let background = Into::<Color>::into(theme.background);
        match gem_color {
            GemColor::Blue => ColorStyle::new(Into::<Color>::into(theme.blue), background),
            GemColor::White => ColorStyle::new(Into::<Color>::into(theme.foreground), background),
            GemColor::Red => ColorStyle::new(Into::<Color>::into(theme.red), background),
            GemColor::Yellow => ColorStyle::new(Into::<Color>::into(theme.yellow), background),
            GemColor::Green => ColorStyle::new(Into::<Color>::into(theme.green), background),
            GemColor::Orange => ColorStyle::new(Into::<Color>::into(theme.orange), background),
            GemColor::Purple => ColorStyle::new(Into::<Color>::into(theme.purple), background),
        }
    }
}
