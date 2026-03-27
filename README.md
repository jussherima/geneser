# Geneser

CLI tool to generate Flutter project architectures from famous templates.

## Installation

### Via npm (recommandé)

```bash
npm install -g geneser
```

### Via cargo

```bash
cargo install --path .
```

## Usage

### Créer un projet

```bash
geneser create
```

Ou avec le nom directement :

```bash
geneser create --name mon_projet
```

Le CLI te guide ensuite avec des prompts interactifs pour :
1. Choisir un template
2. Configurer les options du template

### Lister les templates disponibles

```bash
geneser list
```

## Templates disponibles

### CodeWithAndrea (CWA)

Architecture inspirée du cours [CodeWithAndrea](https://www.codewithAndrea.com). Structure feature-first avec :
- `lib/src/features/` — features organisées par domaine (presentation / application / domain / data)
- `lib/src/common_widgets/` — widgets réutilisables
- `lib/src/routing/` — GoRouter
- `lib/src/constants/`, `exceptions/`, `utils/`, `localization/`

Lors de la création, tu peux définir tes features initiales (ex: `auth`, `home`, `profile`).

### Fybego

Template production-grade avec support conditionnel Firebase et Sentry.

Options disponibles :
- **Firebase** : None / Auth only / Full (Auth + Firestore + Storage)
- **Observability** : None / Sentry / Sentry + Analytics

## Développement

### Build

```bash
cargo build --release
```

### Run en local

```bash
cargo run -- create
cargo run -- list
```

### Tests

```bash
cargo test
```
