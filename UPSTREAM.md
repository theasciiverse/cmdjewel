# Changes for upstream

Hi! These are the small changes the Asciiverse build of cmdjewel carries,
written so each can be taken on its own or left out. All are in the
`cmdjewel` crate; `cmdjewel_core` is unchanged. PATCHES.md has the full
list with the packaging files.

## 1. Saves in the state folder (`src/config/mod.rs`)

`config.toml` holds saved games as well as settings, so on Linux it fits
the XDG state folder: `$XDG_STATE_HOME/cmdjewel`, else
`~/.local/state/cmdjewel` (`dirs::state_dir()`). macOS and Windows keep
`dirs::config_local_dir()`. A new `state_dir()` replaces the four calls to
`dirs::config_local_dir()`, and on first use copies an existing
`~/.config/cmdjewel/config.toml` across so nobody loses a game.

One fix rides along: `save_config` made the folder with `create_dir`,
which fails when the parent is missing (a fresh `~/.local/state`), and
the write after it then panics. `create_dir_all` makes the whole path.

## 2. `--mute` (`src/main.rs`)

`cmdjewel --mute` loads the song as before but opens no cpal stream, so
the game is silent and never touches the audio device. Handy for playing
in a shared room, over ssh, or for recording. `--help` prints the usage.

## 3. Playing on without an audio device (`src/main.rs`)

`it2play_rs::generate_stream()` panics when there is no default output
device or no f32 stereo config, and the `catch_unwind` in `main` turns
that into "Oh dear! cmdjewel crashed!" before the menu appears. `init_audio`
now catches it (with a quiet panic hook for that call only) and returns
`None`, and the game runs silently. A `Result` from it2play-rs would be
nicer still.

## 4. Session logging (`src/session_log.rs`, small calls elsewhere)

`log` and `env_logger`, off unless `RUST_LOG` is set:
`RUST_LOG=debug cmdjewel 2>cmdjewel.log` gives every raw input event (a
pass-through wrapper around the crossterm backend, started with
`siv.run_with`), screen changes, commands, saves, level ups, game over,
and a once-a-second snapshot of the board state from `BoardView`'s refresh.
It makes bug reports easy to follow, since stderr is where the
troubleshooting guide already points.

## 5. Main menu width (`src/ui/mod.rs`)

The menu's `max_width(40)` is one less than the logo's widest line
(41 columns), so `',_` wrapped to its own line under the logo. `max_width(41)`
keeps the logo whole.
