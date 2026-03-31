import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';

extension RefExtension<T> on Ref<T> {
  /// Keeps the provider alive for the given [duration].
  void cacheDataFor(Duration duration) {
    final link = keepAlive();
    Timer? timer;

    onDispose(() => timer?.cancel());

    onCancel(() {
      timer = Timer(duration, link.close);
    });

    onResume(() {
      timer?.cancel();
    });
  }
}