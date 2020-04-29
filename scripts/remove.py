#!/usr/bin/python3.7
import os
from pathlib import Path
bins=list(reversed(sorted(Path("bin").iterdir(), key=os.path.getmtime)))
for b in bins[20:]:
    print("rm -rf "+str(b))
    os.system("rm -rf "+str(b))
