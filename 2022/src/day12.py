#!/usr/bin/env python3
'''jump start with python for this puzzle'''
import time
import numpy as np

def readmap(filename):
    '''parse monkeys from setup'''
    monkeys = []
    with open(filename, 'rb') as derp:
        bla = derp.read()
    bla2d = bla.strip().split(b'\n')
    rows = len(bla2d)
    cols = len(bla2d[0])
#    print('dbug', bla2d[0])
#    print(rows,cols)
    start = (-1, -1)
    finish = (-1, -1)
    height = np.zeros((rows, cols), dtype=np.int16)
    for rdx, row in enumerate(bla2d):
        for cdx, col in enumerate(row):
            if col == ord('S'):
                col = ord('a')
                start = (rdx, cdx)
            elif col == ord('E'):
                col = ord('z')
                finish = (rdx, cdx)
   #         print(rdx, cdx, col)
            height[rdx, cdx] = col - 97
    return height, start, finish



def walk2(height, start, finish):
    '''calculate distance to all points'''
    dist = np.ones_like(height) * np.inf
    dist[start] = 0
    rows, cols = height.shape
    queue = [start]
    while len(queue) > 0:
        step_current = queue.pop()
        dist_current = dist[step_current]

        if dist_current == np.inf:
            continue

        possible = [
                # up
                (step_current[0], step_current[1]+1),
                # down
                (step_current[0], step_current[1]-1),
                # left
                (step_current[0]-1, step_current[1]),
                # right
                (step_current[0]+1, step_current[1]),
        ]
        for pdx in range(4):
            #invalid
            if possible[pdx][0] < 0 or possible[pdx][0] >= rows:
                pass
            elif possible[pdx][1] < 0 or possible[pdx][1] >= cols:
                pass
            elif (height[possible[pdx]] - height[step_current]) <= 1:
                if (dist[possible[pdx]] > dist_current + 1):
                    dist[possible[pdx]] = dist_current + 1
                    queue += [possible[pdx]]
        #print(dist_current, len(queue))

    #import matplotlib.pyplot as plt
    #plt.imshow(dist);plt.show()
    if dist[finish] < np.inf:
        print('out?', dist[finish])
    return dist[finish]

if __name__ == '__main__':
    with open('../input/12_val1') as derp:
        val1 = int(derp.read())
    height, start, finish = readmap('../input/12_train') 
    # print(walk('../input/12_train'), val1)
    assert walk2(height, start, finish) == val1
    starttime = time.time()
    height, start, finish = readmap('../input/12_test')
    print('Part1:', walk2(height, start, finish))
    print(f'elap: {1e6*(time.time()-starttime):} µs')

    solutions = []
    for rdx, row in enumerate(height):
        for cdx, col in enumerate(row):
            if col == 0:
                solutions += [walk2(height, (rdx, cdx), finish)]
                #print(rdx,cdx,solutions[-1])
    starttime = time.time()
    print('Part2:', np.min(solutions))
    print(f'elap : {1e6*(time.time()-starttime):} µs')
    '''
    with open('../input/12_val2') as derp:
        val2 = int(derp.read())
    #print(part2('../input/12_train'), val2)
    assert part2('../input/12_train') == val2
    print('Part2:', part2('../input/12_test'))
    '''
