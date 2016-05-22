##  B(n+1) += B(n) - H(n+1)
##  B(n+1) = 2B(n) - H(n+1)
##  B(n+2) = 4B(n) - 2H(n+1) - H(n+2)
##  B(n+3) = 8B(n) - 4H(n+1) - 2H(n+2) - H(n+3)
##
##  => B(n) >= 1/2H(n+1) + 1/4H(n+1) + 1/8(n+2) ...

from decimal import *

raw_input()
nums = raw_input().split(" ")

tot = 0

for n in nums:
    tot = tot*2 + int(n)

with localcontext() as ctx:
    ctx.prec = 35000
    res = Decimal(tot) / Decimal(2 ** len(nums))
    print res.to_integral_exact(rounding=ROUND_CEILING)
