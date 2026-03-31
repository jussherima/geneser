import 'package:equatable/equatable.dart';

class Authentication extends Equatable {
  const Authentication({
    required this.id,
    required this.name,
    this.createdAt,
  });

  final String id;
  final String name;
  final DateTime? createdAt;

  Authentication copyWith({
    String? id,
    String? name,
    DateTime? createdAt,
  }) {
    return Authentication(
      id: id ?? this.id,
      name: name ?? this.name,
      createdAt: createdAt ?? this.createdAt,
    );
  }

  Map<String, dynamic> toMap() {
    return {
      'id': id,
      'name': name,
      if (createdAt != null) 'createdAt': createdAt!.toIso8601String(),
    };
  }

  factory Authentication.fromMap(Map<String, dynamic> map, [String? docId]) {
    return Authentication(
      id: docId ?? map['id'] as String,
      name: map['name'] as String,
      createdAt: map['createdAt'] != null
          ? DateTime.parse(map['createdAt'] as String)
          : null,
    );
  }

  @override
  List<Object?> get props => [id, name, createdAt];
}