# 🥞 Z-Index Scanner 

A CLI tool that finds and displays z-index layers in your JavaScript/TypeScript project, showing which elements appear on top of others.

![Demo](demo.gif)

## Features

- Scans JavaScript/TypeScript files for z-index definitions
- Supports multiple z-index formats:
  - Tailwind CSS: `z-[10]` or `z-10`
  - React Native: `zIndex: 10`
  - React 'style' prop: `z-index: 10`
- Sorts z-index values in descending order
- Excludes hidden directories and `node_modules`
- Provides a tree-like visualization of z-index hierarchy


# Installation

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