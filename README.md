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

## Commands

| Command | Description |
|---------|-------------|
| `geneser create` | Create a new Flutter project interactively |
| `geneser create --name <name>` | Create a project with a pre-set name |
| `geneser list` | List all available templates |
| `geneser template <name>` | Show documentation for a specific template |

---

## Creating a project

```bash
geneser create
# or
geneser create --name my_app
```

The CLI walks you through a series of prompts:

### 1. Project name

If not passed via `--name`, geneser asks for one. Rules:
- Only lowercase letters, digits and underscores (`a-z`, `0-9`, `_`)
- Cannot start with a digit
- Max 128 characters

### 2. Template selection

Choose from the official templates or any community template found in `.geneser/templates/`.

```
? Choisissez un template:
  ❯ CodeWithAndrea (Feature-first)
    CodeWithAndrea (Medium)
    Feature-First MVVM + GetX
    Clean Architecture + BLoC
    Riverpod + Freezed (Minimal)
    Custom (Choisir packages)
```

### 3. Template-specific options

Each template has its own questions (features, Firebase, HTTP client, etc.).
See the **Templates** section below for the full list.

### 4. Package versions

```
? Versions des packages:
  ❯ Stable (versions pinned)   → e.g. flutter_riverpod: ^2.5.1
    Latest (any)               → flutter_riverpod: any
```

- **Stable** — recommended for production. Geneser pins known-good versions so `flutter pub get` always resolves the same graph.
- **Latest** — resolves the newest compatible version at the time of `pub get`. Useful for experimenting.

If a package is not in Geneser's version table, it always falls back to `any`.

### 5. Starter code

```
? Generer le boilerplate de démarrage (main.dart, screens, repos) ? (Y/n)
```

- **Yes (default)** — every generated `.dart` file contains real, compilable code adapted to the template's state management:
  - `main.dart` — properly initializes the app (ProviderScope, GetMaterialApp, MultiProvider…)
  - `*_screen.dart` — Scaffold with AppBar and body, using the right widget type (ConsumerWidget, StatelessWidget…)
  - `*_repository.dart` — abstract class + concrete implementation stub
  - `*_controller.dart` (GetX) — GetxController with `onInit`
  - `*_cubit.dart` (BLoC) — sealed State + Cubit in one file
- **No** — generates `// TODO: Implement` stubs. Useful if you want a clean file tree to fill in yourself.

### 6. Summary and confirmation

```
Recapitulatif :
 - Projet   : my_app
 - Template : CodeWithAndrea (Feature-first)
 - Features : authentication, home
 - Versions : Stable (versions pinned)
 - Starter  : Boilerplate complet

? Generer ce projet maintenant ? (Y/n)
```

### What happens after confirmation

1. `flutter create <name>` — creates the base Flutter project
2. Folder structure is generated
3. Dart files are written with the right content
4. `pubspec.yaml` is updated with all required packages
5. `flutter pub get` is run

---

## Templates

### CodeWithAndrea — Feature-first

**State management:** Riverpod  |  **Routing:** GoRouter

