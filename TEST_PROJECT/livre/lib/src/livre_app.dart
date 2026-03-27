import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:livre/src/routing/app_router.dart';
import 'package:livre/src/routing/initialization/app_startup_widget_wrapper.dart';

class LivreApp extends ConsumerWidget {
  const LivreApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final appRouter = ref.watch(appRouterProvider);

    return MaterialApp.router(
      routerConfig: appRouter,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.blue),
        useMaterial3: true,
      ),
      restorationScopeId: 'app',
      builder: (context, child) {
        return AppStartupWidgetWrapper(
          onLoaded: (_) => child ?? const SizedBox.shrink(),
        );
      },
    );
  }
}