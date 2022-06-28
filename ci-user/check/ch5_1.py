PATTERN = r"ratio = (\d+)"

def stride_test(result):
    assert len(result) == 6
    factors = [int(i) for i in result]
    print('\nstride ratio =', factors)

    if max(factors) / min(factors) < 1.5:
        print('\033[92m[PASS]\033[0m Stride Test')
        print('\nTest passed: 1/1')
    else:
        print('\033[91m[FAIL]\033[0m Stride Test')
        print('\nTest passed: 0/1')

    assert max(factors) / min(factors) < 1.5