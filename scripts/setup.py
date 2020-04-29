#! /usr/bin/python3.7
import os
os.system("mkdir -p bin .solvers")
os.system("cd .solvers/;git clone https://github.com/Z3Prover/z3.git z3")
os.system("cd .solvers/;git clone https://github.com/CVC4/CVC4.git cvc4")
