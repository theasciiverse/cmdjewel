#!/bin/sh
# Records the 100x30 catalog preview, asciiverse/cmdjewel.cast
# (hub/RECORDING.md): a game of Classic played by asciiverse/drive.py,
# about 40 seconds of swaps, then :q to save and quit. The music is off
# (--mute) and the save goes to a throwaway state folder. asciinema
# records a tmux pane (own socket, no status line, no user config) that
# drive.py reads and plays into. Needs asciinema 3, tmux and Python 3.
#   asciiverse/record-preview.sh target/release/cmdjewel
set -eu
binary=$(cd "$(dirname "$1")" && pwd)/$(basename "$1")
here=$(cd "$(dirname "$0")" && pwd)
out=$here/cmdjewel.cast
home=$(mktemp -d)
trap 'tmux -L avrec kill-server 2>/dev/null; rm -rf "$home"' EXIT
mkdir "$home/bin" "$home/state" && cp "$binary" "$home/bin/cmdjewel"
cd "$home"  # nothing in the recording names a local path
export HOME="$home" XDG_STATE_HOME="$home/state" PATH="$home/bin:$PATH" \
  USER=player LOGNAME=player TERM=xterm-256color COLORTERM=truecolor LANG=C.UTF-8
unset XDG_CONFIG_HOME RUST_LOG
tmux -L avrec -f /dev/null new-session -d -s rec -x 100 -y 30 "cmdjewel --mute" \; set -g status off
python3 -I "$here/drive.py" avrec rec &
asciinema rec --headless --overwrite -q \
  --window-size 100x30 --idle-time-limit 2 \
  --capture-env "TERM,COLORTERM,LANG" --title "cmdjewel" \
  --command "tmux -L avrec attach -t rec" "$out"
wait
