This library implements most of the actual model-checker functionality.

- `parser/` contains parsing logic to turn a set of input files into an AST.
- `ast/` contains the AST definitions.
- `analyzer/` contains logic to ensure that an AST is well-formed and produce a Model.
- `model/` contains representations for the model and its states.
- `executor/` contains logic to check states.
