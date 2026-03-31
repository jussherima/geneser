import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:livreo/src/features/authentication/application/authentication_service.dart';
import 'package:livreo/src/features/authentication/domain/authentication_model.dart';

part 'authentication_controller.g.dart';

@riverpod
class AuthenticationController extends _$AuthenticationController {
  @override
  FutureOr<List<Authentication>> build() async {
    return ref.watch(authenticationServiceProvider).getAll();
  }

  Future<void> create(Authentication item) async {
    state = const AsyncValue.loading();
    state = await AsyncValue.guard(() async {
      await ref.read(authenticationServiceProvider).create(item);
      return ref.read(authenticationServiceProvider).getAll();
    });
  }

  Future<void> delete(String id) async {
    state = const AsyncValue.loading();
    state = await AsyncValue.guard(() async {
      await ref.read(authenticationServiceProvider).delete(id);
      return ref.read(authenticationServiceProvider).getAll();
    });
  }
}