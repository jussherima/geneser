import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:livre/src/routing/initialization/app_startup_controller.dart';

/// Widget class to manage asynchronous app initialization.
class AppStartupWidgetWrapper extends ConsumerWidget {
  const AppStartupWidgetWrapper({required this.onLoaded, super.key});
  final WidgetBuilder onLoaded;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final appStartupState = ref.watch(appStartupControllerProvider);
    return appStartupState.when(
      data: (_) => onLoaded(context),
      loading: () => const Scaffold(
        body: Center(child: CircularProgressIndicator()),
      ),
      error: (e, st) => Scaffold(
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text('Initialization error: $e'),
              const SizedBox(height: 16),
              ElevatedButton(
                onPressed: () =>
                    ref.read(appStartupControllerProvider.notifier).retry(),
                child: const Text('Retry'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}