import 'package:equatable/equatable.dart';

class Home extends Equatable {
  const Home({
    required this.id,
    required this.name,
    this.createdAt,
  });

  final String id;
  final String name;
  final DateTime? createdAt;

  Home copyWith({
    String? id,
    String? name,
    DateTime? createdAt,
  }) {
    return Home(
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

  factory Home.fromMap(Map<String, dynamic> map, [String? docId]) {
    return Home(
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