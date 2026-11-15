# cmdjewel (Asciiverse packaging)

pastthepixels' cmdjewel (Rust, cursive, GPL-3.0), a match-3 game with a
chiptune soundtrack, packaged for the Asciiverse catalog (id `cmdjewel`,
kind `game`, genre `puzzle`). Not our code: keep changes small and
upstreamable, list every one in PATCHES.md and UPSTREAM.md, never change
the game itself.

- `upstream` is codeberg.org/pastthepixels/cmdjewel (branch `main`); ours
  is branch `asciiverse`, `origin` github.com/theasciiverse/cmdjewel.
- Release: tag `v<upstream version>+<upstream commit date>-av.<n>` (first
  `v0.3.0+20261015-av.1`) and push it; `.github/workflows/asciiverse-release.yml`
  builds Linux (glibc, needs libasound2), macOS and Windows.
- Build: needs libasound2-dev and libclang (bindgen for it2play).
  Run with `--mute` and a scratch `XDG_STATE_HOME` when testing.
- Preview: `asciiverse/record-preview.sh target/release/cmdjewel`
  (asciinema 3, tmux, Python 3): drive.py plays Classic with the music off.
