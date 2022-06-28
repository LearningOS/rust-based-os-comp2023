import base

EXPECTED = [
    "Hello, world from user mode program!",
    "Test power_3 OK!",
    "Test power_5 OK!",
    "Test power_7 OK!",
]

TEMP = []

NOT_EXPECTED = [
    "FAIL: T.T",
]

if __name__ == "__main__":
    base.test(EXPECTED + TEMP, NOT_EXPECTED)
