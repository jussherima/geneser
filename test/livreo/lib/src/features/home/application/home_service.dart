import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'home_service.g.dart';

class HomeService {
  const HomeService();

  // TODO: Implement service logic
}

@riverpod
HomeService homeService(HomeServiceRef ref) {
  return const HomeService();
}
