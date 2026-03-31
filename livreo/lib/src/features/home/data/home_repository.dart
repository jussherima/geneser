import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:livreo/src/features/home/domain/home_model.dart';

part 'home_repository.g.dart';

class HomeRepository {
  HomeRepository(this._firestore);

  final FirebaseFirestore _firestore;

  CollectionReference<Map<String, dynamic>> get _collection =>
      _firestore.collection('homes');

  Future<List<Home>> fetchAll() async {
    final snapshot = await _collection.get();
    return snapshot.docs
        .map((doc) => Home.fromMap(doc.data(), doc.id))
        .toList();
  }

  Future<Home?> fetchById(String id) async {
    final doc = await _collection.doc(id).get();
    if (!doc.exists) return null;
    return Home.fromMap(doc.data()!, doc.id);
  }

  Future<void> create(Home item) async {
    await _collection.doc(item.id).set(item.toMap());
  }

  Future<void> update(Home item) async {
    await _collection.doc(item.id).update(item.toMap());
  }

  Future<void> delete(String id) async {
    await _collection.doc(id).delete();
  }
}

@riverpod
HomeRepository homeRepository(Ref ref) {
  return HomeRepository(FirebaseFirestore.instance);
}