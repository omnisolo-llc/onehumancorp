import 'dart:convert';

import 'package:yaml/yaml.dart';

/// Converts a JSON-compatible Dart object to a YAML string.
///
/// The output uses the block-style YAML that is most readable for humans.
String modelToYaml(Object? value, [int depth = 0]) {
  if (value == null) return 'null\n';
  if (value is bool || value is num) return '$value\n';
  if (value is String) {
    if (value.isEmpty ||
        value.contains(':') ||
        value.contains('#') ||
        value.contains('\n') ||
        value.startsWith(' ') ||
        value.startsWith('-') ||
        value.startsWith("'")) {
      final escaped = value.replaceAll(r'\', r'\\').replaceAll('"', r'\"');
      return '"$escaped"\n';
    }
    return '$value\n';
  }
  if (value is List) {
    if (value.isEmpty) return '[]\n';
    final indent = '  ' * depth;
    final buf = StringBuffer('\n');
    for (final item in value) {
      buf.write('$indent- ${modelToYaml(item, depth + 1)}');
    }
    return buf.toString();
  }
  if (value is Map) {
    if (value.isEmpty) return '{}\n';
    final indent = '  ' * depth;
    final buf = StringBuffer('\n');
    for (final entry in value.entries) {
      buf.write('$indent${entry.key}: ${modelToYaml(entry.value, depth + 1)}');
    }
    return buf.toString();
  }
  return '$value\n';
}

/// Parses a YAML string and returns a [Map<String, dynamic>] suitable for
/// passing to a model's [fromJson] factory.
///
/// Returns an empty map if [yaml] is null or contains only whitespace.
Map<String, dynamic> modelFromYaml(String yaml) {
  if (yaml.trim().isEmpty) return {};
  final doc = loadYaml(yaml);
  if (doc == null) return {};
  // Round-trip through JSON to normalise YamlMap/YamlList → plain Map/List.
  return jsonDecode(jsonEncode(doc)) as Map<String, dynamic>;
}
