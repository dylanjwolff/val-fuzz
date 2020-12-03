import sys

import unittest
sys.path.append("..")

from tests.unittests.test_type_aware_op_mutation import TypeAwareOpMutationTestCase
from tests.unittests.test_semantic_fusion import SemanticFusionTestCase
from tests.unittests.test_term import TermTestCase
from tests.unittests.test_solver import SolverTestCase

if __name__ == '__main__':
    unittest.main()
