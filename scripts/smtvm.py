#!/usr/bin/python
import subprocess as sp
import os
import sys

def tip(solver):
    sp.getoutput("cd .solvers/"+solver+"/;git pull;")
    return sp.getoutput("cd .solvers/"+solver + ";git log|head -n 1").split(" ")[1][0:7]

def in_bin(commit):
    for b in os.listdir("bin"):
        if commit in b: return True
    return False

def z3(commit):
    def build(commit):
        os.system("cd .solvers/z3; git checkout master; git pull;")
        if os.system("cd .solvers/z3; git checkout " + commit) != 0: raise "not a commit"
        os.system("cd .solvers/z3; rm -rf .solvers/z3/build")
        os.system("cd .solvers/z3; python scripts/mk_make.py; ./configure -d; cd build; make -j 4")
        os.system("mv .solvers/z3/build/z3 "+"bin/z3-"+commit+";chmod +x" +" bin/z3-"+commit)
    if (not in_bin(commit)):
        build(commit)
    else:
        print("Using cached Z3")

def cvc4(commit):
    def build(commit):
        os.system("cd .solvers/cvc4; git checkout master; git pull;")
        if os.system("cd .solvers/cvc4; git checkout " + commit) != 0: raise "not a commit"
        os.system("cd .solvers/cvc4; rm -rf .solvers/cvc4/build")
        os.system("cd .solvers/cvc4; ./contrib/get-antlr-3.4; ./configure.sh --assertions; cd build; make -j 4")
        os.system("mv .solvers/cvc4/build/bin/cvc4"+" bin/cvc4-"+commit+ ";chmod +x" +" bin/cvc4-"+commit)
    if (not in_bin(commit)):
        build(commit)
    else:
        print("Using cached CVC4")

try:
    if sys.argv[1] == "z3":
        if sys.argv[2] == "install":
            if sys.argv[3] == "tip":
                commit = tip("z3")
            else:
                commit = sys.argv[3]
            z3(commit)
            os.system("cp bin/z3-" + commit + " /usr/local/bin/z3")
        elif sys.argv[2] == "tip":
            z3(tip("z3"))
        else:
            z3(sys.argv[2])

    elif sys.argv[1] == "cvc4":
        if sys.argv[2] == "install":
            if sys.argv[3] == "tip":
                commit = tip("cvc4")
            else:
                commit = sys.argv[3]
            cvc4(commit)
            os.system("cp bin/cvc4-" + commit + " /usr/local/bin/cvc4")
        elif sys.argv[2] == "tip":
            cvc4(tip("cvc4"))
        else:
            cvc4(sys.argv[2])

    elif sys.argv[1] == "tip":
        z3(tip("z3"))
        cvc4(tip("cvc4"))
    elif sys.argv[1] == "install":
        z3(tip("z3"))
        cvc4(tip("cvc4"))
        os.system("cp bin/z3-" + commit + " /usr/local/bin/z3")
        os.system("cp bin/cvc4-" + commit + " /usr/local/bin/cvc4")
except:
    sys.exit(1)
