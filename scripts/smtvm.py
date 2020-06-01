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
        if os.system("cd .solvers/z3; git checkout " + commit) != 0: raise "not a commit"
        os.system("cd .solvers/z3; rm -rf .solvers/z3/build")
        os.system("cd .solvers/z3; ./configure; cd build; make -j 4")
        os.system("mv .solvers/z3/build/z3 "+"bin/z3-"+commit+";chmod +x" +" bin/z3-"+commit)
    if (not in_bin(commit)):
        print("Building Z3")
        build(commit)
    else:
        print("Using cached Z3")

def cvc4(commit):
    def build(commit):
        if os.system("cd .solvers/cvc4; git checkout " + commit) != 0: raise "not a commit"
        os.system("cd .solvers/cvc4; rm -rf .solvers/cvc4/build")
        os.system("cd .solvers/cvc4; ./contrib/get-antlr-3.4; ./configure.sh; cd build; make -j 4")
        os.system("mv .solvers/cvc4/build/bin/cvc4"+" bin/cvc4-"+commit+ ";chmod +x" +" bin/cvc4-"+commit)
    if (not in_bin(commit)):
        print("Building CVC4")
        build(commit)
    else:
        print("Using cached CVC4")

try:
    os.system("cd .solvers/z3; git checkout master; git pull;")
    os.system("cd .solvers/cvc4; git checkout master; git pull;")
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
        print("Getting tips")
        z3(tip("z3"))
        cvc4(tip("cvc4"))
    elif sys.argv[1] == "install":
        z3(tip("z3"))
        cvc4(tip("cvc4"))
        commit = tip("z3")
        os.system("rm ~/bin/z3")
        os.system("ln -s " + os.getcwd() + "/bin/z3-" + commit + " ~/bin/z3")
        commit = tip("cvc4")
        os.system("rm ~/bin/cvc4")
        os.system("ln -s " + os.getcwd() + "/bin/cvc4-" + commit + " ~/bin/cvc4")
except Exception as e:
    print(e)
    sys.exit(1)
