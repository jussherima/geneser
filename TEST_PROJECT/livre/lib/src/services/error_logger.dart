import 'dart:async';
import 'dart:developer';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'error_logger.g.dart';

class ErrorLogger {
  const ErrorLogger();

  FutureOr<void> logException(Object exception, StackTrace? stackTrace) async {
    log(
      exception.toString(),
      name: 'Exception',
      error: exception,
      stackTrace: stackTrace,
    );
  }
}

@Riverpod(keepAlive: true)
ErrorLogger errorLogger(Ref _) => const ErrorLogger();