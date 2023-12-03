
import math


# Returns the sum of the range [0, n].
def sum_to(n: int) -> int:
    return (n * (n + 1)) // 2


# Returns the smallest integer n such that sum_to(n) >= sum.
def inverse_sum_to(sum: int) -> int:
    f: float = math.sqrt(0.25 + 2*sum) - 0.5
    return math.ceil(f)


def compute_y(steps: int, y0: int, dy: int):
    pass


def main():
    # target area: x=192..251, y=-89..-59

    # What's the smallest x velocity that will get you to the window?
    dx_min = inverse_sum_to(192)
    # The largest x velocity is hitting the far side of the window in one step.
    dx_max = 251

    print(dx_min, dx_max)


main()
