enum AppFlavor {
  dev(
    packageName: 'com.example.livreo.dev',
    appName: 'Livreo Dev',
  ),
  stg(
    packageName: 'com.example.livreo.stg',
    appName: 'Livreo Stg',
  ),
  prod(
    packageName: 'com.example.livreo',
    appName: 'Livreo',
  );

  const AppFlavor({
    required this.packageName,
    required this.appName,
  });

  final String packageName;
  final String appName;

  bool get isDev => this == AppFlavor.dev;
  bool get isStg => this == AppFlavor.stg;
  bool get isProd => this == AppFlavor.prod;
}