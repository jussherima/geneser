import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class Not_foundScreen extends ConsumerWidget {
  const Not_foundScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Not_found'),
      ),
      body: const Center(
        child: Text('Welcome to Not_found screen'),
      ),
    );
  }
}
