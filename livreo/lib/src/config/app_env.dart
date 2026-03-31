import 'package:livreo/src/config/app_env_fields.dart';
import 'package:livreo/src/config/app_flavor.dart';

class AppEnv implements AppEnvFields {
  const AppEnv._({
    required this.apiBaseUrl,
  });

  factory AppEnv.fromFlavor(AppFlavor flavor) {
    switch (flavor) {
      case AppFlavor.dev:
        return const AppEnv._(
          apiBaseUrl: 'https://api-dev.example.com',
        );
      case AppFlavor.stg:
        return const AppEnv._(
          apiBaseUrl: 'https://api-stg.example.com',
        );
      case AppFlavor.prod:
        return const AppEnv._(
          apiBaseUrl: 'https://api.example.com',
        );
    }
  }

  @override
  final String apiBaseUrl;

}