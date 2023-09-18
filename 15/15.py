with open("15.real.txt", "r") as f:
    input = f.read().strip()

test_row = 2_000_000
range_limit = 4_000_000


def dist(a, b):
    # Manhattan distance between two points
    return abs(a[0] - b[0]) + abs(a[1] - b[1])


def part1(test_row: int = 2_000_000):
    blocked = set()  # set of x in test_row that can't have a beacon
    blockedSB = set()  # set of x in test_row that have a sensor or beacon
    for line in input.strip().split("\n"):
        # each row is like
        # Sensor at x=13, y=12: closest beacon is at x=-2, y=15
        data = [line.split(" ")[idx] for idx in [2, 3, 8, 9]]
        S = [int(data[0][2:-1]), int(data[1][2:-1])]
        B = [int(data[2][2:-1]), int(data[3][2:])]
        # calculate manhattan distance from sensor to beacon
        distSB = dist(B, S)

        # record sensor.beacon positions---these get excluded from the count
        if B[1] == test_row:
            blockedSB.add(B[0])
        if S[1] == test_row:
            blockedSB.add(S[0])

        # a sensor is only relevant to test_row if it is close enough
        dist_limit = distSB - abs(S[1] - test_row)
        if dist_limit >= 0:
            # we can add these positions to the blocked positions set
            for i in range(S[0] - dist_limit, S[0] + dist_limit + 1):
                blocked.add(i)
    return len(blocked) - len(blockedSB)


print("Part 1: ", part1(test_row=test_row))


def parse(input=input):
    # returns a list of tuples of sensor locations (lists)
    # and their corresponding distance to closest beacon
    sensors = []
    for line in input.strip().split("\n"):
        data = [line.split(" ")[idx] for idx in [2, 3, 8, 9]]
        S = [int(data[0][2:-1]), int(data[1][2:-1])]
        B = [int(data[2][2:-1]), int(data[3][2:])]
        distSB = dist(S, B)
        sensors.append((S, distSB))
    return sensors


def make_circle(centre, radius, range_limit):
    # return points on the radius of a circle* with centre centre
    # and radius radius, excluding points outside [0, range_limit]
    #
    # * in the taxicab metric

    radius_list = range(radius + 1)
    perim = set()
    for x in radius_list:
        y = radius - x
        for offset in [(1, 1), (-1, 1), (1, -1), (-1, -1)]:
            new_x, new_y = (centre[0] + offset[0] * x, centre[1] + offset[1] * y)
            if (
                new_x >= 0
                and new_y >= 0
                and new_x <= range_limit
                and new_y <= range_limit
            ):
                perim.add((new_x, new_y))
    return perim


def find_beacon(range_limit, sensors):
    # the beacon must be at the intersection of 4 circles,
    # centred on sensors with radius d+1 (where d is the
    # distance from the sensor to the nearest beacon)
    for sensor in sensors:
        # create circle radius d+1 around the sensor (excluding points below 0 and above the limit)
        perim = make_circle(sensor[0], sensor[1] + 1, range_limit)
        for p in perim:
            # check each point along the perimeter
            valid = True
            for o in sensors:
                # calculate distance to the sensor
                d = dist([p[0], p[1]], o[0])
                if d <= o[1]:
                    # if it's less than the distance to the closest beacon,
                    # there can't be a beacon here -> move to next point on perimeter
                    valid = False
                    break
            if valid:
                return p


beacon = find_beacon(range_limit, parse(input))
print(f"Part 2: {str(4 * beacon[0])}{str(beacon[1])}")
