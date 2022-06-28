import base
from ch3 import EXPECTED, NOT_EXPECTED


EXPECTED += [
    "Test 04_1 OK!",
    "Test 04_4 test OK!",
    "Test 04_5 ummap OK!",
    "Test 04_6 ummap2 OK!",
]

NOT_EXPECTED += [
    "Should cause error, Test 04_2 fail!",
    "Should cause error, Test 04_3 fail!",
]

if __name__ == "__main__":
    base.test(EXPECTED, NOT_EXPECTED)
