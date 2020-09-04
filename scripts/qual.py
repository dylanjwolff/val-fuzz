import pandas as pd
import json


filenames = ["baseuqual.txt", "skoluqual.txt"]

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
print(cumdf.groupby("name").mean().T)
print(cumdf.groupby("name").std().T)

