import base
from ch5 import EXPECTED, NOT_EXPECTED

EXPECTED += [
    "Test file0 OK!",
    "Test fstat OK!",
    "Test link OK!",
    "Test mass open/unlink OK!"
]

EXPECTED = list(set(EXPECTED) - set([
    "Test set_priority OK!"
]))

TEMP = [
    # "ch6 Usertests passed!",
]

if __name__ == '__main__':
    base.test(EXPECTED + TEMP, NOT_EXPECTED)
