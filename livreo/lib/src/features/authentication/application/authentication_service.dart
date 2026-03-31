import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:livreo/src/features/authentication/data/authentication_repository.dart';
import 'package:livreo/src/features/authentication/domain/authentication_model.dart';

part 'authentication_service.g.dart';

class AuthenticationService {
  AuthenticationService(this._repository);

  final AuthenticationRepository _repository;

  Future<List<Authentication>> getAll() => _repository.fetchAll();

  Future<Authentication?> getById(String id) => _repository.fetchById(id);

  Future<void> create(Authentication item) => _repository.create(item);

  Future<void> update(Authentication item) => _repository.update(item);

  Future<void> delete(String id) => _repository.delete(id);
}

@riverpod
AuthenticationService authenticationService(Ref ref) {
  return AuthenticationService(ref.watch(authenticationRepositoryProvider));
}