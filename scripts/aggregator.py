#!/usr/bin/python
import subprocess as sp
import os
import sys
import pandas as pd
from pandas.api.types import CategoricalDtype


def bug_filter():
    rbs = pd.read_csv("RBASE_SINGLE.csv")
    all_bugs = (rbs == "BUG").all()
    return list(all_bugs[all_bugs == True].index)

def sound_filter():
    rbs = pd.read_csv("RBASE_SINGLE.csv")
    all_bugs = (rbs == "SOUND").all()
    return list(all_bugs[all_bugs == True].index)


cmdstr = "ls *.csv"
o = sp.getoutput(cmdstr)

cumdf = pd.DataFrame()
for f in o.split('\n'):
    df = pd.read_csv(f.strip())
    df["config tag"] = f.split('.')[0]
    cumdf = cumdf.append(df)

cumrepros = cumdf.iloc[:, 15:-1]
reprocats = CategoricalDtype(categories=["BUG", "SOUND", "NOREPRO"])
cumrepros = cumrepros.astype(reprocats)

crt = pd.DataFrame(cumrepros.values.T)
crt.index = cumrepros.columns
crt = crt.astype(reprocats)
# bugs = [row.value_counts()["BUG"] + row.value_counts()["SOUND"] for (seed, row) in reprodf.transpose().items()]
bugs = [row.value_counts()["BUG"] + row.value_counts()["SOUND"] for (seed, row) in crt.drop(bug_filter()).items()]
sounds = [row.value_counts()["SOUND"] for (seed, row) in crt.drop(sound_filter()).items()]

cumdf["bugs_adj"] = bugs
cumdf["sounds_adj"] = sounds
print(cumdf)


cumdf["Bug Efficiency Adj"] = cumdf["bugs_adj"] / (cumdf["Iterations"] + cumdf["Substitutions"])
cumdf["Bug Efficiency"] = cumdf["bugs_found"] / (cumdf["Iterations"] + cumdf["Substitutions"])
cumdf["Soundness Bug Efficiency Adj"] = cumdf["sounds_adj"] / (cumdf["Iterations"] + cumdf["Substitutions"])
cumdf["Soundness Bug Efficiency"] = cumdf["soundness_bugs_found"] / (cumdf["Iterations"] + cumdf["Substitutions"])
cumdf["Bug Time Efficiency"] = cumdf["bugs_adj"] / cumdf["Elapsed Time (s)"]
cumdf["Soundness Bug Time Efficiency"] = cumdf["sounds_adj"] / cumdf["Elapsed Time (s)"]

means = cumdf.groupby("config tag").mean()
print(means.loc[: , ["sounds_adj", "soundness_bugs_found", "bugs_adj", "bugs_found", "Soundness Bug Efficiency", "Bug Efficiency Adj", "Bug Efficiency", "Soundness Bug Time Efficiency", "Bug Time Efficiency"]])

stds = cumdf.groupby("config tag").std()
print(stds.loc[: , ["soundness_bugs_found","bugs_found", "Soundness Bug Efficiency", "Bug Efficiency", "Soundness Bug Time Efficiency", "Bug Time Efficiency"]])
