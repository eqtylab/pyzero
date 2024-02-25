import sys

n = int(sys.argv[0])

print(f'n = {n}')

a, b = 0, 1
for _ in range(n):
    a, b = b, a + b

print(f'nth fib = {a}')
