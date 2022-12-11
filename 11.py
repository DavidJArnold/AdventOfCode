import math
from dataclasses import dataclass
from copy import deepcopy
from functools import reduce

with open("11.real.txt", "r") as f:
    lines = f.read()


@dataclass
class Monkey:
    # keeps track of a single monkey
    id: int
    items: list  # the worry values of each item the monkey is carrying
    test: int  # the test value for division to see where the item gets thrown
    operation: int  # what operation is performed (0: squaring, 1: adding, 2: multiplying)
    num: int  # what value is used in the operation
    if_false: int  # the monkey that gets the item if the test is false
    if_true: int  # if the test is true
    worry_count: int = 0  # running total ofhow many items the monkey has considered


# Parse input
monkeys = []
for monkey in lines.split("\n\n"):
    num = {}
    num2 = {}
    for line in monkey.split("\n"):
        if line.strip().split(" ", 1)[0] == "Monkey":
            id = line[-2]
        elif line.strip().split(" ", 1)[0] == "Starting":
            items = [int(x.strip(",")) for x in line.strip().split(" ")[2:]]
        elif line.strip().split(" ", 1)[0] == "Operation:":
            if line.rsplit(" ", 1)[1] == "old":
                op = 0
                num = 0
                continue
            num = int(line.rsplit(" ", 2)[2])
            if line.rsplit(" ", 2)[1] == "+":
                op = 1
            else:
                op = 2
        elif line.strip().split(" ", 1)[0] == "Test:":
            test = int(line.rsplit(" ", 1)[1])
        elif line.strip().split(" ", 2)[:2] == ["If", "true:"]:
            if_true = int(line.strip().split(" ")[-1])
        elif line.strip().split(" ", 2)[:2] == ["If", "false:"]:
            if_false = int(line.strip().split(" ")[-1])

    monkeys.append(
        Monkey(
            id=id,
            items=items,
            num=num,
            test=test,
            operation=op,
            if_false=if_false,
            if_true=if_true,
        )
    )
# make another copy for part 2
monkeys2 = deepcopy(monkeys)


def operation(op, val, num):
    # Decode the op code and return the function result
    # val = the worry value of the item
    # num = the number for this monkey's operation
    if op == 0:
        return val**2
    elif op == 1:
        return val + num
    elif op == 2:
        return val * num


for _ in range(20):
    for monkey in monkeys:
        for val in monkey.items:
            # iterate through each monkey and each of their items in order
            # do the calculations required
            val2 = operation(monkey.operation, val, monkey.num)
            val3 = math.floor(val2 / 3)
            # decide where the item gets thrown
            if val3 % monkey.test == 0:
                monkeys[monkey.if_true].items.append(val3)
            else:
                monkeys[monkey.if_false].items.append(val3)
            # the monkey has considered an item, increment the count
            monkey.worry_count += 1
        # the monkey has thrown all their items away---clear the list
        monkey.items = []

# get product of two largets worry values
w = [m.worry_count for m in monkeys]
print("Part 1: ", sorted(w)[-1] * sorted(w)[-2])


# For part 2 we run into overflow. To avoid this, we only keep
# track of the value modulo the product of the
# trial division denominators

denom = reduce(lambda x, y: x * y, [t.test for t in monkeys])
for _ in range(10_000):
    for monkey in monkeys2:
        for val in monkey.items:
            # here we don't divide the value by three,
            # but we do get the value modulo the product of denominators
            val2 = operation(monkey.operation, val, monkey.num) % denom
            if val2 % monkey.test == 0:
                monkeys2[monkey.if_true].items.append(val2)
            else:
                monkeys2[monkey.if_false].items.append(val2)
            monkey.worry_count += 1
        monkey.items = []

w = [m.worry_count for m in monkeys2]
print("Part 2: ", sorted(w)[-1] * sorted(w)[-2])
