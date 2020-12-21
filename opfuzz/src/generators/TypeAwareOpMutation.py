import random

from src.generators.Generator import Generator
from src.parsing.parse import *

import subprocess as sp
import os


class TypeAwareOpMutation(Generator):
    def __init__(self, seeds, args):
        assert(len(seeds) == 1)
        self.args = args
        self.formula = parse_file(seeds[0])
        self.bidirectional = []
        self.unidirectional = []

        self.parse_config_file()

    def parse_config_file(self):
        with open(self.args.opconfig) as f:
            lines = f.readlines()
        for l in lines:
            if ";" in l: continue
            if not l.strip(): continue
            arity = -1
            if ":arity" in l:
                arity = l.split(":arity")[-1].split(" ")[-1].strip()
                l =  " ".join(l.split(" ")[:-2])
            if "->" in l:
                op_from,op_to = l.split("->")[0].strip(), l.split("->")[1].strip()
                self.unidirectional.append((arity,op_from,op_to))
                continue

            op_class = [op.strip() for op in l.split(",")]
            self.bidirectional.append((arity, op_class))

    def arities_mismatch(self, arity, op_occ):
        if arity == "2+" and len(op_occ.subterms) < 2:
           return True

        if arity == "1-" and len(op_occ.subterms) > 2:
            return True
        return False

    def get_replacee(self,op_occ):
        for b in self.bidirectional:
            arity, op_class = b[0], b[1]
            if self.arities_mismatch(arity,op_occ):
                continue

            if op_occ.op in op_class:
                diff = op_class.copy()
                diff.remove(op_occ.op)
                replacee = random.choice(diff)
                return replacee

            if op_occ.quantifier in op_class:
                diff = op_class.copy()
                diff.remove(op_occ.quantifier)
                replacee = random.choice(diff)
                return replacee

        for u in self.unidirectional:
            arity, op, replacee = u[0], u[1], u[2]
            if op_occ.op != op or op_occ.quantifier != op: continue
            if self.arities_mismatch(arity, op_occ):
                continue
            return replacee
        return None

    def generate(self):
        for _ in range(self.args.modulo):
            max_choices = len(self.formula.op_occs)
            for _ in range(max_choices):
                op_occ = random.choice(self.formula.op_occs)
                replacee = self.get_replacee(op_occ)
                if replacee:
                    print(op_occ.op,"->",replacee)
                    op_occ.op = replacee
                    break
        mutated_fn = "%s/%s.smt2" % (self.args.scratchfolder, self.args.name)
        with open(mutated_fn,"w") as f: f.write(self.formula.__str__())


        # Call ValFuzz on the operator mutated file
        f = mutated_fn
        od = "%s/%s" % (self.args.scratchfolder, "valfuzz-od")

        out = sp.getoutput("cswap-cli -i 1 --skip-solve --skolemize-universal --min-consts 3 --multi-enforce 7 -w 1,1,2 --abstract-domain-vars -o %s %s" % (od, f))

        # move the file down to the top level of the scratchfolder
        of_names = os.listdir("%s/scratch" % (od))

        out = [sp.getoutput("cp %s/scratch/%s %s" % (od, ofn, self.args.scratchfolder)) for ofn in of_names]

        # remove the valfuzz scratch folder to avoid name collisions in future calls
        out = sp.getoutput("rm -r %s" % od)

        of_names = os.listdir("%s" % (self.args.scratchfolder))
        print("ValFuzz:" + str(of_names))


        ofs = ["%s/%s" % (self.args.scratchfolder, ofn) for ofn in of_names]

        # this function should return a list as valfuzz will return 0 to 2 files per iteration depending on smt solver results
        return ofs
