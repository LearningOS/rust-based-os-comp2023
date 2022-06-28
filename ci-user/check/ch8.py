import base
from ch5 import NOT_EXPECTED

EXPECTED = [
    # ch2b
    "Hello, world from user mode program!",
    "Test power_3 OK!",
    "Test power_5 OK!",
    "Test power_7 OK!",
    # ch3b
    r"get_time OK! (\d+)",
    "Test sleep OK!",
    r"current time_msec = (\d+)",
    r"time_msec = (\d+) after sleeping (\d+) ticks, delta = (\d+)ms!",
    "Test sleep1 passed!",
    "Test write A OK!",
    "Test write B OK!",
    "Test write C OK!",
    # ch5b
    "forktest2 test passed!",
    # ch6b
    "file_test passed!",
    # ch7b
    "pipetest passed!",
    # ch8b
    "mpsc_sem passed!",
    "philosopher dining problem with mutex test passed!",
    "race adder using spin mutex test passed!",
    "sync_sem passed!",
    "test_condvar passed!",
    "threads with arg test passed!",
    "threads test passed!",
    # ch8
    "deadlock test mutex 1 OK!",
    "deadlock test semaphore 1 OK!",
    "deadlock test semaphore 2 OK!",
    "ch8 Usertests passed!",
]


if __name__ == "__main__":
    base.test(EXPECTED, NOT_EXPECTED)
