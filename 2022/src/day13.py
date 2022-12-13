#!/usr/bin/env python3
'''jump start with python for this puzzle'''
import time
import numpy as np

def compare(left, right, verbose=False):
    '''
    0 = bad
    1 = unk
    2 = good
    '''
    if verbose: print('dbug', left,'/////',right)
    if isinstance(left, int) and isinstance(right, int):
        if left < right:
            if verbose: print('+')
            return 2
        elif left == right:
            return 1
        else:
            if verbose: print('-small')
            return 0
    elif isinstance(left, list) and isinstance(right, list):
        for lll, rrr in zip(left, right):
            ok = compare(lll, rrr)
            # special exit early
            if ok == 0: return ok
            elif ok == 2: return ok
        if len(left) < len(right):
            return 2
        elif len(left) > len(right):
            return 0
        else:
            return 1 
        return ok
      
    elif isinstance(left, list) and isinstance(right, int):
        return compare(left, [right])
    elif isinstance(left, int) and isinstance(right, list):
        return compare([left], right) 
    else:
        raise ValueError('not possible')

def parse(filename, part2=False):
    ''''''
    with open(filename, 'rb') as derp:
        bla = derp.read().strip()
    pairs_raw = bla.split(b'\n\n')
    if part2:
        pairs_raw += [b'[[2]]\n[[6]]']
        rows = []
        for ppp in pairs_raw:
            a, b = ppp.split(b'\n')
            rows += [eval(a)]
            rows += [eval(b)]
        import functools
        def cmp(x, y):
            out = compare(x, y)
            if out == 2: return -1
            elif out == 1: return 0
            else: return 1
        keysort = functools.cmp_to_key(lambda x, y: cmp(x, y))
        rows_sorted = sorted(rows, key=keysort)
        return (rows_sorted.index([[2]]) + 1) * (rows_sorted.index([[6]]) + 1)
        #for row in rows_sorted:
        #    print(row)
        #exit(1)
    correct = 0
    for pdx, ppp in enumerate(pairs_raw):
        a, b = ppp.split(b'\n')
        # DANGER
        aa = eval(a)
        bb = eval(b)
        ok = compare(aa, bb)
        # +1 for shitty matlab indexing heathens
        #print('OUT', pdx+1, (ok>1)*'ok', '\n')
        if ok > 1:
            correct += pdx + 1
    return correct

if __name__ == '__main__':
    with open('../input/13_val1') as derp:
        val1 = int(derp.read())
    with open('../input/13_val2') as derp:
        val2 = int(derp.read())
    assert parse('../input/13_train') == val1
    print('Part1:', parse('../input/13_test'))
    assert parse('../input/13_train', part2=True) == val2
    print('Part2:', parse('../input/13_test', part2=True))
