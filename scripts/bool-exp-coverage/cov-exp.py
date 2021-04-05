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
flags["SKOLU"] = "--skolemize-universal"
flags["RBASE"] = "-r"
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
# flags["ADOMAINE"] = "--abstract-domain-sub-expressions"
#flags["MINCONSTS1"] = "--min-consts 1"
#flags["MINCONSTS3"] = "--min-consts 3"
#flags["MINCONSTS5"] = "--min-consts 5"
#flags["MAXCONSTS5"] = "--max-consts 5"
#flags["MAXCONSTS10"] = "--max-consts 10"

configs = flags
reps = range(0,15)
streps = ",".join([str(r) for r in reps])
iters = [5, 10, 15, 25, 35, 50, 100]
filesdir = "~/known/repro"
profile = 1

base_cvc4 = "0675545"
base_z3 = "a35d00e"
cmdstr = "python3 ./smtvm.py cvc4 install " + base_cvc4
o = sp.getoutput(cmdstr)
cmdstr = "python3 ./smtvm.py z3 install " + base_z3
o = sp.getoutput(cmdstr)
 

for config_tag, options in tqdm(configs.items()):


    cum_repros = []
    cumreps = []
    cum_runstats_dicts = []
    cumdf =  pd.DataFrame()
    reprod = {}
    reprods = {}

    for i in tqdm(iters):
        cmdstr = "cswap-cli -v -w 1,1,1 --timeout 1 " + options + " -p " + str(profile) + " -s " + streps + " -i " + str(i) + " --monitors-in-final " + filesdir
        (s, o) = sp.getstatusoutput(cmdstr)

        iter_runstats_strs = [l.split("JSONRUNSTATS:")[-1] for l in o.split("\n") if "JSONRUNSTATS:" in l]

        iter_runstats = [json.loads(rss) for rss in iter_runstats_strs]

        df = pd.DataFrame(iter_runstats)
        df["iters"] = i
        df["seed"] = df.index
        df["profile"] = profile
        covs = []
        for i in iter_runstats:
            maybcov = [v for (k, v) in i["Coverages"].items()]
            if len(maybcov) > 0:
                covs.append(maybcov[0])
            else:
                covs.append(0)
        df["coverage"] = covs


        if cumdf.empty:
            cumdf = df
        else:
            cumdf = cumdf.append(df, ignore_index=True)

        cfg_strs = [l.split("JSONCONFIG:")[-1] for l in o.split("\n") if "JSONCONFIG:" in l]
        cfgs = [json.loads(cfs) for cfs in cfg_strs]
        for cfg in cfgs:
            del cfg["file_provider"]
            cfg["profiles"] = len(cfg["profiles"])

        print(cfgs)

        for rep in reps:
            cmdstr = "rm -r *_" + str(rep) + "-cswap-fuzz-run-out"
            sp.getoutput(cmdstr)

    cumdf["Cov Per Sat"] = cumdf["coverage"]/cumdf["Sat Substitutions"]
    print(config_tag)
    print(cumdf.groupby("iters").mean().T)
    print(cumdf.groupby("iters").std().T)

    cumdf.to_csv(config_tag + "_COV.ceesv")


