#!/usr/bin/python
import subprocess as sp
import os
import sys
import numpy as np
import pandas as pd
import matplotlib
from pandas.api.types import CategoricalDtype
from scipy.stats import ttest_ind_from_stats


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
# print(cumdf)


cumdf["BugEfficiencyAdj"] = cumdf["bugs_adj"] / (cumdf["Iterations"] + cumdf["Substitutions"])
cumdf["BugEfficiency"] = cumdf["bugs_found"] / (cumdf["Iterations"] + cumdf["Substitutions"])
cumdf["SoundnessBugEfficiencyAdj"] = cumdf["sounds_adj"] / (cumdf["Iterations"] + cumdf["Substitutions"])
cumdf["SoundnessBugEfficiency"] = cumdf["soundness_bugs_found"] / (cumdf["Iterations"] + cumdf["Substitutions"])

means = cumdf.groupby("config tag").mean()
# print(means.loc[: , ["sounds_adj", "soundness_bugs_found", "bugs_adj", "bugs_found", "SoundnessBugEfficiency", "BugEfficiencyAdj", "BugEfficiency"]])

stds = cumdf.groupby("config tag").std()
# print(stds.loc[: , ["sounds_adj", "soundness_bugs_found","bugs_found", "SoundnessBugEfficiency", "BugEfficiency"]])

final = means.join(stds, rsuffix="Std")[["soundness_bugs_found", "soundness_bugs_foundStd", "bugs_found", "bugs_foundStd", "SoundnessBugEfficiencyAdj", "SoundnessBugEfficiencyAdjStd", "BugEfficiencyAdj", "BugEfficiencyAdjStd"]]

col = "sounds_adj"
basem = means[col]["SKOLU"]
bases = stds[col]["SKOLU"]
tt = ttest_ind_from_stats(mean1=basem, std1=bases, nobs1=30, mean2=means[col], std2=stds[col], nobs2=30, equal_var=False)
final[col + "PVal"] = np.round(tt[1], 4)

col = "bugs_adj"
basem = means[col]["SKOLU"]
bases = stds[col]["SKOLU"]
tt = ttest_ind_from_stats(mean1=basem, std1=bases, nobs1=30, mean2=means[col], std2=stds[col], nobs2=30, equal_var=False)
final[col + "PVal"] = np.round(tt[1], 4)


col = "SoundnessBugEfficiencyAdj"
basem = means[col]["SKOLU"]
bases = stds[col]["SKOLU"]
tt = ttest_ind_from_stats(mean1=basem, std1=bases, nobs1=30, mean2=means[col], std2=stds[col], nobs2=30, equal_var=False)
final[col + "PVal"] = np.round(tt[1], 4)

col = "BugEfficiencyAdj"
basem = means[col]["SKOLU"]
bases = stds[col]["SKOLU"]
tt = ttest_ind_from_stats(mean1=basem, std1=bases, nobs1=30, mean2=means[col], std2=stds[col], nobs2=30, equal_var=False)
final[col + "PVal"] = np.round(tt[1], 4)



final.to_csv("aggereg_ceesssvee")

print(final)
cumdf.boxplot()
