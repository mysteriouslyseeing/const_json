# const_json

Provides a way to embed and access const JSON in Rust code, using a single `macro_rules`
declaration, and no dependencies, so it is quick to compile.

```rust
use const_json::{Json, const_json};

const JSON: Json = const_json!({
    "null": null,
    "bool": true,
    "float": 12.3,
    "int": 42,
    "str": "Hello, World!",
    "array": [1, null],
    "object": {
        "inner_bool": false,
        "inner_str": "foo bar"
    },

    "variable": VARIABLE,
    // Has to be surrounded in parentheses if it is a complex expression
    "function_result": (10 + 4)
});

const VARIABLE: i64 = 10;
```
