#!/usr/bin/python
import subprocess as sp
import os
import sys
import pandas as pd
import json
from pandas.api.types import CategoricalDtype
from tqdm import tqdm

base_cvc4 = "0675545"
base_z3 = "a35d00e"

o = sp.getoutput("ls ~/known/repro").split("\n")
solv_hashes = [l.split("-") for l in o]

for zsh in tqdm(solv_hashes):
    cmdstr = "python3 ./smtvm.py " + zsh[0] + " install " + zsh[1]
    o = sp.getoutput(cmdstr)

    if zsh[0].strip() == "z3":
        cmdstr = "python3 ./smtvm.py cvc4 install " + base_cvc4
        o = sp.getoutput(cmdstr)
    elif zsh[0].strip() == "cvc4":
        cmdstr = "python3 ./smtvm.py z3 install " + base_z3
        o = sp.getoutput(cmdstr)
    else:
        sys.exit("didn't recognize solver version")
    cmdstr = "ls -l ~/bin"
    o = sp.getoutput(cmdstr)
    print(o)


    cmdstr = "ls ~/known/repro/" + zsh[0] + "-" + zsh[1] + " | parallel solver-cli -v >> solver90.txt"
    (s, o) = sp.getstatusoutput(cmdstr)
