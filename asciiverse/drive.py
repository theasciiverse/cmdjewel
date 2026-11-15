#!/usr/bin/env python3
"""drive.py SOCKET SESSION: plays a game of cmdjewel Classic in a tmux pane.

Starts Classic from the menu, then reads the board from the pane, waits for
it to settle, and picks a swap: a hypercube when there is one, else the swap
that clears the most gems, so special gems turn up. It walks the cursor
there with the arrow keys at a person's pace, presses Enter for SWAP mode and
an arrow to swap. After about 40 seconds of play it saves and quits with :q.
Run by record-preview.sh while asciinema records the pane."""
import random, subprocess, sys, time

SOCK, S = sys.argv[1], sys.argv[2]
rng = random.Random(11)
PLAY_SECONDS = 40

COLOUR = {}
for colour, glyphs in {"blue": "▼▽", "white": "●○", "red": "■□", "yellow": "◆◇",
                       "green": "⬟⬠", "orange": "⬢⬡", "purple": "▲△"}.items():
    for g in glyphs:
        COLOUR[g] = colour
HYPERCUBE = "◩"


def tmux(*a): return subprocess.run(["tmux", "-L", SOCK, *a], capture_output=True, text=True).stdout
def key(*k): tmux("send-keys", "-t", S, *k)
def lit(s): tmux("send-keys", "-l", "-t", S, s)
def pane(): return tmux("capture-pane", "-p", "-t", S).split("\n")


def board():
    """The 8x8 board as a list of rows of glyphs, or None while it is not readable."""
    rows = pane()
    try:
        top = next(i for i, l in enumerate(rows) if "Level" in l)
    except StopIteration:
        return None
    grid = []
    for line in rows[top:top + 8]:
        parts = line.split("│")
        if len(parts) < 4:
            return None
        cells = parts[2].split()
        if len(cells) != 8:
            return None
        grid.append(cells)
    return grid


def settled():
    """Waits until the board stops changing and is full; returns it."""
    last, same = None, 0
    deadline = time.time() + 6
    while time.time() < deadline:
        b = board()
        if b and "•" not in sum(b, []) and b == last:
            same += 1
            if same >= 2:
                return b
        else:
            same = 0
        last = b
        time.sleep(0.3)
    return last


def cleared(b, x1, y1, x2, y2):
    """How many gems a swap would line up, 0 when it makes no match."""
    g = [row[:] for row in b]
    g[y1][x1], g[y2][x2] = g[y2][x2], g[y1][x1]
    c = lambda x, y: COLOUR.get(g[y][x])
    hit = set()
    for y in range(8):
        for x in range(8):
            col = c(x, y)
            if col is None:
                continue
            for dx, dy in ((1, 0), (0, 1)):
                run = [(x, y)]
                while True:
                    nx, ny = run[-1][0] + dx, run[-1][1] + dy
                    if nx < 8 and ny < 8 and c(nx, ny) == col:
                        run.append((nx, ny))
                    else:
                        break
                if len(run) >= 3:
                    hit.update(run)
    if not ({(x1, y1), (x2, y2)} & hit):
        return 0
    return len(hit)


def best_move(b):
    moves = []
    for y in range(8):
        for x in range(8):
            for dx, dy, arrow in ((1, 0, "Right"), (0, 1, "Down")):
                nx, ny = x + dx, y + dy
                if nx > 7 or ny > 7:
                    continue
                if HYPERCUBE in (b[y][x], b[ny][nx]):
                    moves.append((99, x, y, arrow))
                    continue
                n = cleared(b, x, y, nx, ny)
                if n:
                    moves.append((n + rng.random(), x, y, arrow))
    return max(moves) if moves else None


time.sleep(2.0)
key("Enter")            # Play Game
time.sleep(1.6)
key("Enter")            # Classic
time.sleep(2.0)

cx, cy = 0, 0
start = time.time()
while time.time() - start < PLAY_SECONDS:
    b = settled()
    if not b:
        break
    m = best_move(b)
    if not m:
        break
    _, x, y, arrow = m
    while (cx, cy) != (x, y):
        if cx < x: key("Right"); cx += 1
        elif cx > x: key("Left"); cx -= 1
        elif cy < y: key("Down"); cy += 1
        else: key("Up"); cy -= 1
        time.sleep(rng.uniform(0.09, 0.16))
    time.sleep(rng.uniform(0.25, 0.45))
    key("Enter")        # SWAP mode
    time.sleep(rng.uniform(0.3, 0.45))
    key(arrow)
    time.sleep(0.8)

time.sleep(1.5)
lit(":")
time.sleep(0.6)
lit("q")
time.sleep(0.5)
key("Enter")            # save and quit
