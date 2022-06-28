import base

EXPECTED = [
    "Hello, world!",
]

TEMP = []

NOT_EXPECTED = [
    "FAIL: T.T",
]

if __name__ == "__main__":
    base.test(EXPECTED + TEMP, NOT_EXPECTED)