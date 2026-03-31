import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:livreo/src/localization/localization.dart';
import 'package:livreo/src/config/app_flavor.dart';
import 'package:livreo/src/routing/app_router.dart';
import 'package:livreo/src/routing/initialization/app_startup_widget_wrapper.dart';

class LivreoApp extends ConsumerWidget {
  const LivreoApp({required this.flavor, super.key});

  final AppFlavor flavor;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final appRouter = ref.watch(appRouterProvider);

    return MaterialApp.router(
      routerConfig: appRouter,
      title: 'Livreo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.blue),
        useMaterial3: true,
      ),
      darkTheme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.blue,
          brightness: Brightness.dark,
        ),
        useMaterial3: true,
      ),
      localizationsDelegates: const [
        AppLocalizations.delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
      ],
      supportedLocales: SupportedLocale.values.map((e) => e.locale),
      restorationScopeId: 'app',
      builder: (context, child) {
        return Banner(
          message: flavor.name.toUpperCase(),
          location: BannerLocation.topStart,
          color: flavor == AppFlavor.prod ? Colors.transparent : Colors.red,
          child: AppStartupWidgetWrapper(
            onLoaded: (_) => child ?? const SizedBox.shrink(),
          ),
        );
      },
    );
  }
}