#!/usr/bin/env python3
'''find the beacon'''
import time
import numpy as np
import numba
from scipy.optimize import dual_annealing


def main_naive(filename:str, loverow:int=2000000, fullbuffer:bool=False) -> int:
    '''
    create a buffer and just fill it up with sensors and whatnot
    '''
    # parse sensors and whatnot
    with open(filename, 'rb') as derp:
        bla = derp.read().strip()
    xmin, xmax = np.inf, -np.inf
    ymin, ymax = np.inf, -np.inf
    sensors = []
    beacons = []
    for row in bla.split(b'\n'):
        left, right = row.split(b':')
        leftx, lefty = left.split(b',')
        rightx, righty = right.split(b',')
        leftx = int(leftx.split(b'=')[1])
        lefty = int(lefty.split(b'=')[1])
        rightx = int(rightx.split(b'=')[1])
        righty = int(righty.split(b'=')[1])
        ymin = min(ymin, lefty, righty)
        ymax = max(ymax, lefty, righty)
        xmin = min(xmin, leftx, rightx)
        xmax = max(xmax, leftx, rightx)
        sensors += [(leftx, lefty)]
        beacons += [(rightx, righty)]

    # draw map (3x for extra headroom, indent by ymax)
    xspan = (xmax-xmin)//2
    ybonus = (ymax-ymin)//2

    if fullbuffer:
        buffer = np.zeros((ymax-ymin+ybonus*2+1, xmax-xmin+xspan*2+1), dtype=np.uint8)
    bufrow = np.zeros((xmax-xmin+xspan*2+1), dtype=np.uint8)

    if fullbuffer:
        print(buffer.shape, 'buff')
    for (sensor, beacon) in zip(sensors, beacons):
        sx, sy = sensor
        bx, by = beacon
        if sy == loverow:
            bufrow[sx-xmin+xspan] = 7
        if by == loverow:
            bufrow[bx-xmin+xspan] = 5
        if fullbuffer:
            buffer[sy+ybonus, sx-xmin+xspan] = 7
            buffer[by+ybonus, bx-xmin+xspan] = 5

    # draw sensor zones
    for (sensor, beacon) in zip(sensors, beacons):
        sx, sy = sensor
        bx, by = beacon
        dist = matdist(sensor, beacon)
        # fill up adjacent area
        # scan top to bottom
        # if sx != 8: continue
        for odx, offset in enumerate(np.arange(sy-dist+ybonus, sy+ybonus+1)):
            # scan from top to middle
            if fullbuffer:
                minibuf = buffer[offset, sx-xmin-odx+xspan:sx-xmin+1+odx+xspan]
                minibuf[minibuf < 5] = 3
            if offset-ybonus == loverow:
                minibuf = bufrow[sx-xmin-odx+xspan:sx-xmin+1+odx+xspan]
                minibuf[minibuf < 5] = 3
        for odx, offset in enumerate(np.arange(1+sy+ybonus, sy+ybonus+dist+1)):
            # scan from middle to bottom
            if fullbuffer:
                minibuf = buffer[offset, sx-xmin-(dist-odx)+1+xspan:sx-xmin+(dist-odx)+xspan]
                minibuf[minibuf < 5] = 3
            if offset-ybonus == loverow:
                minibuf = bufrow[sx-xmin-(dist-odx)+1+xspan:sx-xmin+(dist-odx)+xspan]
                minibuf[minibuf < 5] = 3

    if fullbuffer:
        printbuf(buffer, xspan, ybonus)
    #print(bufrow)
    return np.sum(bufrow == 3)


def main_optimize(filename:str, zzyzx:int=4000000) -> int:
    '''
    use an optimizer to find the lowest cost
    13360899249595 is correct
    '''
    # parse sensors and whatnot
    with open(filename, 'rb') as derp:
        bla = derp.read().strip()
    xmin, xmax = np.inf, -np.inf
    ymin, ymax = np.inf, -np.inf
    sensors = []
    beacons = []
    dists = []
    for row in bla.split(b'\n'):
        left, right = row.split(b':')
        leftx, lefty = left.split(b',')
        rightx, righty = right.split(b',')
        leftx = int(leftx.split(b'=')[1])
        lefty = int(lefty.split(b'=')[1])
        rightx = int(rightx.split(b'=')[1])
        righty = int(righty.split(b'=')[1])
        ymin = min(ymin, lefty, righty)
        ymax = max(ymax, lefty, righty)
        xmin = min(xmin, leftx, rightx)
        xmax = max(xmax, leftx, rightx)
        sensors += [(leftx, lefty)]
        beacons += [(rightx, righty)]
        dist = matdist((leftx, lefty), (rightx, righty))
        dists += [dist]

    sensors = np.array(sensors, dtype=int)
    dists = np.array(dists, dtype=int)

    fcost = lambda point: cost(point, sensors, dists)
    fval = np.inf
    while fval > .1:
        result = dual_annealing(fcost, bounds=((0, zzyzx), (0, zzyzx)), maxiter=3000)
        fval = result.fun
        front = 'ok!  ' if fval < .1 else '     '
        print(f'{front}x={result.x[0]} y={result.x[1]} fval={result.fun:.2f} iters={result.nit}')
    print('here we go!', result.x[0]*4000000+result.x[1])

    # Plot it
    if False:
        bla = np.empty((zzyzx, zzyzx), dtype=np.int)
        for rdx in range(zzyzx):
            for cdx in range(zzyzx):
                bla[rdx, cdx] = fcost((cdx, rdx))
            print(f'{rdx:2d}', bla[rdx], 'bug?')
        import matplotlib.pyplot as plt
        plt.imshow(bla, interpolation=None, aspect='equal', extent=[0, 20, 0, 20])
        # recall y is flipped w.r.t. sensor locations
        plt.scatter(sensors[:, 0], zzyzx-sensors[:, 1]-1)
        plt.show()

    return int(np.round(result.x[0]*4000000+result.x[1]))


