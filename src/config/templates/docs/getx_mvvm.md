# Feature-First MVVM + GetX

---

## Motivation

GetX est un framework tout-en-un (state, routing, DI) très populaire dans les
équipes qui veulent aller vite avec peu de boilerplate. L'associer au pattern
MVVM donne une séparation claire entre la logique (Controller/ViewModel) et
l'affichage (View), sans la complexité de Clean Architecture.

Ce template est adapté aux équipes qui connaissent GetX et veulent une structure
cohérente entre les features sans surcharger le projet.

---

## Vue d'ensemble

```
lib/
├── main.dart
├── app/
│   ├── app.dart             ← GetMaterialApp + config globale
│   ├── bindings/
│   │   └── initial_binding.dart   ← Services injectés au démarrage
│   ├── routes/
│   │   ├── app_pages.dart   ← Mapping route → page + binding
│   │   └── app_routes.dart  ← Constantes des noms de routes
│   └── themes/
│       └── app_theme.dart
├── core/
│   ├── constants/
│   ├── errors/
│   ├── extensions/
│   ├── network/             ← (si Dio ou http activé)
│   ├── storage/             ← (si get_storage ou Hive activé)
│   ├── utils/
│   └── widgets/             ← Widgets partagés (loading, error)
└── features/
    └── {feature}/
        ├── bindings/        ← Injection des dépendances de la feature
        ├── controllers/     ← Logique métier + state (GetxController)
        ├── models/          ← Modèles de données
        ├── repositories/    ← Accès données (API, local)
        └── views/
            ├── {feature}_view.dart
            └── widgets/     ← Widgets spécifiques à la feature
```

---

## Pattern MVVM avec GetX

```
View  ──observe──▶  Controller  ──appelle──▶  Repository
 (Widget)           (GetxController)           (Data source)
```

### `Controller` (ViewModel)
Hérite de `GetxController`. Contient les variables observables (`.obs`)
et la logique métier. La View ne fait qu'observer et appeler des méthodes.

```dart
class AuthController extends GetxController {
  final _isLoading = false.obs;
  bool get isLoading => _isLoading.value;

  Future<void> login(String email, String password) async {
    _isLoading.value = true;
    await _repository.login(email, password);
    _isLoading.value = false;
    Get.offAllNamed(AppRoutes.home);
  }
}
```

### `Binding`
Instancie et injecte les dépendances de la feature via `Get.lazyPut`.
Appelé automatiquement par GetX avant d'afficher la page.

```dart
class AuthBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<AuthRepository>(() => AuthRepositoryImpl());
    Get.lazyPut<AuthController>(() => AuthController(Get.find()));
  }
}
```

---

## Options configurables à la génération

### Stockage local
| Choix | Usage |
|-------|-------|
| Aucun | — |
| get_storage | Stockage clé-valeur léger (JSON), idéal pour préférences |
| Hive | Base NoSQL locale plus robuste pour données structurées |

### Client HTTP
| Choix | Usage |
|-------|-------|
| Aucun | — |
| Dio | Client HTTP avancé (interceptors, retry, multipart) |
| http | Package officiel Flutter, minimaliste |

---

## Quand utiliser

- Équipe déjà familière avec GetX
- Applications mid-size avec features bien délimitées
- Quand on veut minimiser le boilerplate sans sacrifier la structure
- Prototypes qui ont vocation à évoluer en production

## Quand NE PAS utiliser

- Grandes applications avec des règles métier complexes (préférez Clean Architecture + BLoC)
- Équipes qui veulent éviter GetX (couplage fort au framework)
- Si vous avez besoin d'une testabilité maximale (GetX est moins testable que Riverpod/BLoC)

---

## Ressources

- https://pub.dev/packages/get
- https://github.com/jonataslaw/getx/blob/master/documentation/en_US/route_management.md
