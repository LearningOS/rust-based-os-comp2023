import base
from ch4 import EXPECTED, NOT_EXPECTED

EXPECTED += [
    r"Test getpid OK! pid = (\d+)",
    "Test spawn0 OK!",
    "Test wait OK!",
    "Test waitpid OK!",
    "Test set_priority OK!",
]

EXPECTED = list(set(EXPECTED) - set([
    "string from task info test",
    "Test task info OK!",
]))

TEMP = [
    # "ch5 Usertests passed!",
]

if __name__ == '__main__':
    base.test(EXPECTED + TEMP, NOT_EXPECTED)
