def fib_mod(A, B, n):
    a = A
    b = B
    for i in range(n-2):
        a, b = b, b*b + a
    return b

params = [int(x) for x in raw_input().split(" ")]
print fib_mod(*params)
