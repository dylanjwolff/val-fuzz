#!/usr/bin/python
import subprocess as sp
import os
import sys
import numpy as np
import pandas as pd
import matplotlib
from pandas.api.types import CategoricalDtype
from scipy.stats import ttest_ind_from_stats
import matplotlib.pyplot as plt

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


cumdf["BugEfficiencyAdj"] = 1000 * cumdf["bugs_adj"] / (cumdf["Iterations"] + cumdf["Substitutions"])
cumdf["BugEfficiency"] = 1000 * cumdf["bugs_found"] / (cumdf["Iterations"] + cumdf["Substitutions"])
cumdf["SoundnessBugEfficiencyAdj"] = 1000 * cumdf["sounds_adj"] / (cumdf["Iterations"] + cumdf["Substitutions"])
cumdf["SoundnessBugEfficiency"] = 1000 * cumdf["soundness_bugs_found"] / (cumdf["Iterations"] + cumdf["Substitutions"])

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



relcdf= cumdf[cumdf["config tag"].map(lambda i: "RELC" in i)]
relcdf["xnum"] = relcdf["config tag"].map(lambda s: int(s.split("RELC")[-1]))
relmeans = relcdf.groupby("xnum").mean()
relstds = relcdf.groupby("xnum").std()

fig=plt.figure()
ax=fig.add_subplot(111, label="1")
ax2=fig.add_subplot(111, label="2", frame_on=False)

y1 = relmeans["soundness_bugs_found"]
y1e = relstds["soundness_bugs_found"]
y2 = relmeans["SoundnessBugEfficiencyAdj"]
y2e = relstds["SoundnessBugEfficiencyAdj"]
ax.plot(relmeans.index, y1, color="C0",ls='--', marker='o')
ax2.plot(relmeans.index, y2, color="C1",ls='--', marker='*')
ax.fill_between(relmeans.index, y1 - y1e, y1 + y1e, alpha=0.2)
ax2.fill_between(relmeans.index, y2 - y2e, y2 + y2e, alpha=0.2, color="C1")
ax2.yaxis.tick_right()

ax.set_ylabel("Soundness Bugs Found", color="C0")
ax2.set_ylabel("Soundness Bugs Found Per 1000 Solver Invocations", color="C1")
ax2.yaxis.set_label_position('right') 
ax.tick_params(axis='y', colors="C0")
ax2.tick_params(axis='y', colors="C1")
ax2.set_xticks(range(0,16))
ax.set_xticks(range(0,16))
plt.xticks(range(0,16))
plt.xlabel("Values used for Relative Constant Relations")
plt.show()

# ----------

multicdf= cumdf[cumdf["config tag"].map(lambda i: "MULTIEF" in i)]
multicdf["xnum"] = multicdf["config tag"].map(lambda s: int(s.split("MULTIEF")[-1]))
multimeans = multicdf.groupby("xnum").mean()
multistds = multicdf.groupby("xnum").std()

print("multimeans")
print(multimeans)
fig=plt.figure()
ax=fig.add_subplot(111, label="1")
ax2=fig.add_subplot(111, label="2", frame_on=False)

y1 = multimeans["soundness_bugs_found"]
y1e = multistds["soundness_bugs_found"]
y2 = multimeans["SoundnessBugEfficiencyAdj"]
y2e = multistds["SoundnessBugEfficiencyAdj"]
ax.plot(multimeans.index, y1, color="C0",ls='--', marker='o')
ax2.plot(multimeans.index, y2, color="C1",ls='--', marker='*')
ax.fill_between(multimeans.index, y1 - y1e, y1 + y1e, alpha=0.2)
ax2.fill_between(multimeans.index, y2 - y2e, y2 + y2e, alpha=0.2, color="C1")
ax2.yaxis.tick_right()

ax.set_ylabel("Soundness Bugs Found", color="C0")
ax2.set_ylabel("Soundness Bugs Found Per 1000 Solver Invocations", color="C1")
ax2.yaxis.set_label_position('right') 
ax.tick_params(axis='y', colors="C0")
ax2.tick_params(axis='y', colors="C1")
ax2.set_xticks(range(0,9))
ax.set_xticks(range(0,9))
plt.xticks(range(0,9))
plt.xlabel("Number of Constraints Enforced")
plt.show()




plotdf = cumdf[cumdf["config tag"].map(lambda i: i in ["RBASE", "SOUND", "SKOLU", "LINEAR"])]

plotdf.boxplot(by="config tag", column=["SoundnessBugEfficiencyAdj"], grid=False)
plt.ylabel("Bugs per Call To Solver")
plt.title("")
plt.suptitle("")
# plt.show()
plt.savefig('plt1.png', bbox_inches='tight')

plotdf.boxplot(by="config tag", column=["soundness_bugs_found"], grid=False)
plt.ylabel("Bugs")
plt.title("")
plt.suptitle("")
# plt.show()
plt.savefig('plt2.png', bbox_inches='tight')



