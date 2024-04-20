=============|||
Infix Operator Definition
=============|||

---- MODULE Test ----

x & y == 0
x && y == 1
x \approx y == 2
x := y == 3
x \asymp y == 4
x \bigcirc y == 5
x ::= y == 6
x \bullet y == 7
x \intersect y == 8
x \cap y == 9
x \cdot y == 10
x \o y == 11
x \circ y == 12
x @@ y == 13
x \cong y == 14
x \union y == 15
x \cup y == 16
x \div y == 17
x $ y == 18
x $$ y == 19
x \doteq y == 20
x .. y == 21
x ... y == 22
x = y == 23
x \equiv y == 24
x !! y == 25
x >= y == 26
x \geq y == 27
x \gg y == 28
x > y == 29
x ## y == 30
x <=> y == 31
x => y == 32
x \in y == 33
x /\ y == 34
x \land y == 35
x =| y == 36
x ~> y == 37
x <= y == 38
x =< y == 39
x \leq y == 40
x \ll y == 41
x \/ y == 42
x \lor y == 43
x -| y == 44
x < y == 45
x <: y == 46
x :> y == 47
x - y == 48
x -- y == 49
x % y == 50
x %% y == 51
x * y == 52
x ** y == 53
x /= y == 54
x # y == 55
x \notin y == 56
x (.) y == 57
x \odot y == 58
x (-) y == 59
x \ominus y == 60
x (+) y == 61
x \oplus y == 62
x (/) y == 63
x \oslash y == 64
x (\X) y == 65
x \otimes y == 66
x + y == 67
x -+-> y == 68
x ++ y == 69
x ^ y == 70
x ^^ y == 71
x \prec y == 72
x \preceq y == 73
x \propto y == 74
x ?? y == 75
x |= y == 76
x |- y == 77
x \ y == 78
x \sim y == 79
x \simeq y == 80
x / y == 81
x // y == 82
x \sqcap y == 83
x \sqcup y == 84
x \sqsubset y == 85
x \sqsubseteq y == 86
x \sqsupset y == 87
x \sqsupseteq y == 88
x \star y == 89
x \subset y == 90
x \subseteq y == 91
x \succ y == 92
x \succeq y == 93
x \supset y == 94
x \supseteq y == 95
x \X y == 96
x \times y == 97
x \uplus y == 98
x | y == 99
x || y == 100
x \wr y == 101

====

-------------|||

