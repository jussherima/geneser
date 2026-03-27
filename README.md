```
   ██████╗ ███████╗███╗   ██╗███████╗███████╗███████╗██████╗
  ██╔════╝ ██╔════╝████╗  ██║██╔════╝██╔════╝██╔════╝██╔══██╗
  ██║  ███╗█████╗  ██╔██╗ ██║█████╗  ███████╗█████╗  ██████╔╝
  ██║   ██║██╔══╝  ██║╚██╗██║██╔══╝  ╚════██║██╔══╝  ██╔══██╗
  ╚██████╔╝███████╗██║ ╚████║███████╗███████║███████╗██║  ██║
   ╚═════╝ ╚══════╝╚═╝  ╚═══╝╚══════╝╚══════╝╚══════╝╚═╝  ╚═╝
```

> **Flutter project scaffolding CLI** — Generate production-ready architectures from battle-tested templates.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/geneser.svg)](https://crates.io/crates/geneser)
[![npm](https://img.shields.io/npm/v/geneser.svg)](https://www.npmjs.com/package/geneser)

---

## Installation

### Via npm (recommended)

```bash
npm install -g geneser
```

### Via cargo

```bash
cargo install --path .
```

---

## Usage

### Create a project

```bash
geneser create
```

Or with a name directly:

```bash
geneser create --name my_project
```

The CLI guides you with interactive prompts to:
1. Choose a template
2. Configure template-specific options

### List available templates

```bash
geneser list
```

---

## Templates

### CodeWithAndrea — Feature-first

Architecture inspired by [CodeWithAndrea](https://www.codewithAndrea.com). Layered feature-first structure:

```
lib/
├── src/
│   ├── features/          # Domain-organized features (presentation/application/domain/data)
│   ├── common_widgets/    # Shared widgets
│   ├── routing/           # GoRouter setup
│   ├── constants/
│   ├── exceptions/
│   ├── utils/
│   └── localization/
```

Define your initial features at creation time (e.g. `auth`, `home`, `profile`).

### CodeWithAndrea — Medium

Production-grade structure with optional Firebase and Sentry integration.

```
lib/
├── src/
│   ├── features/
│   ├── routing/
│   ├── exceptions/
│   └── utils/
```

Configurable at generation:
- **Firebase** — None / Auth only / Full (Auth + Firestore + Storage)
- **Observability** — None / Sentry

---

## Development

```bash
# Build
cargo build --release

# Run locally
cargo run -- create
cargo run -- list

# Tests
cargo test
```

---

## License

MIT

See [LICENSE](./LICENSE) for full terms.
