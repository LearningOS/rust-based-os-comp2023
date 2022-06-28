import sys
import re
import ch5_1

def test(expected, not_expected=[]):
    output = sys.stdin.read(1000000)

    count = 0
    total = len(expected) + len(not_expected)

    for pattern in expected:
        if re.search(pattern, output):
            count += 1
            print(f'\033[92m[PASS]\033[0m found <{pattern}>')
        else:
            print(f'\033[91m[FAIL]\033[0m not found <{pattern}>')

    for pattern in not_expected:
        if not re.search(pattern, output):
            count += 1
            print(f'\033[92m[PASS]\033[0m not found <{pattern}>')
        else:
            print(f'\033[91m[FAIL]\033[0m found <{pattern}>')

    print('\nTest passed: %d/%d' % (count, total))
    assert count == total

    # test stride
    if re.search(ch5_1.PATTERN, output):
        ch5_1.stride_test(re.compile(ch5_1.PATTERN).findall(output))

# def test_str(expected):
#     output = sys.stdin.read(1000000)

#     count = 0
#     total = len(expected)

#     for pattern in expected:
#         if output.find(pattern) != -1:
#             count += 1
#             print('\033[92m[PASS]\033[0m', pattern)
#         else:
#             print('\033[91m[FAIL]\033[0m', pattern)

#     print('\nTest passed: %d/%d' % (count, total))
#     assert count == total
