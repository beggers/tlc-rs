=============|||
Operators with Colon Prefixes & Jlists (GH87)
=============|||

---- MODULE Test ----
op ==
  /\ A :> B
  /\ A := B
  /\ A ::= B
  /\ A :: B
  /\ A
====

--------------|||

(source_file (module (header_line) (identifier) (header_line)
  (operator_definition (identifier) (def_eq)
    (conj_list
      (conj_item (bullet_conj)
        (bound_infix_op (identifier_ref) (map_to) (identifier_ref))
      )
      (conj_item (bullet_conj)
        (bound_infix_op (identifier_ref) (assign) (identifier_ref))
      )
      (conj_item (bullet_conj)
        (bound_infix_op (identifier_ref) (bnf_rule) (identifier_ref))
      )
      (conj_item (bullet_conj)
        (label (identifier) (label_as) (identifier_ref))
      )
      (conj_item (bullet_conj) (identifier_ref))
    )
  )
(double_line)))

