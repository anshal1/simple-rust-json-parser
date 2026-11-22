# Simple JSON Parser

A lightweight JSON parser written in Rust, built from scratch as a learning exercise. It includes a lexer (tokenizer) and a recursive descent parser that converts JSON into a typed Rust representation.

## Features

### Currently Implemented

| Feature | Status | Notes |
|---------|--------|-------|
| **Objects** (`{ ... }`) | ✅ | Parses key-value pairs with string keys |
| **Arrays** (`[ ... ]`) | ✅ | Supports nested arrays |
| **Strings** (`"..."`) | ✅ | Basic string parsing with quote delimiters |
| **Integers** | ⚠️ Partial | Token type exists (`NumberToken(i128)`) but lexer doesn't emit it yet |
| **Whitespace handling** | ✅ | Skips spaces between tokens |
| **Error reporting** | ✅ | Returns descriptive parse errors |

### Data Model

```rust
enum JsonVal {
    String(String),
    Number(f64),      // Currently unused - parser doesn't produce numbers
    Bool(bool),       // Not yet implemented
    Array(Vec<JsonVal>),
    Object(HashMap<String, JsonVal>),
}
```

## Missing / Incomplete Features

### High Priority
- [ ] **Number parsing** - Lexer has commented-out code for integer parsing but doesn't handle:
  - Floating point numbers (`3.14`, `-0.5`)
  - Scientific notation (`1e10`, `2.5E-3`)
  - Negative numbers
- [ ] **Boolean literals** - `true` / `false` (token variants exist but commented out)
- [ ] **Null literal** - `null` (token variant exists but commented out)

### Medium Priority
- [ ] **String escape sequences** - `\n`, `\t`, `\"`, `\\`, `\uXXXX`
- [ ] **Unicode support** in strings
- [ ] **Multi-line string handling** - Currently processes file line-by-line, breaking multi-line strings
- [ ] **Trailing comma tolerance** - Optional but common in JSON-like configs

### Nice to Have
- [ ] **Serialization** - Convert `JsonVal` back to JSON string
- [ ] **Pretty printing** - Formatted output
- [ ] **CLI arguments** - Accept file path as argument instead of hardcoded `sample.json`
- [ ] **Streaming/Incremental parsing** - For large files
- [ ] **Comprehensive test suite**
- [ ] **Benchmarking** against `serde_json`

## Project Structure

```
simple-json-parser/
├── Cargo.toml
├── sample.json          # Test input file
└── src/
    └── main.rs          # Lexer + Parser + Main entry point
```

## Usage

```bash
# Build
cargo build --release

# Run (parses sample.json in project root)
cargo run
```

## Example

Given `sample.json`:
```json
{
  "glossary": {
    "title": "example glossary",
    "GlossDiv": {
      "title": "S",
      "GlossList": {
        "GlossEntry": {
          "ID": "SGML",
          "SortAs": "SGML",
          "GlossTerm": "Standard Generalized Markup Language",
          "Acronym": "SGML",
          "Abbrev": "ISO 8879:1986",
          "GlossDef": {
            "para": "A meta-markup language, used to create markup languages such as DocBook.",
            "GlossSeeAlso": ["GML", "XML"]
          },
          "GlossSee": "markup"
        }
      }
    }
  }
}
```

Output:
```text
Object({
    "glossary": Object({
        "title": String("example glossary"),
        "GlossDiv": Object({
            "title": String("S"),
            "GlossList": Object({
                "GlossEntry": Object({
                    "ID": String("SGML"),
                    "SortAs": String("SGML"),
                    "GlossTerm": String("Standard Generalized Markup Language"),
                    "Acronym": String("SGML"),
                    "Abbrev": String("ISO 8879:1986"),
                    "GlossDef": Object({
                        "para": String("A meta-markup language, used to create markup languages such as DocBook."),
                        "GlossSeeAlso": Array([
                            String("GML"),
                            String("XML"),
                        ]),
                    }),
                    "GlossSee": String("markup"),
                }),
            }),
        }),
    }),
})
```

## Architecture

### Lexer (`Lexer`)
- Single-pass character scanner
- Produces `Vec<Token>` from input string
- Tokens: `CurlOpen`, `CurlClose`, `Colon`, `Comma`, `OpenBracket`, `CloseBracket`, `QuoteStart`, `QuoteEnd`, `StringToken(String)`, `NumberToken(i128)`

### Parser (`Parser`)
- Recursive descent parser consuming `Vec<Token>`
- Methods: `parse()`, `parse_tokens()`, `parse_object()`, `parse_array()`, `parse_string()`
- Returns `Result<JsonVal, String>`

## License

MIT
