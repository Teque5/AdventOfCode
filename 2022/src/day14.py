#!/usr/bin/env python3
'''falling sand'''
import time
import numpy as np


def step(buffer, sand_row, sand_col):
    '''down, then down left, then down right, or stop'''
    # drop until where
    for ddx in range(sand_row, buffer.shape[0]):
        #print(ddx, sand_col)
        if buffer[ddx, sand_col] > 1:
            #print('edge')
            #if sand_col - 1 >= 0:
            if buffer[ddx, sand_col-1] == 0:
                # down and to the left is empty
                return step(buffer, ddx, sand_col-1)
            elif buffer[ddx, sand_col+1] == 0:
                # down and to the right is empty
                return step(buffer, ddx, sand_col+1)
            else:
                # stay where you are
                #print('stop',ddx-1, sand_col)
                buffer[ddx-1, sand_col] = 3
                return buffer, True
            break
    return buffer, False


def main(filename, part2=False):
    '''
    sand pouring in at (500,0)
    '''
    # parse rocks
    with open(filename, 'rb') as derp:
        bla = derp.read().strip()
    rocks = []
    xmin, xmax = np.inf, -np.inf
    ymax = 0
    for row in bla.split(b'\n'):
        rockrow = []
        for vert in row.split(b' -> '):
            vx, vy = map(int, vert.split(b','))
            if vx < xmin: xmin = vx
            if vx > xmax: xmax = vx
            if vy > ymax: ymax = vy
            rockrow += [(vx, vy)]
        rocks += [rockrow]
    #print(xmin, xmax, ymax)
    if part2:
        ymax += 2
        xmin -= ymax
        xmax += ymax
    buffer = np.zeros((ymax+1, xmax-xmin+1), dtype=np.uint8)
    # fill with rocks
    for rockrow in rocks:
        vert_prev = rockrow[0]
        for vert_next in rockrow[1:]:
            # draw line from vert to vert
            colstart = min(vert_prev[0], vert_next[0]) - xmin
            colsteps = abs(vert_prev[0] - vert_next[0])
            rowstart = min(vert_prev[1], vert_next[1])
            rowsteps = abs(vert_prev[1] - vert_next[1])
            for cdx in range(colstart, colstart+colsteps+1):
                for rdx in range(rowstart, rowstart+rowsteps+1):
                    buffer[rdx, cdx] = 7 # let 7 be rock
            vert_prev = vert_next
    if part2:
        buffer[ymax] = 2
    ok = True
    acc = 0
    while ok:
        buffer, ok = step(buffer, 0, 500-xmin)
        if ok:
            acc += 1
        if part2 and buffer[0, 500-xmin] != 0:
            ok = False
        if False:
            # export frames for our old friend, FFMPEG
            # ffmpeg -framerate 60 -i frame_177_frame%04d.png out2.webm
            from PIL import Image
            import matplotlib.pyplot as plt
            bufnorm = buffer.astype(np.float32) / 7
            img = Image.fromarray(np.uint8(plt.cm.viridis(bufnorm)*255))
            # upscale small images
            if img.size[0] < 20:
                img = img.resize((img.size[0]*4, img.size[1]*4), Image.NEAREST)
            img.save(f'/tmp/frame_{buffer.shape[0]+1*part2}_frame{acc:04d}.png')
    print(buffer, buffer.shape)
    return acc


if __name__ == '__main__':
    with open('../input/14_val1') as derp:
        val1 = int(derp.read())
    with open('../input/14_val2') as derp:
        val2 = int(derp.read())
    assert main('../input/14_train') == val1
    starttime = time.time()
    print('Part1:', main('../input/14_test'))
    print(f'elap: {1e6*(time.time()-starttime):} µs')
    assert main('../input/14_train', part2=True) == val2
    starttime = time.time()
    print('Part2:', main('../input/14_test', part2=True))
    print(f'elap: {1e6*(time.time()-starttime):} µs')