(source_file (module (header_line) name: (identifier) (header_line)
  (operator_definition parameter: (identifier) name: (infix_op_symbol (amp)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (ampamp)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (approx)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (assign)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (asymp)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (bigcirc)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (bnf_rule)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (bullet)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (cap)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (cap)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (cdot)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (circ)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (circ)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (compose)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (cong)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (cup)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (cup)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (div)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (dol)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (doldol)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (doteq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (dots_2)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (dots_3)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (eq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (equiv)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (excl)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (geq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (geq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (gg)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (gt)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (hashhash)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (iff)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (implies)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (in)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (land)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (land)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (ld_ttile)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (leads_to)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (leq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (leq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (leq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (ll)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (lor)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (lor)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (ls_ttile)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (lt)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (map_from)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (map_to)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (minus)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (minusminus)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (mod)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (modmod)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (mul)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (mulmul)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (neq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (neq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (notin)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (odot)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (odot)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (ominus)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (ominus)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (oplus)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (oplus)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (oslash)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (oslash)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (otimes)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (otimes)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (plus)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (plus_arrow)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (plusplus)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (pow)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (powpow)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (prec)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (preceq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (propto)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (qq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (rd_ttile)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (rs_ttile)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (setminus)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (sim)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (simeq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (slash)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (slashslash)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (sqcap)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (sqcup)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (sqsubset)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (sqsubseteq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (sqsupset)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (sqsupseteq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (star)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (subset)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (subseteq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (succ)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (succeq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (supset)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (supseteq)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (times)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (times)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (uplus)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (vert)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (vertvert)) parameter: (identifier) (def_eq) definition: (nat_number))
  (operator_definition parameter: (identifier) name: (infix_op_symbol (wr)) parameter: (identifier) (def_eq) definition: (nat_number))
(double_line)))


=============|||
Infix Operator Declaration as Parameter
=============|||

---- MODULE Test ----

op(
  _ & _,
  _ && _,
  _ \approx _,
  _ := _,
  _ \asymp _,
  _ \bigcirc _,
  _ ::= _,
  _ \bullet _,
  _ \intersect _,
  _ \cap _,
  _ \cdot _,
  _ \o _,
  _ \circ _,
  _ @@ _,
  _ \cong _,
  _ \union _,
  _ \cup _,
  _ \div _,
  _ $ _,
  _ $$ _,
  _ \doteq _,
  _ .. _,
  _ ... _,
  _ = _,
  _ \equiv _,
  _ !! _,
  _ >= _,
  _ \geq _,
  _ \gg _,
  _ > _,
  _ ## _,
  _ <=> _,
  _ => _,
  _ \in _,
  _ /\ _,
  _ \land _,
  _ =| _,
  _ ~> _,
  _ <= _,
  _ =< _,
  _ \leq _,
  _ \ll _,
  _ \/ _,
  _ \lor _,
  _ -| _,
  _ < _,
  _ <: _,
  _ :> _,
  _ - _,
  _ -- _,
  _ % _,
  _ %% _,
  _ * _,
  _ ** _,
  _ /= _,
  _ # _,
  _ \notin _,
  _ (.) _,
  _ \odot _,
  _ (-) _,
  _ \ominus _,
  _ (+) _,
  _ \oplus _,
  _ (/) _,
  _ \oslash _,
  _ (\X) _,
  _ \otimes _,
  _ + _,
  _ -+-> _,
  _ ++ _,
  _ ^ _,
  _ ^^ _,
  _ \prec _,
  _ \preceq _,
  _ \propto _,
  _ ?? _,
  _ |= _,
  _ |- _,
  _ \ _,
  _ \sim _,
  _ \simeq _,
  _ / _,
  _ // _,
  _ \sqcap _,
  _ \sqcup _,
  _ \sqsubset _,
  _ \sqsubseteq _,
  _ \sqsupset _,
  _ \sqsupseteq _,
  _ \star _,
  _ \subset _,
  _ \subseteq _,
  _ \succ _,
  _ \succeq _,
  _ \supset _,
  _ \supseteq _,
  _ \X _,
  _ \times _,
  _ \uplus _,
  _ | _,
  _ || _,
  _ \wr _
) == 1

====

-------------|||

(source_file (module (header_line) name: (identifier) (header_line)
  (operator_definition name: (identifier)
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (amp)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (ampamp)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (approx)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (assign)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (asymp)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (bigcirc)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (bnf_rule)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (bullet)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (cap)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (cap)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (cdot)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (circ)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (circ)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (compose)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (cong)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (cup)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (cup)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (div)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (dol)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (doldol)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (doteq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (dots_2)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (dots_3)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (eq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (equiv)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (excl)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (geq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (geq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (gg)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (gt)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (hashhash)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (iff)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (implies)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (in)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (land)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (land)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (ld_ttile)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (leads_to)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (leq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (leq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (leq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (ll)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (lor)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (lor)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (ls_ttile)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (lt)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (map_from)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (map_to)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (minus)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (minusminus)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (mod)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (modmod)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (mul)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (mulmul)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (neq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (neq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (notin)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (odot)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (odot)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (ominus)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (ominus)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (oplus)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (oplus)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (oslash)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (oslash)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (otimes)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (otimes)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (plus)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (plus_arrow)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (plusplus)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (pow)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (powpow)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (prec)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (preceq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (propto)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (qq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (rd_ttile)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (rs_ttile)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (setminus)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (sim)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (simeq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (slash)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (slashslash)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (sqcap)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (sqcup)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (sqsubset)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (sqsubseteq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (sqsupset)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (sqsupseteq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (star)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (subset)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (subseteq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (succ)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (succeq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (supset)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (supseteq)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (times)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (times)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (uplus)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (vert)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (vertvert)) (placeholder))
    parameter: (operator_declaration (placeholder) name: (infix_op_symbol (wr)) (placeholder))
    (def_eq)
    definition: (nat_number)
  )
(double_line)))

=============|||
Infix Operator Application
=============|||

---- MODULE Test ----

op == {
  x & y,
  x && y,
  x \approx y,
  x := y,
  x \asymp y,
  x \bigcirc y,
  x ::= y,
  x \bullet y,
  x \intersect y,
  x \cap y,
  x \cdot y,
  x \o y,
  x \circ y,
  x @@ y,
  x \cong y,
  x \union y,
  x \cup y,
  x \div y,
  x $ y,
  x $$ y,
  x \doteq y,
  x .. y,
  x ... y,
  x = y,
  x \equiv y,
  x !! y,
  x >= y,
  x \geq y,
  x \gg y,
  x > y,
  x ## y,
  x <=> y,
  x => y,
  x \in y,
  x /\ y,
  x \land y,
  x =| y,
  x ~> y,
  x <= y,
  x =< y,
  x \leq y,
  x \ll y,
  x \/ y,
  x \lor y,
  x -| y,
  x < y,
  x <: y,
  x :> y,
  x - y,
  x -- y,
  x % y,
  x %% y,
  x * y,
  x ** y,
  x /= y,
  x # y,
  x \notin y,
  x (.) y,
  x \odot y,
  x (-) y,
  x \ominus y,
  x (+) y,
  x \oplus y,
  x (/) y,
  x \oslash y,
  x (\X) y,
  x \otimes y,
  x + y,
  x -+-> y,
  x ++ y,
  x ^ y,
  x ^^ y,
  x \prec y,
  x \preceq y,
  x \propto y,
  x ?? y,
  x |= y,
  x |- y,
  x \ y,
  x \sim y,
  x \simeq y,
  x / y,
  x // y,
  x \sqcap y,
  x \sqcup y,
  x \sqsubset y,
  x \sqsubseteq y,
  x \sqsupset y,
  x \sqsupseteq y,
  x \star y,
  x \subset y,
  x \subseteq y,
  x \succ y,
  x \succeq y,
  x \supset y,
  x \supseteq y,
  x \X y,
  x \times y,
  x \uplus y,
  x | y,
  x || y,
  x \wr y
}

====

-------------|||

(source_file (module (header_line) name: (identifier) (header_line)
  (operator_definition name: (identifier) (def_eq) definition: (finite_set_literal
    (bound_infix_op lhs: (identifier_ref) symbol: (amp) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (ampamp) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (approx) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (assign) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (asymp) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (bigcirc) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (bnf_rule) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (bullet) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (cap) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (cap) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (cdot) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (circ) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (circ) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (compose) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (cong) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (cup) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (cup) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (div) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (dol) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (doldol) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (doteq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (dots_2) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (dots_3) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (eq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (equiv) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (excl) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (geq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (geq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (gg) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (gt) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (hashhash) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (iff) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (implies) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (in) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (land) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (land) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (ld_ttile) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (leads_to) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (leq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (leq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (leq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (ll) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (lor) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (lor) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (ls_ttile) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (lt) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (map_from) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (map_to) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (minus) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (minusminus) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (mod) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (modmod) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (mul) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (mulmul) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (neq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (neq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (notin) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (odot) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (odot) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (ominus) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (ominus) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (oplus) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (oplus) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (oslash) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (oslash) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (otimes) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (otimes) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (plus) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (plus_arrow) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (plusplus) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (pow) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (powpow) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (prec) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (preceq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (propto) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (qq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (rd_ttile) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (rs_ttile) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (setminus) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (sim) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (simeq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (slash) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (slashslash) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (sqcap) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (sqcup) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (sqsubset) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (sqsubseteq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (sqsupset) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (sqsupseteq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (star) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (subset) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (subseteq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (succ) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (succeq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (supset) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (supseteq) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (times) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (times) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (uplus) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (vert) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (vertvert) rhs: (identifier_ref))
    (bound_infix_op lhs: (identifier_ref) symbol: (wr) rhs: (identifier_ref))
  ))
(double_line)))

=============|||
Infix Operators as Parameters
=============|||

---- MODULE Test ----
op == f(
  &,
  &&,
  \approx,
  :=,
  \asymp,
  \bigcirc,
  ::=,
  \bullet,
  \intersect,
  \cap,
  \cdot,
  \o,
  \circ,
  @@,
  \cong,
  \union,
  \cup,
  \div,
  $,
  $$,
  \doteq,
  ..,
  ...,
  =,
  \equiv,
  !!,
  >=,
  \geq,
  \gg,
  >,
  ##,
  <=>,
  =>,
  \in,
  \land,
  =|,
  ~>,
  <=,
  =<,
  \leq,
  \ll,
  \lor,
  -|,
  <,
  <:,
  :>,
  -,
  --,
  %,
  %%,
  *,
  **,
  /=,
  #,
  \notin,
  (.),
  \odot,
  (-),
  \ominus,
  (+),
  \oplus,
  (/),
  \oslash,
  (\X),
  \otimes,
  +,
  -+->,
  ++,
  ^,
  ^^,
  \prec,
  \preceq,
  \propto,
  ??,
  |=,
  |-,
  \,
  \sim,
  \simeq,
  /,
  //,
  \sqcap,
  \sqcup,
  \sqsubset,
  \sqsubseteq,
  \sqsupset,
  \sqsupseteq,
  \star,
  \subset,
  \subseteq,
  \succ,
  \succeq,
  \supset,
  \supseteq,
  \uplus,
  |,
  ||,
  \wr
)
====

-------------|||

(source_file (module (header_line) name: (identifier) (header_line)
  (operator_definition name: (identifier) (def_eq) definition: (bound_op name: (identifier_ref)
    parameter: (infix_op_symbol (amp))
    parameter: (infix_op_symbol (ampamp))
    parameter: (infix_op_symbol (approx))
    parameter: (infix_op_symbol (assign))
    parameter: (infix_op_symbol (asymp))
    parameter: (infix_op_symbol (bigcirc))
    parameter: (infix_op_symbol (bnf_rule))
    parameter: (infix_op_symbol (bullet))
    parameter: (infix_op_symbol (cap))
    parameter: (infix_op_symbol (cap))
    parameter: (infix_op_symbol (cdot))
    parameter: (infix_op_symbol (circ))
    parameter: (infix_op_symbol (circ))
    parameter: (infix_op_symbol (compose))
    parameter: (infix_op_symbol (cong))
    parameter: (infix_op_symbol (cup))
    parameter: (infix_op_symbol (cup))
    parameter: (infix_op_symbol (div))
    parameter: (infix_op_symbol (dol))
    parameter: (infix_op_symbol (doldol))
    parameter: (infix_op_symbol (doteq))
    parameter: (infix_op_symbol (dots_2))
    parameter: (infix_op_symbol (dots_3))
    parameter: (infix_op_symbol (eq))
    parameter: (infix_op_symbol (equiv))
    parameter: (infix_op_symbol (excl))
    parameter: (infix_op_symbol (geq))
    parameter: (infix_op_symbol (geq))
    parameter: (infix_op_symbol (gg))
    parameter: (infix_op_symbol (gt))
    parameter: (infix_op_symbol (hashhash))
    parameter: (infix_op_symbol (iff))
    parameter: (infix_op_symbol (implies))
    parameter: (infix_op_symbol (in))
    parameter: (infix_op_symbol (land))
    parameter: (infix_op_symbol (ld_ttile))
    parameter: (infix_op_symbol (leads_to))
    parameter: (infix_op_symbol (leq))
    parameter: (infix_op_symbol (leq))
    parameter: (infix_op_symbol (leq))
    parameter: (infix_op_symbol (ll))
    parameter: (infix_op_symbol (lor))
    parameter: (infix_op_symbol (ls_ttile))
    parameter: (infix_op_symbol (lt))
    parameter: (infix_op_symbol (map_from))
    parameter: (infix_op_symbol (map_to))
    parameter: (infix_op_symbol (minus))
    parameter: (infix_op_symbol (minusminus))
    parameter: (infix_op_symbol (mod))
    parameter: (infix_op_symbol (modmod))
    parameter: (infix_op_symbol (mul))
    parameter: (infix_op_symbol (mulmul))
    parameter: (infix_op_symbol (neq))
    parameter: (infix_op_symbol (neq))
    parameter: (infix_op_symbol (notin))
    parameter: (infix_op_symbol (odot))
    parameter: (infix_op_symbol (odot))
    parameter: (infix_op_symbol (ominus))
    parameter: (infix_op_symbol (ominus))
    parameter: (infix_op_symbol (oplus))
    parameter: (infix_op_symbol (oplus))
    parameter: (infix_op_symbol (oslash))
    parameter: (infix_op_symbol (oslash))
    parameter: (infix_op_symbol (otimes))
    parameter: (infix_op_symbol (otimes))
    parameter: (infix_op_symbol (plus))
    parameter: (infix_op_symbol (plus_arrow))
    parameter: (infix_op_symbol (plusplus))
    parameter: (infix_op_symbol (pow))
    parameter: (infix_op_symbol (powpow))
    parameter: (infix_op_symbol (prec))
    parameter: (infix_op_symbol (preceq))
    parameter: (infix_op_symbol (propto))
    parameter: (infix_op_symbol (qq))
    parameter: (infix_op_symbol (rd_ttile))
    parameter: (infix_op_symbol (rs_ttile))
    parameter: (infix_op_symbol (setminus))
    parameter: (infix_op_symbol (sim))
    parameter: (infix_op_symbol (simeq))
    parameter: (infix_op_symbol (slash))
    parameter: (infix_op_symbol (slashslash))
    parameter: (infix_op_symbol (sqcap))
    parameter: (infix_op_symbol (sqcup))
    parameter: (infix_op_symbol (sqsubset))
    parameter: (infix_op_symbol (sqsubseteq))
    parameter: (infix_op_symbol (sqsupset))
    parameter: (infix_op_symbol (sqsupseteq))
    parameter: (infix_op_symbol (star))
    parameter: (infix_op_symbol (subset))
    parameter: (infix_op_symbol (subseteq))
    parameter: (infix_op_symbol (succ))
    parameter: (infix_op_symbol (succeq))
    parameter: (infix_op_symbol (supset))
    parameter: (infix_op_symbol (supseteq))
    parameter: (infix_op_symbol (uplus))
    parameter: (infix_op_symbol (vert))
    parameter: (infix_op_symbol (vertvert))
    parameter: (infix_op_symbol (wr))
  ))
(double_line)))

=============|||
Nonfix Infix Operators
=============|||

---- MODULE Test ----
op == {
  A!B!&(x, y),
  A!B!&&(x, y),
  A!B!\approx(x, y),
  A!B!:=(x, y),
  A!B!\asymp(x, y),
  A!B!\bigcirc(x, y),
  A!B!::=(x, y),
  A!B!\bullet(x, y),
  A!B!\intersect(x, y),
  A!B!\cap(x, y),
  A!B!\cdot(x, y),
  A!B!\o(x, y),
  A!B!\circ(x, y),
  A!B!@@(x, y),
  A!B!\cong(x, y),
  A!B!\union(x, y),
  A!B!\cup(x, y),
  A!B!\div(x, y),
  A!B!$(x, y),
  A!B!$$(x, y),
  A!B!\doteq(x, y),
  A!B!..(x, y),
  A!B!...(x, y),
  A!B!=(x, y),
  A!B!\equiv(x, y),
  A!B!>=(x, y),
  A!B!\geq(x, y),
  A!B!\gg(x, y),
  A!B!>(x, y),
  A!B!##(x, y),
  A!B!<=>(x, y),
  A!B!=>(x, y),
  A!B!\in(x, y),
  A!B!\land(x, y),
  A!B!=|(x, y),
  A!B!~>(x, y),
  A!B!<=(x, y),
  A!B!=<(x, y),
  A!B!\leq(x, y),
  A!B!\ll(x, y),
  A!B!\lor(x, y),
  A!B!-|(x, y),
  A!B!<(x, y),
  A!B!<:(x, y),
  A!B!:>(x, y),
  A!B!-(x, y),
  A!B!--(x, y),
  A!B!%(x, y),
  A!B!%%(x, y),
  A!B!*(x, y),
  A!B!**(x, y),
  A!B!/=(x, y),
  A!B!#(x, y),
  A!B!\notin(x, y),
  A!B!(.)(x, y),
  A!B!\odot(x, y),
  A!B!(-)(x, y),
  A!B!\ominus(x, y),
  A!B!(+)(x, y),
  A!B!\oplus(x, y),
  A!B!(/)(x, y),
  A!B!\oslash(x, y),
  A!B!(\X)(x, y),
  A!B!\otimes(x, y),
  A!B!+(x, y),
  A!B!-+->(x, y),
  A!B!++(x, y),
  A!B!^(x, y),
  A!B!^^(x, y),
  A!B!\prec(x, y),
  A!B!\preceq(x, y),
  A!B!\propto(x, y),
  A!B!??(x, y),
  A!B!|=(x, y),
  A!B!|-(x, y),
  A!B!\(x, y),
  A!B!\sim(x, y),
  A!B!\simeq(x, y),
  A!B!/(x, y),
  A!B!//(x, y),
  A!B!\sqcap(x, y),
  A!B!\sqcup(x, y),
  A!B!\sqsubset(x, y),
  A!B!\sqsubseteq(x, y),
  A!B!\sqsupset(x, y),
  A!B!\sqsupseteq(x, y),
  A!B!\star(x, y),
  A!B!\subset(x, y),
  A!B!\subseteq(x, y),
  A!B!\succ(x, y),
  A!B!\succeq(x, y),
  A!B!\supset(x, y),
  A!B!\supseteq(x, y),
  A!B!\X(x, y),
  A!B!\times(x, y),
  A!B!\uplus(x, y),
  A!B!|(x, y),
  A!B!||(x, y),
  A!B!\wr(x, y)
}
====

-------------|||

(source_file (module (header_line) name: (identifier) (header_line)
  (operator_definition name: (identifier) (def_eq) definition: (finite_set_literal
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (amp)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (ampamp)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (approx)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (assign)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (asymp)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (bigcirc)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (bnf_rule)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (bullet)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (cap)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (cap)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (cdot)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (circ)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (circ)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (compose)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (cong)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (cup)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (cup)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (div)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (dol)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (doldol)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (doteq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (dots_2)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (dots_3)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (eq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (equiv)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (geq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (geq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (gg)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (gt)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (hashhash)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (iff)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (implies)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (in)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (land)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (ld_ttile)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (leads_to)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (leq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (leq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (leq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (ll)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (lor)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (ls_ttile)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (lt)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (map_from)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (map_to)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (minus)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (minusminus)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (mod)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (modmod)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (mul)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (mulmul)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (neq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (neq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (notin)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (odot)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (odot)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (ominus)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (ominus)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (oplus)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (oplus)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (oslash)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (oslash)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (otimes)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (otimes)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (plus)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (plus_arrow)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (plusplus)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (pow)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (powpow)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (prec)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (preceq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (propto)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (qq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (rd_ttile)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (rs_ttile)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (setminus)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (sim)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (simeq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (slash)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (slashslash)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (sqcap)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (sqcup)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (sqsubset)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (sqsubseteq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (sqsupset)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (sqsupseteq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (star)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (subset)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (subseteq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (succ)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (succeq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (supset)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (supseteq)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (times)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (times)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (uplus)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (vert)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (vertvert)) (identifier_ref) (identifier_ref)))
    (prefixed_op prefix: (subexpr_prefix (subexpr_component (identifier_ref)) (subexpr_component (identifier_ref))) op:
      (bound_nonfix_op symbol: (infix_op_symbol (wr)) (identifier_ref) (identifier_ref)))
  ))
(double_line)))

