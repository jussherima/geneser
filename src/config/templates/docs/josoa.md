# Josoa Template

Production-grade Flutter architecture with Riverpod, GoRouter, multi-flavor support (dev/stg/prod), and optional Firebase, Sentry, Stripe, and i18n integration.

## Architecture

- **State Management**: Riverpod + riverpod_annotation (code generation)
- **Routing**: GoRouter with StatefulShellRoute navigation
- **Flavors**: dev / stg / prod entry points with AppEnv configuration
- **Clean Architecture**: domain / data / application / presentation per feature

## Features

Each selected feature generates a complete module:
- `domain/{feature}.dart` — Equatable domain model
- `data/{feature}_repository.dart` — CRUD repository (Firestore or in-memory)
- `application/{feature}_service.dart` — Service layer
- `presentation/{feature}_screen.dart` — ConsumerWidget screen
- `presentation/{feature}_controller.dart` — AsyncNotifier controller

## Conditional Modules

- **Firebase**: Auth exceptions, Firestore repositories, initialization
- **Sentry**: Error logging, navigation observer, DSN configuration
- **Stripe**: Payment exception, stripe initialization, env keys
- **Intl**: Localization setup with SupportedLocale enum
