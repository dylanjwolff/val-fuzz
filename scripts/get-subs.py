import json

with open ("all3.log", "r") as myfile:
    data=myfile.readlines()
    run_stats = [json.loads(run) for run in data]
    unique_subs = [run["Unique Substitutions"] for run in run_stats]
    print(unique_subs)
    print(6/sum(unique_subs))
