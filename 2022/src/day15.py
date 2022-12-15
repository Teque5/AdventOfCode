#!/usr/bin/env python3
'''falling sand'''
import time
import numpy as np



def main(filename, part2=False, loverow=2000000):
    '''
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
    fullbuffer= False
    if fullbuffer:
        buffer = np.zeros((ymax-ymin+ybonus*2+1, xmax-xmin+xspan*2+1), dtype=np.uint8)
    bufrow = np.zeros((xmax-xmin+xspan*2+1), dtype=np.uint8)
    print(bufrow.shape)
    if fullbuffer:
        print(buffer.shape,'buff')
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
#        if sx != 8: continue
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
        printbuf(buffer, ybonus)
    print(bufrow)
    return np.sum(bufrow == 3)   

def printbuf(buffer, ybonus):
    for rdx, row in enumerate(buffer):
        print(f'{rdx-ybonus:4d} ',end='')
        for item in row:
            print(item,end='')
        print()

def matdist(a, b):
    '''manhattan distance'''
    return abs(a[1]-b[1]) + abs(a[0]-b[0])


if __name__ == '__main__':
    with open('../input/15_val1') as derp:
        val1 = int(derp.read())
    #with open('../input/15_val2') as derp:
    #    val2 = int(derp.read())
    #assert main('../input/15_train') == val1
    print(main('../input/15_train', loverow=10))
    starttime = time.time()
    print('Part1:', main('../input/15_test', loverow=2000000))
    print(f'elap: {1e6*(time.time()-starttime):} µs')
    #assert main('../input/15_train', part2=True) == val2
    #starttime = time.time()
    #print('Part2:', main('../input/15_test', part2=True))
    #print(f'elap: {1e6*(time.time()-starttime):} µs')

