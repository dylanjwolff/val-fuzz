import pandas as pd
import json


filenames = ["baseuqual.txt", "skoluqual.txt", "baseequal.txt"]

cumdf = pd.DataFrame()
for fname in filenames:
    f = open(fname)
    lines = f.readlines()
    filtered = [l for l in lines if "JSONRUNSTATS" in l]
    stripped = [l.split("RUNSTATS:")[-1].strip() for l in filtered]
    jsons = [json.loads(l) for l in stripped]
    df = pd.DataFrame(jsons)
    df["name"] = fname
    cumdf = cumdf.append(df)

cumdf["sat-ratio"] = cumdf["Sat Iterations"]/cumdf["Iterations"]
cumdf["sat-ratio-sub"] = cumdf["Sat Substitutions"]/cumdf["Unique Substitutions"]
cumdf["unsat-ratio"] = cumdf["Unsat Iterations"]/cumdf["Iterations"]
cumdf["to-ratio"] = cumdf["All Non-Errors Timeout on Iterations"]/cumdf["Iterations"]

print(cumdf.groupby("name").mean().T)
print(cumdf.groupby("name").std().T)

m = cumdf.groupby("name").mean()
# iter_accounter_for = m["Unsat Iterations"] + m["Sat Iterations"] + m["All Non-Errors Timeout on Iterations"] + m["All Errors on Iterations"] 
# print(iter_accounter_for)
