# Riverpod + Freezed (Minimal)

---

## Motivation

Les templates CWA Medium et Clean Architecture sont excellents mais imposants.
Ce template prend le parti inverse : **le minimum viable qui reste maintenable**.

Riverpod + Freezed couvrent 90% des besoins d'une application Flutter moderne
avec très peu de setup. Freezed génère les modèles immuables, Riverpod gère
le state avec une API cohérente, go_router s'occupe de la navigation déclarative.

Idéal quand vous voulez de la structure sans la cérémonie de Clean Architecture.

---

## Vue d'ensemble

```
lib/
├── main.dart
└── src/
    ├── app/
    │   └── app.dart           ← ProviderScope + MaterialApp.router
    ├── routing/
    │   ├── app_router.dart    ← GoRouter avec routes typées
    │   └── app_router.g.dart  ← Généré par build_runner
    ├── common_widgets/
    │   └── async_value_widget.dart
    ├── constants/
    ├── exceptions/
    │   └── app_exception.dart
    ├── network/               ← (si Dio ou http activé)
    ├── storage/               ← (si SharedPrefs ou Hive activé)
    ├── utils/
    └── features/
        └── {feature}/
            ├── presentation/  ← Écrans et widgets
            ├── providers/     ← Providers Riverpod (state + logique)
            ├── models/        ← Modèles Freezed
            └── repositories/  ← Accès données
```

---

## Couches

### `presentation/`
Widgets ConsumerWidget qui observent les providers.
Aucune logique métier — uniquement de l'affichage et des interactions.

```dart
class HomeScreen extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final products = ref.watch(productsProvider);
    return products.when(
      data: (list) => ProductList(list),
      loading: () => const CircularProgressIndicator(),
      error: (e, _) => ErrorWidget(e.toString()),
    );
  }
}
```

### `providers/`
Les providers Riverpod encapsulent la logique et exposent l'AsyncValue.
Générés avec `@riverpod` (riverpod_generator).

```dart
@riverpod
Future<List<Product>> products(ProductsRef ref) =>
    ref.watch(productsRepositoryProvider).fetchAll();
```

### `models/`
Classes Freezed — immuables, avec `copyWith`, `==`, `hashCode` et
sérialisation JSON générés automatiquement.

```dart
@freezed
class Product with _$Product {
  const factory Product({
    required String id,
    required String name,
    required double price,
  }) = _Product;

  factory Product.fromJson(Map<String, dynamic> json) => _$ProductFromJson(json);
}
```

### `repositories/`
Accès aux données (API, cache local). Exposés via un provider pour
l'injection de dépendances sans container externe.

```dart
@riverpod
ProductsRepository productsRepository(ProductsRepositoryRef ref) =>
    ProductsRepositoryImpl(baseUrl: '...');
```

---

## `async_value_widget.dart`

Utilitaire pour afficher un `AsyncValue<T>` sans répéter `.when(...)` partout.

```dart
AsyncValueWidget<List<Product>>(
  value: ref.watch(productsProvider),
  data: (products) => ProductList(products),
)
```

---

## Options configurables à la génération

### Client HTTP
| Choix | Usage |
|-------|-------|
| Aucun | — |
| Dio | Interceptors, headers globaux, gestion d'erreurs centralisée |
| http | Package officiel, pour des besoins simples |

### Persistance locale
| Choix | Usage |
|-------|-------|
| Aucun | — |
| SharedPreferences | Préférences utilisateur, flags simples |
| Hive | Données structurées, plus rapide que SharedPreferences |

---

## Différence avec CWA Feature-First

| | CWA Feature-First | Riverpod Minimal |
|-|-------------------|-----------------|
| Couches par feature | 4 (pres/app/domain/data) | 4 (pres/providers/models/data) |
| Codegen | riverpod_generator | riverpod_generator + freezed |
| Modèles | Manuel | Freezed (immuables + JSON auto) |
| Routing | go_router | go_router typé (génération de code) |
| Orientation | Architecturale | Pragmatique |

---

## Quand utiliser

- Applications de taille petite à moyenne
- Développeur solo ou petite équipe (1-3 personnes)
- Quand vous voulez Riverpod + Freezed sans Clean Architecture complète
- Bon point d'entrée avant de migrer vers CWA Medium

## Quand NE PAS utiliser

- Applications avec règles métier très complexes (préférez Clean Architecture)
- Grandes équipes qui ont besoin de contrats stricts entre couches
- Si vous n'êtes pas à l'aise avec la génération de code (build_runner)

---

## Ressources

- https://riverpod.dev/docs/from_provider/motivation
- https://pub.dev/packages/freezed
- https://pub.dev/packages/go_router
- https://codewithandrea.com/articles/flutter-riverpod-data-caching-providers
