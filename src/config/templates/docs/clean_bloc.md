# Clean Architecture + BLoC

---

## Motivation

Clean Architecture (Robert C. Martin) appliquée à Flutter. L'objectif est
l'**indépendance totale** : le code métier (domain) ne dépend d'aucun
framework, d'aucune base de données, d'aucune API. On peut changer
Flutter, changer l'API REST en GraphQL, changer SQLite en Hive, sans
toucher aux use cases.

BLoC (Business Logic Component) assure que le state est prévisible,
testable, et que l'UI ne contient aucune logique.

Ce template est le plus structuré de Geneser. Il demande plus de
boilerplate mais c'est le choix par défaut pour les applications
critiques à long terme.

---

## Vue d'ensemble

```
lib/
├── main.dart
├── app/
│   ├── app.dart
│   ├── router/              ← GoRouter
│   └── themes/
├── core/                    ← Partagé entre toutes les features
│   ├── constants/
│   ├── errors/
│   │   ├── failures.dart    ← Failures métier (Either<Failure, T>)
│   │   └── exceptions.dart  ← Exceptions techniques
│   ├── extensions/
│   ├── network/             ← (si Dio ou http activé)
│   ├── local_db/            ← (si Drift, Hive ou SharedPrefs activé)
│   ├── usecases/
│   │   └── usecase.dart     ← Interface générique UseCase<Type, Params>
│   └── widgets/
├── injection/               ← get_it ou injectable
│   └── injection_container.dart
└── features/
    └── {feature}/
        ├── presentation/
        │   ├── pages/       ← Écrans Flutter
        │   ├── widgets/     ← Widgets spécifiques
        │   └── bloc/        ← BLoC/Cubit + events + states
        ├── domain/
        │   ├── entities/    ← Objets métier purs Dart
        │   ├── repositories/ ← Interfaces (abstraites)
        │   └── usecases/    ← Un fichier = un use case
        └── data/
            ├── models/      ← DTOs avec fromJson/toJson
            ├── repositories/ ← Implémentations concrètes
            └── datasources/ ← Appels API, DB locale
```

---

## La règle de dépendance

Les dépendances ne pointent **que vers l'intérieur** :

```
presentation  →  domain  ←  data
```

- `presentation` dépend de `domain` (via BLoC qui appelle les use cases)
- `data` dépend de `domain` (implémente les interfaces de repositories)
- `domain` ne dépend de rien

---

## Couches en détail

### `domain/` — Le cœur
Pur Dart, zéro dépendance externe. Contient :

**Entities** : objets métier immuables, validés.
```dart
class Product extends Equatable {
  final String id;
  final String name;
  final double price;
  // ...
}
```

**Repository interfaces** : contrats, pas d'implémentation.
```dart
abstract class ProductRepository {
  Future<Either<Failure, List<Product>>> getProducts();
}
```

**Use cases** : une opération métier par fichier.
```dart
class GetProducts extends UseCase<List<Product>, NoParams> {
  @override
  Future<Either<Failure, List<Product>>> call(NoParams params) =>
      repository.getProducts();
}
```

### `data/` — L'infrastructure
Implémente les interfaces du domain.

**Models** : DTOs pour la sérialisation JSON (étendent les entités ou les mappent).
**Repository implementations** : orchestrent remote + local datasources.
**Datasources** : appels API Dio/http, requêtes Drift/Hive.

### `presentation/` — L'UI

**BLoC** sépare events → states :
```dart
// Event
class LoadProducts extends ProductEvent {}

// State
class ProductsLoaded extends ProductState {
  final List<Product> products;
}

// BLoC
class ProductBloc extends Bloc<ProductEvent, ProductState> {
  ProductBloc(this._getProducts) : super(ProductInitial()) {
    on<LoadProducts>((event, emit) async {
      emit(ProductsLoading());
      final result = await _getProducts(NoParams());
      result.fold(
        (failure) => emit(ProductsError(failure.message)),
        (products) => emit(ProductsLoaded(products)),
      );
    });
  }
}
```

---

## Either<Failure, T>

Utilisé avec `dartz`. Remplace les exceptions par des types explicites.
La fonction **ne peut pas** retourner sans gérer les deux cas.

```dart
final result = await getProducts(NoParams());
result.fold(
  (failure) => showError(failure.message),
  (products) => showProducts(products),
);
```

---

## Options configurables à la génération

### Injection de dépendances
| Choix | Description |
|-------|-------------|
| get_it | Locator de services manuel, simple et rapide |
| injectable | get_it + génération de code via annotations |

### Client HTTP
| Choix | Usage |
|-------|-------|
| Aucun | — |
| Dio | Interceptors, retry, multipart, cancel tokens |
| http | Package officiel, minimaliste |

### Base de données locale
| Choix | Usage |
|-------|-------|
| Aucun | — |
| Drift | SQL typé avec génération de code, le plus robuste |
| Hive | NoSQL clé-valeur rapide, sans setup SQL |
| SharedPreferences | Stockage simple clé-valeur (préférences user) |

---

## Quand utiliser

- Applications en production avec durée de vie longue (2+ ans)
- Équipes multiples travaillant sur des features différentes
- Projets avec des règles métier complexes et évolutives
- Quand la testabilité est une priorité (chaque couche est mockable)
- Applications qui peuvent changer de backend ou de base de données

## Quand NE PAS utiliser

- MVP ou prototype — le rapport code/fonctionnalité est défavorable
- Équipes débutantes sur Flutter — commencez par CWA Feature-First
- Applications simples avec peu de logique métier
- Quand la vitesse de livraison prime sur la maintenabilité

---

## Ressources

- https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html
- https://bloclibrary.dev
- https://pub.dev/packages/dartz
- https://pub.dev/packages/get_it
- https://pub.dev/packages/injectable
- https://drift.simonbinder.eu
