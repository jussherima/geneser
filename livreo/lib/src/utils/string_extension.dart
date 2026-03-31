extension StringExtension on String {
  /// Converts this string into a route parameter pattern (e.g. `:id`).
  String get routePattern => ':$this';

  /// Capitalizes the first letter.
  String get capitalize => '${this[0].toUpperCase()}${substring(1)}';

  /// Builds a query string from the given map.
  String buildQueryParams(Map<String, dynamic> queryParams) {
    final filtered = Map.fromEntries(
      queryParams.entries.where((e) => e.value != null),
    );
    if (filtered.isEmpty) return this;
    final query = filtered.entries
        .map((e) => '${e.key}=${Uri.encodeComponent(e.value.toString())}')
        .join('&');
    return '$this?$query';
  }
}