def cost(point, sensors, dists) -> int:
    point_to_sensors = matdistvec(sensors, point)
    # print(dists-point_to_sensors)
    height = dists-point_to_sensors+1
    height[height < 0] = 0
    return np.sum(height)


@numba.njit()
def matdistvec(avec, bbb) -> int:
    '''manhattan distance for a vector of points to a point'''
    return np.sum(np.abs(avec-bbb), axis=1)


@numba.njit
def matdist(aaa, bbb) -> int:
    '''manhattan distance'''
    return abs(aaa[1]-bbb[1]) + abs(aaa[0]-bbb[0])


def printbuf(buffer, xbonus, ybonus) -> None:
    for rdx, row in enumerate(buffer):
        if rdx == 11+ybonus:
            print('       '+' '*(xbonus)+ '0    5    1    1    2')
            print('       '+' '*(xbonus)+ '          0    5    0')
        print(f'{rdx-ybonus:4d} ', end='')
        for item in row:
            print(item, end='')
        print()


def main_polygons(filename:str, zzyzx:int) -> None:
    '''
    just plot circles and inspect, did not work; inscrutable
    '''
    # parse sensors and whatnot
    with open(filename, 'rb') as derp:
        bla = derp.read().strip()
    sensors = []
    beacons = []
    for row in bla.split(b'\n'):
        left, right = row.split(b':')
        leftx, lefty = left.split(b',')
        rightx, righty = right.split(b',')
        leftx = int(leftx.split(b'=')[1])
        lefty = int(lefty.split(b'=')[1])
        rightx = int(rightx.split(b'=')[1])
        righty = int(righty.split(b'=')[1])
        sensors += [(leftx, lefty)]
        beacons += [(rightx, righty)]
    import matplotlib.pyplot as plt
    cdx = -1
    colors = plt.get_cmap('tab20')
    for (sensor, beacon) in zip(sensors, beacons):
        cdx += 1
        # if sensor[0] != 8: continue
        dist0 = matdist(sensor, beacon) +.5
        width = dist0 * np.sqrt(2) / 2
        print(sensor,dist0)
        # plt.axvline(sensor[0])
        # plt.axhline(sensor[1])
        if dist0 > 100000:
            # biggest rects
            rect = plt.Rectangle(
                (sensor[0]-width, sensor[1]-width),
                width*2, width*2, facecolor=colors(cdx),
                edgecolor=None, alpha=.9, angle=45, rotation_point=(sensor[0], sensor[1])
            )
        else:
            rect = plt.Rectangle(
                (sensor[0]-width, sensor[1]-width),
                width*2, width*2, facecolor=colors(cdx),
                edgecolor='k', alpha=.5, angle=45, rotation_point=(sensor[0], sensor[1])
            )
        # circle = plt.Circle((sensor[0], sensor[1]), dguess, fill=False, edgecolor='b', alpha=.1)
        plt.gca().add_artist(rect)
    plt.ylim(0, zzyzx)
    plt.xlim(0, zzyzx)
    plt.grid()
    plt.gca().set_aspect(1)
    plt.show()


if __name__ == '__main__':
    with open('../input/15_val1') as derp:
        val1 = int(derp.read())
    with open('../input/15_val2') as derp:
        val2 = int(derp.read())

    # part 1
    assert main_naive('../input/15_train', loverow=10, fullbuffer=True) == val1
    starttime = time.time()
    print('Part1:', main_naive('../input/15_test', loverow=2000000))
    print(f'elap: {1e6*(time.time()-starttime):} µs')

    # part 2
    assert main_optimize('../input/15_train', zzyzx=21) == val2
    starttime = time.time()
    print('Part2:', main_optimize('../input/15_test', zzyzx=4000001))
    print(f'elap: {1e6*(time.time()-starttime):} µs')
