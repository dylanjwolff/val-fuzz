#!/usr/bin/python
import subprocess as sp
import os
import sys

o = sp.getoutput("ls ~/known/repro").split("\n")
solv_hashes = [l.split("-") for l in o]
print(solv_hashes)
cum_runstats = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

reprod = {}
for zsh in solv_hashes:
    cmdstr = "python3 ./smtvm.py " + zsh[0] + " install " + zsh[1]
    print(cmdstr)
    o = sp.getoutput(cmdstr)
    print(o)

    cmdstr = "rm -r 0-cswap-fuzz-run-out"
    print(cmdstr)
    o = sp.getstatusoutput(cmdstr)

    cmdstr = "cswap-cli -v -i 500 ~/known/repro/" + zsh[0] + "-" + zsh[1]
    print(cmdstr)
    (s, o) = sp.getstatusoutput(cmdstr)
    runstats = [l for l in o.split("\n") if "CSVRUNSTATS:" in l][0].split(':')[-1]
    numstats = [int(n.strip()) for n in runstats.split(',')]
    cum_runstats = [x + y for (x, y) in zip(numstats, cum_runstats)]

    cmdstr = "ls 0-cswap-fuzz-run-out/bugs"
    print(cmdstr)
    bug_files = sp.getoutput(cmdstr).split("\n")

    cmdstr = "ls ~/known/repro/" + zsh[0] + "-" + zsh[1]
    print(cmdstr)
    seed_files = sp.getoutput(cmdstr).split("\n")
    seed_file_stems = [os.path.splitext(f)[0] for f in seed_files]

    for sfs in seed_file_stems:
        solver_sfs = str(zsh[0]) + "-" + sfs
        if solver_sfs not in reprod.keys():
            reprod[solver_sfs] = "NOREPRO"

        for bf in bug_files:
            cmdstr = "cat 0-cswap-fuzz-run-out/bugs/" + bf
            bf_cts = sp.getoutput(cmdstr).lower()

            if sfs in bf:
                if "soundness" in bf_cts:
                    reprod[solver_sfs] = "SOUND" 
                elif reprod[solver_sfs] != "SOUND":
                    reprod[solver_sfs] = "BUG" 



sortedks = sorted(reprod.keys())

print("csv runstats: " +str(cum_runstats))

print("csv header:")
for k in sortedks:
    print(str(k) + ", ", end = '')
print("\ncsv row:")
for k in sortedks:
    print(str(reprod[k]) + ", ", end = '')