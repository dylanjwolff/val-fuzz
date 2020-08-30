#!/usr/bin/python
import subprocess as sp
import os
import sys
import pandas as pd
import json
from pandas.api.types import CategoricalDtype
from tqdm import tqdm

config_tag = "BASE"
repetitions = 30

base_cvc4 = "0675545"
base_z3 = "a35d00e"

o = sp.getoutput("ls ~/known/repro").split("\n")
solv_hashes = [l.split("-") for l in o]

cum_repros = []
cumreps = []
for rep in tqdm(range(repetitions)):
    cum_runstats_dicts = []
    total_elapsed = 0
    cumdf =  pd.DataFrame()
    reprod = {}
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


        cmdstr = "cswap-cli -v -s " + str(rep) + " -i 5 ~/known/repro/" + zsh[0] + "-" + zsh[1]
        (s, o) = sp.getstatusoutput(cmdstr)
        elapsed = int([l for l in o.split("\n") if "Elapsed Time" in l][0].split(':')[-1].strip())
        total_elapsed = elapsed + total_elapsed

        runstats_str = [l for l in o.split("\n") if "JSONRUNSTATS:" in l][0].split("JSONRUNSTATS:")[-1]
        runstats = json.loads(runstats_str)
        cum_runstats_dicts.append(runstats)

        cfg_str = [l for l in o.split("\n") if "JSONCONFIG:" in l][0].split("JSONCONFIG:")[-1]
        cfg = json.loads(cfg_str)
        del cfg["file_provider"]
        cfg["profiles"] = len(cfg["profiles"])
        dfcfg = pd.DataFrame.from_dict(cfg, orient="index")

        cmdstr = "ls 0-cswap-fuzz-run-out/bugs"
        bug_files = sp.getoutput(cmdstr).split("\n")

        cmdstr = "ls ~/known/repro/" + zsh[0] + "-" + zsh[1]
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
        cmdstr = "rm -r " + str(rep) + "-cswap-fuzz-run-out"
        o = sp.getstatusoutput(cmdstr)





    reprocats = CategoricalDtype(categories=["BUG", "SOUND", "NOREPRO"])
    reprodf = pd.DataFrame.from_dict(reprod, orient="index", dtype=reprocats)
    cum_repros.append(reprodf[0])

    cumdf = pd.DataFrame(cum_runstats_dicts)

    iter_df = cumdf.sum()
    iter_df["Bugs"] = reprodf[0].value_counts()["BUG"] + reprodf[0].value_counts()["SOUND"]
    iter_df["Soundness Bugs"] = reprodf[0].value_counts()["SOUND"]

    cumreps.append(iter_df)

cumreps_repros_df = pd.DataFrame(cum_repros).reset_index(drop=True)
cumrepsdf = pd.DataFrame(cumreps)

cumrepsdf.to_csv(config_tag + ".csv")
res = pd.DataFrame()
res["Means"] = cumrepsdf.mean()
res["Std"] = cumrepsdf.std()
print(res)
# print(cumrepsdf.std())

