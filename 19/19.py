from typing import List, Tuple, Callable
from time import time
from functools import lru_cache
import math


def get_input(filename="19.real.txt"):
    with open(filename, "r") as f:
        return f.read().strip()


def parse(input: str) -> List[List[int]]:
    blueprints = []
    for blueprint in input.split("\n"):
        words = blueprint.split(" ")
        costs = []
        for word in words:
            try:
                costs.append(int(word))
            except ValueError:
                pass
        blueprints.append(costs)
    return blueprints


def find_geodes(
    blueprint: List[int],
    max_time: int = 24
) -> Callable:
    # wrapper for cached recursive function

    # pre-compute a constant
    max_ore = max(blueprint[idx] for idx in [0, 1, 2, 4])

    @lru_cache(maxsize=None)
    def recursive(
        current_time: int = 0,
        state: Tuple = (0, 0, 0, 0, 1, 0, 0, 0),
        best_score: int = 0,
    ) -> int:
        # given a list of costs, find number open
        # geodes that can be opened in num_minutes
        # blueprint is
        # [ore/ore, ore/clay, ore/obs, clay/obs, ore/geo, obs/geo]
        # state is number of [ore, clay, obs, geo]
        # followed by number of [ore, clay, obs, geo] robots
        if current_time + 1 == max_time:
            # can't build another robot -> calcualte final value and return
            return state[3] + state[7]

        # common part of upper bound calculation
        ub = state[3] + state[7]*(max_time-current_time)

        # flag to indicate if a robot has been built in this recursive call
        build_anything = False

        # go through each robot and work when the next one can be built
        # build ore robot
        if state[4] < max_ore:
            # only build if we have less robots than the maximum ore cost of a robot
            # how long until we could build it with current robot and ore supply?
            time_until_build = max(0, math.ceil((blueprint[0] - state[0])/state[4])) + 1
            # time_left after the robot is built
            time_left = max_time - current_time - time_until_build
            if time_left > 0:
                # not worth building if it wont be done in time to produce anything
                if ub + time_left*(time_left+1)/2 > best_score:
                    # if it can do better than the current best score, update the state
                    # to the conditions after the robot is built
                    temp = (
                        state[0] + time_until_build * state[4] - blueprint[0],
                        state[1] + time_until_build * state[5],
                        state[2] + time_until_build * state[6],
                        state[3] + time_until_build * state[7],
                        state[4] + 1,
                        state[5],
                        state[6],
                        state[7]
                    )
                    # and call the function again with this state
                    score = recursive(current_time + time_until_build, temp, best_score)
                    # track best outcome
                    best_score = max(score, best_score)
                    # indicate we have built a robot in this call
                    build_anything = True

        # build clay robot
        if state[5] < blueprint[3]:
            time_until_build = max(0, math.ceil((blueprint[1] - state[0])/state[4])) + 1
            time_left = max_time - current_time - time_until_build
            if time_left > 0:
                if ub + time_left*(time_left+1)/2 > best_score:
                    temp = (
                        state[0] + time_until_build * state[4] - blueprint[1],
                        state[1] + time_until_build * state[5],
                        state[2] + time_until_build * state[6],
                        state[3] + time_until_build * state[7],
                        state[4],
                        state[5] + 1,
                        state[6],
                        state[7]
                    )
                    score = recursive(current_time + time_until_build, tuple(temp), best_score)
                    best_score = max(score, best_score)
                    build_anything = True

        # build obsidian robot
        if state[5] > 0 and state[6] < blueprint[5]:
            time_until_build = math.ceil(max(
                    0,
                    (blueprint[2] - state[0])/state[4],
                    (blueprint[3] - state[1])/state[5]
                )) + 1
            time_left = max_time - current_time - time_until_build
            if time_left > 0:
                if ub + time_left*(time_left+1)/2 > best_score:
                    temp = (
                        state[0] + time_until_build * state[4] - blueprint[2],
                        state[1] + time_until_build * state[5] - blueprint[3],
                        state[2] + time_until_build * state[6],
                        state[3] + time_until_build * state[7],
                        state[4],
                        state[5],
                        state[6] + 1,
                        state[7]
                    )
                    score = recursive(current_time + time_until_build, tuple(temp), best_score)
                    best_score = max(score, best_score)
                    build_anything = True

        # build geode robot
        if state[4] > 0 and state[6] > 0:
            time_until_build = math.ceil(max(
                    0,
                    (blueprint[4] - state[0])/state[4],
                    (blueprint[5] - state[2])/state[6]
                )) + 1
            time_left = max_time - current_time - time_until_build
            if time_left > 0:
                if ub + time_left*(time_left+1)/2 > best_score:
                    temp = (
                        state[0] + time_until_build * state[4] - blueprint[4],
                        state[1] + time_until_build * state[5],
                        state[2] + time_until_build * state[6] - blueprint[5],
                        state[3] + time_until_build * state[7],
                        state[4],
                        state[5],
                        state[6],
                        state[7] + 1
                    )
                    score = recursive(current_time + time_until_build, temp, best_score)
                    best_score = max(score, best_score)
                    build_anything = True

        if not build_anything:
            # if we haven't built anything, it's because we will not build any more robots
            # so we can find the final score and finish running
            score = state[3] + (max_time - current_time) * state[7]
            best_score = max(score, best_score)

        return best_score
    return recursive


def part1(filename):
    input = get_input(filename)
    blueprints = parse(input)
    quality = 0
    for id, blueprint in enumerate(blueprints):
        score_func = find_geodes(blueprint)
        score = score_func()
        quality += (id + 1) * score
    return quality


def part2(filename):
    blueprints = parse(get_input(filename))
    scores = 1
    for idx in range(3):
        score_func = find_geodes(blueprints[idx], 32)
        scores *= score_func()
    return scores


if __name__ == "__main__":
    filename = "19.real.txt"
    t = time()
    p1 = part1(filename)
    t = time() - t
    print("Part 1:", p1)
    print(f"{t:.2}s")
    t = time()
    p2 = part2(filename)
    t = time() - t
    print("Part 2:", p2)
    print(f"{t:.2}s")
