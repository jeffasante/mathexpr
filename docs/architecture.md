mathexpr/
├── Cargo.toml
├── README.md
├── examples/
│   └── basic_expressions.rs
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── token/
│   │   ├── mod.rs          # Token definitions and core token types
│   │   └── tokenizer.rs    # Pure functions for tokenization
│   ├── parser/
│   │   ├── mod.rs          # Parser trait and core parsing logic
│   │   └── expression.rs   # Expression-specific parsing functions
│   └── evaluator/
│       ├── mod.rs          # Evaluation traits and types
│       └── arithmetic.rs    # Pure arithmetic evaluation functions
└── tests/
    ├── tokenizer_tests.rs
    ├── parser_tests.rs
    └── evaluator_tests.rs