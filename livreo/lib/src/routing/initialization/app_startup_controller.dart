import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'app_startup_controller.g.dart';

@riverpod
class AppStartupController extends _$AppStartupController {
  @override
  FutureOr<void> build() async {
    await _initialize();
  }

  Future<void> _initialize() async {
    try {
      // Add your async initialization here
      // e.g. local storage, remote config, etc.
      await Future<void>.delayed(const Duration(milliseconds: 100));
    } on Exception catch (e, st) {
      state = AsyncError(e, st);
    }
  }

  Future<void> retry() async {
    state = const AsyncValue.loading();
    state = await AsyncValue.guard(_initialize);
  }
}