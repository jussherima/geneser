import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'authentication_service.g.dart';

class AuthenticationService {
  const AuthenticationService();

  // TODO: Implement service logic
}

@riverpod
AuthenticationService authenticationService(AuthenticationServiceRef ref) {
  return const AuthenticationService();
}
