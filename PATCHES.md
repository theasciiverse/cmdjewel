# Changes from upstream

This is the Asciiverse packaging of [cmdjewel](https://codeberg.org/pastthepixels/cmdjewel)
by pastthepixels (GPL-3.0, `LICENSE`), a match-3 game for the terminal in
Rust and cursive, with special gems, a chiptune soundtrack played by
it2play, and Classic and Zen modes. Upstream is the `upstream` remote; this
build is upstream's `main` at `d309551` (2026-10-15), 0.3.0 plus the work
since: the sound feature flag, configurable colour themes (`[settings.theme]`
in the config file) and updated dependencies. The `asciiverse` branch
carries the changes below, all in the `cmdjewel` crate; the game logic in
`cmdjewel_core` is untouched.

## Code

- **State folder.** Settings and saves (`cmdjewel/config.toml`) live in
  `$XDG_STATE_HOME/cmdjewel`, else `~/.local/state/cmdjewel` on Linux and
  the BSDs. macOS and Windows keep upstream's folder (the OS config
  folder) unless `XDG_STATE_HOME` is set. A `config.toml` in the old
  folder (`~/.config/cmdjewel`) is copied over the first time, so saved
  games carry on. The folder is created with its parents (upstream made
  only the last level, so a missing `~/.local/state` would have lost the
  save). Under `cfg(test)` there is no folder, so tests never touch a
  real save. (`src/config/mod.rs`)
- **`--mute`.** Plays without music and opens no audio device; the song is
  still loaded, so the music calls elsewhere stay as they are. `--help`
  lists it; other arguments are logged and ignored, as before.
  (`src/main.rs`)
- **No audio device, no crash.** it2play-rs panics when there is no
  usable output (a server, a container, a machine with no sound card), and
  upstream's `catch_unwind` then ended the game before it started. The
  game now logs the reason and plays silently. (`src/main.rs`)
- **Session logging** with `log` and `env_logger`:
  `RUST_LOG=debug cmdjewel 2>cmdjewel.log`. Every raw input event is
  logged as the terminal delivers it, by a thin wrapper around cursive's
  crossterm backend (`src/session_log.rs`); screen changes, commands,
  volume changes, saves, level ups and game over at info; a snapshot of the
  game (mode, score, level, progress, cursor, cursor mode, animations) once
  a second at debug. cursive's own layout messages are kept at info.
  Without `RUST_LOG`, only errors, as env_logger does by default.
- **Main menu width.** The menu column was capped at 40 columns and the
  logo is 41 wide, so the logo's last stroke (`',_`) wrapped onto a line of
  its own. The cap is now 41. (`src/ui/mod.rs`)

Left as upstream has it: one of the goodbye lines printed on quitting
jokes about the save file in `~/.config/cmdjewel/`; the first-save dialog
shows the real path.

## Packaging (new files)

- `.github/workflows/asciiverse-release.yml`: av archives for Linux
  (glibc, x86-64 and ARM64, built on Ubuntu 22.04), macOS (Intel and Apple
  silicon) and Windows (x86-64 and ARM64), plus SHA256SUMS. Linux is not
  static musl: the music goes out through cpal and ALSA, linked
  dynamically, so the binary needs `libasound2` like our other titles with
  sound. Upstream's `.forgejo` workflow runs only on Codeberg and is left
  in place.
- `asciiverse/record-preview.sh`, `asciiverse/drive.py`: the catalog
  preview, played by a script in a tmux pane with the music off.
- `asciiverse/cmdjewel.toml`, `asciiverse/cmdjewel.md`,
  `asciiverse/cmdjewel.cast`: the catalog manifest, store page and
  preview, drafts for hub.
