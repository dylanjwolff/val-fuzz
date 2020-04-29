#!/usr/bin/python3.7
# -*- coding: utf-8 -*-
import subprocess as sp
import os
import sys
d=".".join(sys.argv[1].split(".")[0:-1])
sp.getoutput("mkdir "+d)
os.system("chdir `pwd`")
sp.getoutput("cp reduce.py "+d)
out=sp.getoutput("cd "+d+ ";wget https://people.inf.ethz.ch/suz/bugstodo/"+sys.argv[1])
os.system("mv "+d+"/"+sys.argv[1] + " "+d+"/bug.smt2")
os.system("chmod +x "+d+"/reduce.py")
os.system("screen -S " +sys.argv[1][:50])
