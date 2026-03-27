import 'package:livre/src/exceptions/app_exception_code.dart';

sealed class AppException implements Exception {
  AppException(this.code, this.message);

  final AppExceptionCode code;
  final String message;

  @override
  String toString() => 'AppException($code): $message';
}

class NetworkException extends AppException {
  NetworkException([String message = 'Network error'])
      : super(AppExceptionCode.network, message);
}

class DataNotFoundException extends AppException {
  DataNotFoundException([String message = 'Data not found'])
      : super(AppExceptionCode.dataNotFound, message);
}
