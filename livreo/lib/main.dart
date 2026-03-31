import 'dart:async';

import 'package:firebase_core/firebase_core.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:livreo/src/livreo_app.dart';
import 'package:livreo/src/config/app_env.dart';
import 'package:livreo/src/config/app_flavor.dart';

Future<void> runMainApp(AppFlavor flavor) async {
  WidgetsFlutterBinding.ensureInitialized();

  final env = AppEnv.fromFlavor(flavor);

  await SystemChrome.setPreferredOrientations([
    DeviceOrientation.portraitUp,
    DeviceOrientation.portraitDown,
  ]);

  await Firebase.initializeApp();

  final container = ProviderContainer();

  runApp(
    UncontrolledProviderScope(
      container: container,
      child: LivreoApp(flavor: flavor),
    ),
  );
}