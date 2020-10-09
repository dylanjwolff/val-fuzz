#!/usr/bin/python
import subprocess as sp
import os
import sys
import pandas as pd
import json
from pandas.api.types import CategoricalDtype
from tqdm import tqdm

flags = {}
# flags["SPLITSKOL"] = ""
# flags["REVSPLITSKOL"] = "--skolemize-universal --no-skolemize-existential"
# flags["NOSKOLE"] = "--no-skolemize-existential"
# flags["SKOLU"] = "--skolemize-universal"
# flags["RBASE_SINGLE"] = "-r"
# flags["MULTIEF5"] = "--multi-enforce 5"
# flags["EFFINAL"] = "--enforce-final"
# flags["CPOG"] = "--copy-original"
# flags["RELC15"] = "--const-relations 15"
# flags["ADOMAIN"] = "--abstract-domain-vars"
# flags["LEAFOPT"] = "--leaf-opt"
# flags["MULTIEF4"] = "--multi-enforce 4"
# flags["MULTIEF6"] = "--multi-enforce 6"
# flags["MULTIEF7"] = "--multi-enforce 7"
# flags["MULTIEF8"] = "--multi-enforce 8"
# flags["RELC10"] = "--const-relations 10"
# flags["RELC5"] = "--const-relations 5"
# flags["RELC2"] = "--const-relations 2"
# flags["LINEAR"] = "--copy-original --abstract-domain-vars"
# flags["LEAFLINEAR"] = "--leaf-opt --copy-original --abstract-domain-vars"
# flags["RELC3"] = "--const-relations 3"
# flags["RELC4"] = "--const-relations 4"
# flags["EXP"] = "--const-relations 3 --multi-enforce 7"
# flags["LEAFEXP"] = "--leaf-opt --const-relations 3 --multi-enforce 7"
# flags["SOUND"] = "--abstract-domain-vars --const-relations 3 --multi-enforce 7"
# flags["LEAFSOUND"] = "--leaf-opt --abstract-domain-vars --const-relations 3 --multi-enforce 7"
#flags["UQUALOG"] = "--uqual-og-vars"
flags["ADOMAINE"] = "--abstract-domain-sub-expressions"
#flags["MINCONSTS1"] = "--min-consts 1"
#flags["MINCONSTS3"] = "--min-consts 3"
#flags["MINCONSTS5"] = "--min-consts 5"
#flags["MAXCONSTS5"] = "--max-consts 5"
#flags["MAXCONSTS10"] = "--max-consts 10"

configs = flags
reps = range(0,30)
streps = ",".join([str(r) for r in reps])
maxiter = 50

for config_tag, options in tqdm(configs.items()):


    base_cvc4 = "0675545"
    base_z3 = "a35d00e"

    o = sp.getoutput("ls ~/known/repro").split("\n")
    solv_hashes = [l.split("-") for l in o]

    cum_repros = []
    cumreps = []
    cum_runstats_dicts = []
    cumdf =  pd.DataFrame()
    reprod = {}
    reprods = {}

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


        cmdstr = "cswap-cli -v -w 1,1,1 --skolemize-universal " + options + " -s " + streps + " -i " + str(maxiter) + " ~/known/repro/" + zsh[0] + "-" + zsh[1]
        (s, o) = sp.getstatusoutput(cmdstr)

        iter_runstats_strs = [l.split("JSONRUNSTATS:")[-1] for l in o.split("\n") if "JSONRUNSTATS:" in l]

        iter_runstats = [json.loads(rss) for rss in iter_runstats_strs]
        df = pd.DataFrame(iter_runstats)
        df["subset_of_samples"] = str(zsh[0]) + str(zsh[1])
        df["seed"] = df.index
        if cumdf.empty:
            cumdf = df
        else:
            cumdf = cumdf.append(df, ignore_index=True)

        cfg_strs = [l.split("JSONCONFIG:")[-1] for l in o.split("\n") if "JSONCONFIG:" in l]
        cfgs = [json.loads(cfs) for cfs in cfg_strs]
        for cfg in cfgs:
            del cfg["file_provider"]
            cfg["profiles"] = len(cfg["profiles"])

        for rep in reps:
            if not rep in reprods.keys():
                reprods[rep] = {}
            cmdstr = "ls " + str(rep) + "-cswap-fuzz-run-out/bugs"
            bug_files = sp.getoutput(cmdstr).split("\n")

            cmdstr = "ls ~/known/repro/" + zsh[0] + "-" + zsh[1]
            seed_files = sp.getoutput(cmdstr).split("\n")
            seed_file_stems = [os.path.splitext(f)[0] for f in seed_files]

            for sfs in seed_file_stems:
                solver_sfs = str(zsh[0]) + "-" + sfs
                if solver_sfs not in reprod.keys():
                    reprods[rep][solver_sfs] = "NOREPRO"

                for bf in bug_files:
                    cmdstr = "cat " + str(rep) + "-cswap-fuzz-run-out/bugs/" + bf
                    bf_cts = sp.getoutput(cmdstr).lower()

                    if sfs in bf:
                        if "soundness" in bf_cts:
                            reprods[rep][solver_sfs] = "SOUND" 
                        elif reprods[rep][solver_sfs] != "SOUND":
                            reprods[rep][solver_sfs] = "BUG" 
            cmdstr = "rm -r *" + str(rep) + "-cswap-fuzz-run-out"
            sp.getoutput(cmdstr)

    reprocats = CategoricalDtype(categories=["BUG", "SOUND", "NOREPRO"])
    reprodf = pd.DataFrame.from_dict(reprods, orient="index", dtype=reprocats)
    reprodf.index.names = ["seed"]

    per_seed_stats = cumdf.groupby("seed").sum()

    bugs = [row.value_counts()["BUG"] + row.value_counts()["SOUND"] for (seed, row) in reprodf.transpose().items()]
    sounds = [row.value_counts()["SOUND"] for (seed, row) in reprodf.transpose().items()]
    per_seed_stats["bugs_found"] = bugs
    per_seed_stats["soundness_bugs_found"] = sounds 
    per_seed_stats = per_seed_stats.join(reprodf)
    per_seed_stats.to_csv(config_tag + ".csv")

    res = pd.DataFrame()
    res["Means"] = per_seed_stats.mean()
    res["Std"] = per_seed_stats.std()
    print(config_tag)
    print(res)

