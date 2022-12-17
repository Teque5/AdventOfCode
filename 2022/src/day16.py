#!/usr/bin/env python3
'''relieve the pressure'''
import time
import numpy as np
#import numba
#from scipy.optimize import dual_annealing
#import torch
#import pytorch_lightning as pl


def main(filename:str, maxiters=10_000_000, part2=False) -> int:
    '''
    create a buffer and just fill it up with sensors and whatnot
    '''
    # parse sensors and whatnot
    with open(filename, 'rb') as derp:
        bla = derp.read().strip()
    tunnels = []
    flows_ppm = []
    paths = []

    for rdx, row in enumerate(bla.split(b'\n')):
        left, right = row.split(b';')
        left_ppm = int(left.split(b'=')[1])
        left_name = left[6:8]
        tunnels += [left_name]
        flows_ppm += [left_ppm]
    for rdx, row in enumerate(bla.split(b'\n')):
        left, right = row.split(b';')
        rights = right.replace(b'valves', b'valve').split(b'valve ')[1].split(b', ')
        path = [tunnels.index(right) for right in rights]
        paths += [path]
#    print('flows',flows_ppm)
#    print('paths',paths)
#    print(path_count, tunnels)
    path_count = len(paths)
    startplace = tunnels.index(b'AA')

    # find path to open optimal tunnels
    best_routes = []
    best_pressure = -np.inf
    for tdx in range(maxiters):
        rand = np.random.random()
        # at the start use all new routes, but toward the end use up to 50% of old routes
        if len(best_routes) and rand < (tdx/maxiters)/2:
            # chance to rewind prior route increases as we go
            rdx = np.random.randint(len(best_routes))
            prior_route = best_routes[rdx][1]
            if rand < .1:
                fast_forward = np.random.randint(4) # last part less important
            if .1 < rand < .2:
                fast_forward = np.random.randint(6) # last part less important
            if .2 < rand < .3:
                fast_forward = np.random.randint(12) # last part less important
        else:
            prior_route = []
            fast_forward = 0
        pressure, route = smart_route(flows_ppm, paths, path_count, startplace, fast_forward, prior_route)
        if pressure > best_pressure:
            best_pressure = pressure
            best_routes += [(pressure, route)]
            if len(best_routes) > 5:
                # sort big to small
                best_routes = sorted(best_routes, reverse=True)
                # drop smallest
                best_routes.pop(-1)
            if True:
                print(f'iter={tdx:9d} pressure={pressure:5d}', end='   ')
                for bb in route:
                    if bb > 100:
                        # valve on
                        print(f'//', end=' ')
                    else:
                        print(f'{tunnels[bb].decode()}', end=' ')
                print()

    
def all_routes(paths, place, prev_routes, max_minutes=8):
    '''
    find branches from current path
    not feasable -> garbage
    '''
    new_routes = ()
    # +1 for open current valve
    for route in prev_routes:
        if len(route) < max_minutes:
            # add next point
            possible_paths = list(paths[place])
            if len(route) != 0:
                # allow +100 if not previous
                if (place + 100) not in route:
                    possible_paths += [place+100]
            for nextplace in possible_paths:
                new_route = (route + (nextplace,),)
                if nextplace >= 100:
                    nextplace -= 100
                new_routes += create_routes(paths, nextplace, new_route, max_minutes=max_minutes)
        else:
            # add prev_route (shortcut to exit)
            new_routes += (route,)
    return(new_routes)


def smart_route(flows_ppm, paths, path_count, startplace, fast_forward=0, prior_route=[]):
    '''
    resolve a random route with optional fast forwarding
    note: slower with numba due to random calls
    '''
    open_status = [False] * path_count
    route = np.empty(30, dtype=np.uint8) # current route
    pressure = 0 # current pressure
    ppm = 0 # current ppm
    place = startplace # current pos
    for minute in range(30):
        pressure += ppm
        if fast_forward > minute:
            # follow set path
            prior_place = prior_route[minute]
            route[minute] = prior_place
            if prior_route[minute] < 100:
                place = prior_place
            else:
                open_status[prior_place-100] = True
                ppm += flows_ppm[prior_place-100]
        else:
            rand = np.random.random()
            if flows_ppm[place] == 0 or open_status[place]:
                # do not open valve if no pressure change or valve already open
                rand = 0
            if rand < .5:
                # move to random valve
                rdx = np.random.randint(len(paths[place]))
                place = paths[place][rdx]
                route[minute] = place
            else:
                route[minute] = place + 100
                # open valve
                if open_status[place] == False:
                    open_status[place] = True
                    ppm += flows_ppm[place]
    return pressure, route
       

# best for part 1:
# 1546   WR KM // JF SD CN // SD JF KM TX ZB IC // ZZ EK GB // EK ZZ IC BS JN BV // OV DS // AW DS

if __name__ == '__main__':
    day = 16

    # part 1
    with open(f'../input/{day}_val1') as derp:
        val1 = int(derp.read())
    result = main(f'../input/{day}_train', maxiters=5_000_000)
    print('Val1:', result)
    assert result == val1
    starttime = time.time()
    # 100M evals is about an hour; best solutions were 1516, 1546, 1549
    print('Part1:', main(f'../input/{day}_test'))
    print(f'elap: {1e6*(time.time()-starttime):} µs')

    # part 2
#    with open(f'../input/{day}_val2') as derp:
#        val2 = int(derp.read())
#    result = main(f'../input/{day}_train', part2=True)
#    assert result == val2
#    starttime = time.time()
#    print('Part2:', main_optimize(f'../input/{day}_test', part2=True))
#    print(f'elap: {1e6*(time.time()-starttime):} µs')
