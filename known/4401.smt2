(declare-datatypes () (
                        (tree
                            (numLeaf  (num Int))
                            (boolLeaf (bool Bool))
                            (node (left tree) (right tree))
                        )
                        (treeType
                            (numT)
                            (boolT)
                            (Prod (left treeType) (right treeType))
                        )
                      )
)

(define-fun-rec correctType ((t tree) (T treeType)) Bool
    (match t
        (
            ((numLeaf  _) (= T numT))
            ((boolLeaf _) (= T boolT))
            ((node ta tb)
                (exists ((TA treeType) (TB treeType))
                    (and (correctType ta TA) (correctType tb TB) (= T (Prod TA TB)))))
        )
    )
)

(declare-const TARGET treeType)
(assert (correctType (node (numLeaf 1) (boolLeaf true)) TARGET))
(check-sat)
