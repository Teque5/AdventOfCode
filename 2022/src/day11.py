#!/usr/bin/env python3
'''jump start with python for this puzzle'''
from sympy.parsing.sympy_parser import parse_expr

class monkey:
    def __init__(self, items, op, divisor, action_true, action_false, part2=False):
        self.items = items
        self.op = op
        self.divisor = divisor
        self.action_true = action_true
        self.action_false = action_false
        self.part2 = part2
        self.inspect = 0

    def forward(self):
        outs = []
        for item in self.items:
            self.inspect += 1
            newitem = int(self.op.evalf(subs={'x':item}))
            if not self.part2:
                newitem //= 3
            if newitem % self.divisor == 0:
                # divisible
                outs += [(self.action_true, newitem)]
            else:   
                # not divisible
                outs += [(self.action_false, newitem)]
        self.items = []
        return outs

    def add_item(self, newitem):
        self.items += [newitem]

    def __repr__(self):
        if self.items:
            return f'M:{len(self.items)}'
        else:
            return 'MX'

def readfile(filename, part2=False):
    monkeys = []
    with open(filename, 'rb') as derp:
        bla = derp.read()
    raw_monkeys = bla.split(b'\n\n')
    for raw in raw_monkeys:
        row = raw.split(b'\n')
        items = list(map(int, row[1][18:].split(b', ')))
        #expr = parse_expr(row[2][19:].replace(b'old', b'x').replace(b' ', b'').decode())
        #print(expr)
        #op = lambda x: expr.evalf(subs={'x': x})
        op = parse_expr(row[2][19:].replace(b'old', b'x').replace(b' ', b'').decode())
        divisor = int(row[3].split(b' ')[-1])
        action_true = int(row[4].split(b' ')[-1])
        action_false = int(row[5].split(b' ')[-1])
        monkeys += [monkey(items, op, divisor, action_true, action_false, part2=part2)]
    return monkeys


def part1(filename):
    monkeys = readfile(filename)
#    print(monkeys[0].forward())
    for _ in range(20):
        for mmm in monkeys:
            outs = mmm.forward()
            for mdx, item in outs:
                monkeys[mdx].add_item(item)
    inspect = [mmm.inspect for mmm in monkeys]
    inspect.sort()
    a, b = inspect[-2:]
    return a * b

def part2(filename):
    monkeys = readfile(filename, part2=True)
    multi = 1
    for mmm in monkeys:
        multi *= mmm.divisor
    for _ in range(10000):
        for mmm in monkeys:
            outs = mmm.forward()
            for mdx, item in outs:
                item %= multi # reduce insanely huge values
                monkeys[mdx].add_item(item)
        if False:
            # debug only
            for mmm in monkeys:
                print(mmm.items)
    inspect = [mmm.inspect for mmm in monkeys]
    print(inspect)
    inspect.sort()
    a, b = inspect[-2:]
    return a * b

if __name__ == '__main__':
    with open('../input/11_val1') as derp:
        val1 = int(derp.read())
    #print(part1('../input/11_train'), val1)
    assert part1('../input/11_train') == val1
    print('part1:', part1('../input/11_test'))

    with open('../input/11_val2') as derp:
        val2 = int(derp.read())
    #print(part2('../input/11_train'), val2)
    assert part2('../input/11_train') == val2
    print('part2:', part2('../input/11_test'))
