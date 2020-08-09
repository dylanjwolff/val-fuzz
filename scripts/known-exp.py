#!/usr/bin/python
import subprocess as sp
import os
import sys

o = sp.getoutput("ls ~/known").split("\n")
solv_hashes = [l.split("-") for l in o]
print(solv_hashes)

reprod = set([])
for zsh in solv_hashes:
    cmdstr = "python3 ./smtvm.py " + zsh[0] + " install " + zsh[1]
    print(cmdstr)
    o = sp.getoutput(cmdstr)
    print(o)

    cmdstr = "rm -r 0-cswap-fuzz-run-out"
    print(cmdstr)
    o = sp.getstatusoutput(cmdstr)

    cmdstr = "cswap-cli -v -i 500 ~/known/" + zsh[0] + "-" + zsh[1]
    print(cmdstr)
    o = sp.getstatusoutput(cmdstr)
    print(o)

    cmdstr = "ls 0-cswap-fuzz-run-out/bugs"
    print(cmdstr)
    bug_files = sp.getoutput(cmdstr).split("\n")

    cmdstr = "ls ~/known/" + zsh[0] + "-" + zsh[1]
    print(cmdstr)
    seed_files = sp.getoutput(cmdstr).split("\n")
    seed_file_stems = [os.path.splitext(f)[0] for f in seed_files]

    for sfs in seed_file_stems:
        for bf in bug_files:
            if sfs in bf:
                reprod.add(sfs)

print("Found following bugs")
print(reprod)