Feature-first architecture by [Andrea Bizzotto](https://www.codewithAndrea.com).
All code for a feature lives in one place.

```
lib/
├── main.dart
└── src/
    ├── features/
    │   └── {feature}/
    │       ├── presentation/    ← Widgets, screens
    │       ├── application/     ← Services, use cases, providers
    │       ├── domain/          ← Pure Dart models, repo interfaces
    │       └── data/            ← Repository implementations
    ├── common_widgets/
    ├── constants/
    ├── exceptions/
    ├── localization/
    ├── routing/
    └── utils/
```

**Prompts:**

| Prompt | Options |
|--------|---------|
| Features to generate | `authentication`, `home`, `products`, `cart`, `orders`, `reviews` |

---

### CodeWithAndrea — Medium

**State management:** Riverpod  |  **Routing:** GoRouter

Production-grade version with optional Firebase and observability.
Includes app startup management, error handling widgets, and root config files
(analysis_options, lefthook, fvm, commitlint).

**Prompts:**

| Prompt | Options |
|--------|---------|
| Firebase | None / Auth + Firestore / Full (Auth + Firestore + Storage + Functions + Messaging) |
| Observability | None / Sentry / Sentry + Mixpanel |
| Additional features | `profile`, `settings`, `notifications` (`home` always included) |

---

### Feature-First MVVM + GetX

**State management:** GetX  |  **Routing:** GetX routing

Combines GetX's all-in-one approach with the MVVM pattern.
Low boilerplate, fast to set up, familiar for GetX users.

```
lib/
├── main.dart
├── app/
│   ├── bindings/
│   ├── routes/
│   └── themes/
├── core/
│   ├── constants/
│   ├── errors/
│   ├── network/      ← if Dio or http selected
│   └── storage/      ← if get_storage or Hive selected
└── features/
    └── {feature}/
        ├── bindings/
        ├── controllers/
        ├── models/
        ├── repositories/
        └── views/
```

**Prompts:**

| Prompt | Options |
|--------|---------|
| Local storage | None / `get_storage` / `hive` |
| HTTP client | None / `dio` / `http` |
| Features | `auth`, `home`, `profile`, `settings`, `dashboard` |

---

### Clean Architecture + BLoC

**State management:** BLoC  |  **Routing:** GoRouter  |  **DI:** get_it or injectable

The most structured template. Domain, data and presentation are fully decoupled.
Use this for long-lived production apps where testability and maintainability are critical.

```
lib/
├── main.dart
├── app/
│   ├── router/
│   └── themes/
├── core/
│   ├── errors/       ← Failures + Exceptions
│   ├── usecases/     ← Generic UseCase<T, P> interface
│   ├── network/      ← if Dio or http selected
│   └── local_db/     ← if Drift, Hive or SharedPrefs selected
├── injection/
│   └── injection_container.dart
└── features/
    └── {feature}/
        ├── presentation/
        │   ├── pages/
        │   ├── widgets/
        │   └── bloc/
        ├── domain/
        │   ├── entities/
        │   ├── repositories/   ← Interfaces
        │   └── usecases/
        └── data/
            ├── models/
            ├── repositories/   ← Implementations
            └── datasources/
```

**Prompts:**

| Prompt | Options |
|--------|---------|
| HTTP client | None / `dio` / `http` |
| Local database | None / `drift` / `hive` / `shared_preferences` |
| Dependency injection | `get_it` / `injectable` |
| Features | `auth`, `home`, `profile`, `settings`, `dashboard` |

---

### Riverpod + Freezed (Minimal)

**State management:** Riverpod  |  **Routing:** GoRouter

The pragmatic middle ground. Riverpod + Freezed cover 90% of app needs with minimal ceremony.
Simpler than CWA Medium, more opinionated than Custom.

```
lib/
├── main.dart
└── src/
    ├── app/
    ├── routing/
    ├── common_widgets/
    ├── constants/
    ├── exceptions/
    ├── network/      ← if Dio or http selected
    ├── storage/      ← if SharedPrefs or Hive selected
    ├── utils/
    └── features/
        └── {feature}/
            ├── presentation/
            ├── providers/
            ├── models/
            └── repositories/
```

**Prompts:**

| Prompt | Options |
|--------|---------|
| HTTP client | None / `dio` / `http` |
| Local persistence | None / `shared_preferences` / `hive` |
| Features | `auth`, `home`, `profile`, `settings` |

---

### Custom

Pick your own stack from scratch.

**Prompts:**

| Prompt | Options |
|--------|---------|
| State management | Riverpod / BLoC / GetX / Provider |
| Routing | GoRouter / AutoRoute / GetX routing / Navigator 2.0 |
| Extra packages | Drift, Dio, Freezed, FlutterGen, Hive, SharedPreferences, Firebase, Equatable, Dartz, Intl |
| Base features | `authentication`, `home` |

---

## Community templates

Place a `.json` file following the template schema in `.geneser/templates/` at the root of your project (or home directory).
It will appear in the template menu tagged `[communautaire]`.

See `src/config/templates/_community_example.json` for the schema.

---

## Choosing a template

| Situation | Recommended template |
|-----------|---------------------|
| Learning Flutter architecture | CWA Feature-first |
| Production app, team of 2–6 | CWA Medium |
| Team already on GetX | Feature-First MVVM + GetX |
| Complex business rules, long-term project | Clean Architecture + BLoC |
| Solo dev, small-to-medium app | Riverpod + Freezed (Minimal) |
| Specific package combo | Custom |

---

## Development

```bash
# Build
cargo build --release

# Run locally
cargo run -- create
cargo run -- list
cargo run -- template "Clean Architecture + BLoC"

# Tests
cargo test
```

---

## License

MIT — see [LICENSE](./LICENSE) for full terms.
