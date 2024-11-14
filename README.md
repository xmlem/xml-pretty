# xml-pretty

Make your XML pretty. A frontend to the [`xmlem`](https://github.com/xmlem/xmlem) crate.

### Features

- It pretty prints your XML
- Outputs to file, terminal or replaces original file
- Handle entities properly and can enforce hex entities
- Customisable indentation width and maximum file width
- Ought not eat your comments even if they're outside the root element

## Installation

```
cargo install --git https://github.com/xmlem/xml-pretty
```

## Usage

Use `xml-pretty --help` to see the flags.

Simplest invocation is `xml-pretty <file>`.

## License

This project is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
