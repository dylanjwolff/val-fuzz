#!/usr/bin/python
import subprocess as sp
import os
import sys

seeds_dir = "/home/wolffd/known/repro/z3-9847675"
z3_location = "/home/wolffd/git/constant-swap/scripts/.solvers/z3/build/z3"
opfuzz_location = "/home/wolffd/git/constant-swap/opfuzz/yinyang.py"
lcov_aggregate = "total.info"
lcov_temp = "temp.info"

src_location = os.path.split(os.path.split(z3_location)[0])[0]

o = sp.getoutput(f'find {src_location} -name "*.gcda" -type f -delete')
for root, dirs, filenames in os.walk(seeds_dir, topdown=False):
    for filename in filenames:
        current_file = os.path.join(root, filename)
        o = sp.getoutput(f'python3 {opfuzz_location} -i 1 -t 6 "{z3_location}" {current_file}')

        o = sp.getoutput(f'fastcov -l -o {lcov_temp} -d {src_location} -X -b')
        print(o)

        if os.path.exists(lcov_aggregate):
            sp.getoutput(f'fastcov -C {lcov_temp} {lcov_aggregate} -l -o {lcov_aggregate}')
        else:
            sp.getoutput(f'mv {lcov_temp} {lcov_aggregate}')
        sp.getoutput(f'find {src_location} -name "*.gcda" -type f -delete')

