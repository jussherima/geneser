# CodeWithAndrea — Feature-First

Auteur : Andrea Bizzotto (codewithAndrea.com) — adapté par Geneser

---

## Motivation

Cette architecture répond à un problème classique : les projets Flutter qui grandissent
deviennent difficiles à naviguer quand le code est organisé par type technique
(tous les widgets ensemble, tous les models ensemble, etc.).

L'approche Feature-First regroupe **tout ce qui concerne une feature au même endroit**,
ce qui réduit le saut entre fichiers quand on travaille sur une fonctionnalité.

---

## Vue d'ensemble

```
lib/
├── main.dart
└── src/
    ├── features/
    │   └── {feature}/
    │       ├── presentation/    ← Widgets, écrans
    │       ├── application/     ← Services, use cases
    │       ├── domain/          ← Modèles, entités
    │       └── data/            ← Repositories, datasources
    ├── common_widgets/
    ├── constants/
    ├── exceptions/
    ├── localization/
    ├── routing/
    └── utils/
```

---

## Couches

### `presentation/`
Widgets Flutter et écrans. Lit le state depuis Riverpod, dispatche des actions.
Aucune logique métier ici — uniquement de l'UI.

### `application/`
Services et use cases. Orchestre les appels entre domain et data.
Contient les providers Riverpod qui exposent le state à la couche présentation.

### `domain/`
Modèles de données purs Dart (aucune dépendance Flutter ou package externe).
Entités, value objects, interfaces de repositories.

### `data/`
Implémentation concrète des repositories.
Appels API, base de données locale, cache.

---

## Quand utiliser

- Applications avec 5 à 30 features distinctes
- Équipes de 2 à 10 développeurs travaillant en parallèle sur des features différentes
- Projets qui ont besoin d'une bonne testabilité feature par feature
- Quand vous voulez pouvoir supprimer ou isoler une feature facilement

## Quand NE PAS utiliser

- Prototypes ou MVPs (trop de structure pour si peu de code)
- Applications avec une seule feature centrale
- Équipes qui débutent sur Riverpod (préférez d'abord apprendre Riverpod seul)

---

## State management : Riverpod

Riverpod est utilisé à travers toutes les couches via des providers.
La génération de code (`riverpod_generator`) réduit le boilerplate.

```dart
// Exemple dans application/
@riverpod
Future<List<Product>> products(ProductsRef ref) async {
  return ref.watch(productsRepositoryProvider).fetchAll();
}
```

---

## Ressources

- https://www.codewithAndrea.com
- https://riverpod.dev
- https://pub.dev/packages/go_router
