#!/usr/bin/env python3
'''find the beacon'''
import time
import numpy as np
import numba
from scipy.optimize import dual_annealing
import torch
import pytorch_lightning as pl


def main(filename:str, part2=False) -> int:
    '''
    create a buffer and just fill it up with sensors and whatnot
    '''
    # parse sensors and whatnot
    with open(filename, 'rb') as derp:
        bla = derp.read().strip()
    lut = {}
    flows_ppm = []
    paths = []

    for rdx, row in enumerate(bla.split(b'\n')):
        left, right = row.split(b';')
        left_ppm = int(left.split(b'=')[1])
        left_name = left[6:8]
        lut[left_name] = rdx
        flows_ppm += [left_ppm]
    for rdx, row in enumerate(bla.split(b'\n')):
        left, right = row.split(b';')
        rights = right.replace(b'valves', b'valve').split(b'valve ')[1].split(b', ')
        path = [lut[right] for right in rights]
        paths += [path]
    print(lut)
    print('flows',flows_ppm)
    print('paths',paths)
    path_count = len(paths)

    # find path to open optimal tunnels
    best_routes = []
    best_pressure = -np.inf
    for _ in range(10000000):
        pressure, route = tryit(flows_ppm, paths, path_count, best_routes)
        if pressure > best_pressure:
            best_pressure = pressure
            best_routes += [(best_pressure, route)]
            if len(best_routes) > 10:
                best_routes = sorted(best_routes) # sorts on first value by default
                best_routes.pop(-1)
            for bbb in best_routes:
                print(bbb)

    # DL approach
    # inputs are position


@numba.jit(forceobj=True, parallel=True)
def tryit(flows_ppm, paths, path_count, best_routes):
    open_status = [False] * path_count
    route = [] # current route
    pressure = 0 # current pressure
    ppm = 0 # current ppm
    place = 0 # current pos
    for minute in range(30):
        pressure += ppm
        ''''
        rand = np.random.random()
        if rand < .2 and len(best_routes):
            # copy previous choice here
            choice = np.random.randint(len(best_routes))
            place = best_routes[choice][1][minute]
            if place >= 100:
                place -= 100
                #open valve
                if open_status[place] == False:
                    open_status[place] = True
                    ppm += flows_ppm[place]
                route += [place+100]
            else:
                route += [place]
        '''
        if np.random.random() > .5:
            # move to random valve
            place = np.random.choice(paths[place])
            route += [place]
        else:
            route += [place+100] # not sure i need to track this
            # open valve
            if open_status[place] == False:
                open_status[place] = True
                ppm += flows_ppm[place]
    return pressure, route
       



if __name__ == '__main__':
    with open('../input/16_val1') as derp:
        val1 = int(derp.read())
#    with open('../input/16_val2') as derp:
#        val2 = int(derp.read())

    # part 1
    starttime = time.time()
#    assert main('../input/16_train') == val1
    print(main('../input/16_train'))
#    print('Part1:', main('../input/16_test'))
    print(f'elap: {1e6*(time.time()-starttime):} µs')

    # part 2
#    assert main_optimize('../input/15_train', zzyzx=21) == val2
#    starttime = time.time()
#    print('Part2:', main_optimize('../input/15_test', zzyzx=4000001))
#    print(f'elap: {1e6*(time.time()-starttime):} µs')
