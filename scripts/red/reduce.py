#! /usr/bin/python3.7
import subprocess
formula="bug.smt2"
s1="/home/windomin/comp/bin/cvc4-fa2ba76 --quiet"
s2="/home/windomin/comp/bin/z3-e075f38"
out1=subprocess.getoutput("echo C|"+s1+" "+formula)
# out2=subprocess.getoutput("echo C|"+s2+" "+formula)
print(out1)
print()
# print(out2)
# if "sat\nsat\nsat\nsat\nsat" in out1 and "sat\nunsat\nsat\nsat\nsat" in out2:exit(0)
# if "sequences_rewriter.cpp:1199" in out1:exit(0)
# exit(1)

# if "error" in out1 or "error" in out2: exit(1)
# if "Fatal failure" in out2: exit(0)

print("\n \n")
print("% "+s1.split("/")[-1]+" bug.smt2")
print(out1)
# print("% "+s2.split("/")[-1]+" bug.smt2")
# print(out2)
print("% cat bug.smt2")
with open("bug.smt2") as bug:print(bug.read())
