import pandas as pd
import json


fname = "noeffinal-filtered.txt"

with open(fname) as f:
    lines = f.readlines()
    filtered = [l for l in lines if "SEED" in l]

    tupled = []
    for i, l in enumerate(filtered):
        seed = int(l.split(':')[-1].strip())
        tupled.append((("SUCCESS" in l) ,("FAILURE" in l), seed))

    df = pd.DataFrame(tupled, columns=["success","failure", "seed"])

    total = df.groupby("seed").sum()
    total = total["success"] / total["failure"]
    print(total.mean())
    print(total.std())

