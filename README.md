# 🥞 Z-Index Scanner 

A CLI tool that finds and displays z-index layers in your JavaScript/TypeScript project, showing which elements appear on top of others.

## Installation

```bash
cargo install zindex-scanner
```

## Usage

```bash
zscan <directory>
```

### Example

```bash
zscan ./src
```

### Output Example

```
Z-Index Tree:
=============
z-100
  ├─ File: src/components/Modal.tsx
  └─ Line: 15

z-50
  ├─ File: src/components/Dropdown.tsx
  └─ Line: 23

z-10
  ├─ File: src/components/Header.tsx
  └─ Line: 8
```

## Supported File Types

- `.js`
- `.jsx`
- `.ts`
- `.tsx`

## Development

### Prerequisites

- Rust (latest stable version)
- Cargo

### Building

```bash
cargo build --release
```



## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 