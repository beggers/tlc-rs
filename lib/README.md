This library implements most of the actual model-checker functionality.

- `ast/` contains the AST definitions.
- `parser/` contains parsing logic to turn a set of input files into an AST.
- `static_analysis/` contains logic to ensure that an AST is well-formed.
- `state/` contains representations for model states.
- `executor/` contains and logic which actually checks states.
