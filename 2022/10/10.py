import math


with open("10.real.txt", "r") as f:
    lines = [x.split(" ") for x in f.read().split("\n")][:-1]

vals = [1]  # starting value
for line in lines:
    vals.append(vals[-1])  # any operation repeats the last element
    if len(line) == 2:
        # if it's an addx, it takes an extra operation
        # and extends the vector with the new result
        vals.append(vals[-1] + int(line[1]))
print("Part 1: ", sum([vals[20 * x - 1] * 20 * x for x in [1, 3, 5, 7, 9, 11]]))

SCREEN_WIDTH = 40
SCREEN_HEIGHT = 6


def draw_screen(draw_pos, sprite_centre, screen):
    # given sprite position and drawing position
    # works out whether or not to draw the pixel
    sprite_pos = [
        # sprite_centre tracks horizontal position IN A ROW
        # so you have to add 40 * row_number to get the index in
        # the screen array. This confuesd me for a long time.
        # the sprite has width 3 so get a vector with its positions.
        sprite_centre + SCREEN_WIDTH * math.floor(draw_pos / SCREEN_WIDTH) + x
        for x in [-1, 0, 1]
    ]
    if draw_pos in sprite_pos:
        screen[draw_pos] = "#"
    else:
        screen[draw_pos] = "."
    return screen


sprite_pos = [0, 1, 2]
draw_pos = 0
screen = ["" for _ in range(SCREEN_WIDTH * SCREEN_HEIGHT)]
for val in vals:
    # we already found how the sprite centre moves in Part 1
    screen = draw_screen(draw_pos, val, screen)
    draw_pos += 1
    if draw_pos >= SCREEN_HEIGHT * SCREEN_WIDTH:
        # we have reached the end of the screen
        break
print("Part 2:")
[  # format the output row-by-row
    print("".join(screen[i * SCREEN_WIDTH : (i + 1) * SCREEN_WIDTH]))
    for i in range(SCREEN_HEIGHT)
]
