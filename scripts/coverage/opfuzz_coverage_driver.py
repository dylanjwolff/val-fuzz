#!/usr/bin/python
import subprocess as sp
import os
import sys

iterations = 20
seeds_dir = "/home/wolffd/coverage-experiment/coverage-dataset"
z3_location = "/home/wolffd/scratch/z3/build/z3"
opfuzz_location = "/home/wolffd/scratch/opfuzz/yinyang.py"
lcov_aggregate = "total.info"
lcov_temp = "temp.info"

src_location = os.path.split(os.path.split(z3_location)[0])[0]
print(src_location)

o = sp.getoutput(f'find {src_location} -name "*.gcda" -type f -delete')
for root, dirs, filenames in os.walk(seeds_dir, topdown=False):
    for filename in filenames:
        for iteration in range(iterations):
            current_file = os.path.join(root, filename)
            o = sp.getoutput(f'python3 {opfuzz_location} -i 1 -t 6 "{z3_location}" {current_file}')
            print(o)

            o = sp.getoutput(f'fastcov -l -o {lcov_temp} -d {src_location} -X -b')
            print(o)

            if os.path.exists(lcov_aggregate):
                sp.getoutput(f'fastcov -C {lcov_temp} {lcov_aggregate} -l -o {lcov_aggregate}')
            else:
                sp.getoutput(f'mv {lcov_temp} {lcov_aggregate}')
            sp.getoutput(f'find {src_location} -name "*.gcda" -type f -delete')

