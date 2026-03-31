import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:livreo/src/features/home/data/home_repository.dart';
import 'package:livreo/src/features/home/domain/home_model.dart';

part 'home_service.g.dart';

class HomeService {
  HomeService(this._repository);

  final HomeRepository _repository;

  Future<List<Home>> getAll() => _repository.fetchAll();

  Future<Home?> getById(String id) => _repository.fetchById(id);

  Future<void> create(Home item) => _repository.create(item);

  Future<void> update(Home item) => _repository.update(item);

  Future<void> delete(String id) => _repository.delete(id);
}

@riverpod
HomeService homeService(Ref ref) {
  return HomeService(ref.watch(homeRepositoryProvider));
}