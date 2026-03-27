# CodeWithAndrea — Medium

Auteur : Andrea Bizzotto (codewithAndrea.com) — adapté par Geneser

---

## Motivation

Variante plus complète du template Feature-First, conçue pour des applications
en production. Elle intègre dès le départ les outils transversaux qu'on ajoute
toujours au bout de quelques semaines : observabilité (Sentry), analytics,
Firebase, et une config qualité (linting strict, hooks de commit, FVM).

L'objectif est d'éviter la dette technique d'ajouter ces outils "plus tard"
— ce qui finit souvent par ne jamais arriver proprement.

---

## Vue d'ensemble

```
lib/
├── main.dart
└── src/
    ├── features/
    │   └── {feature}/
    │       ├── presentation/
    │       ├── application/
    │       ├── domain/
    │       └── data/
    ├── common_widgets/
    ├── constants/
    ├── exceptions/          ← AppException + codes d'erreur typés
    ├── localization/
    ├── routing/
    │   ├── app_router.dart
    │   ├── app_route.dart
    │   ├── initialization/  ← Startup flow (splash, init async)
    │   └── go_router_delegate_listener.dart  (si Sentry activé)
    ├── services/
    │   └── error_logger.dart  ← Abstraction Sentry/console
    └── utils/

Racine du projet :
├── .fvmrc                   ← Version Flutter fixée
├── analysis_options.yaml    ← very_good_analysis (strict)
├── lefthook.yaml            ← Hooks pre-commit
└── commitlint.config.js     ← Conventional commits
```

---

## Options configurables à la génération

### Firebase
| Choix | Packages ajoutés |
|-------|-----------------|
| Aucun | — |
| Auth + Firestore | firebase_core, firebase_auth, cloud_firestore |
| Full | + cloud_functions, firebase_storage, firebase_messaging |

### Observabilité
| Choix | Packages ajoutés |
|-------|-----------------|
| Aucun | — |
| Sentry | sentry_flutter + go_router_delegate_listener |
| Sentry + Analytics | + mixpanel_flutter |

---

## Couches

### `exceptions/`
`AppException` est une sealed class qui centralise toutes les erreurs métier.
`AppExceptionCode` est une enum des codes d'erreur typés (pas de strings magiques).

```dart
throw AppException(code: AppExceptionCode.productNotFound);
```

### `routing/initialization/`
Gère le démarrage asynchrone de l'app (chargement config, auth check…).
`AppStartupController` expose un `AsyncValue` que `AppStartupWidgetWrapper` observe
pour afficher splash / erreur / app selon l'état.

### `services/error_logger.dart`
Abstraction autour de Sentry (ou console en dev). Tous les blocs `catch` passent
par ce service, pas directement par `Sentry.captureException`.

---

## Quand utiliser

- Application de production dès le départ
- Équipe qui veut un setup qualité prêt à l'emploi (linting, hooks, FVM)
- Projets avec Firebase ou Sentry planifiés
- Quand on ne veut pas "ajouter les outils plus tard"

## Quand NE PAS utiliser

- Prototype ou preuve de concept — trop lourd
- Si vous n'utilisez pas FVM (version Flutter fixée dans .fvmrc)
- Si votre équipe n'est pas à l'aise avec le linting très strict (very_good_analysis)

---

## Ressources

- https://www.codewithAndrea.com/articles/flutter-project-structure
- https://pub.dev/packages/very_good_analysis
- https://riverpod.dev
- https://docs.sentry.io/platforms/flutter
