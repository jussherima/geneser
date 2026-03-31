import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:livreo/src/features/authentication/domain/authentication_model.dart';

part 'authentication_repository.g.dart';

class AuthenticationRepository {
  AuthenticationRepository(this._firestore);

  final FirebaseFirestore _firestore;

  CollectionReference<Map<String, dynamic>> get _collection =>
      _firestore.collection('authentications');

  Future<List<Authentication>> fetchAll() async {
    final snapshot = await _collection.get();
    return snapshot.docs
        .map((doc) => Authentication.fromMap(doc.data(), doc.id))
        .toList();
  }

  Future<Authentication?> fetchById(String id) async {
    final doc = await _collection.doc(id).get();
    if (!doc.exists) return null;
    return Authentication.fromMap(doc.data()!, doc.id);
  }

  Future<void> create(Authentication item) async {
    await _collection.doc(item.id).set(item.toMap());
  }

  Future<void> update(Authentication item) async {
    await _collection.doc(item.id).update(item.toMap());
  }

  Future<void> delete(String id) async {
    await _collection.doc(id).delete();
  }
}

@riverpod
AuthenticationRepository authenticationRepository(Ref ref) {
  return AuthenticationRepository(FirebaseFirestore.instance);
}