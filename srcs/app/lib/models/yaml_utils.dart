import 'dart:convert';

/// Converts a JSON-compatible Dart object to a YAML string.
///
/// The output uses the block-style YAML that is most readable for humans.
/// The resulting YAML is also valid JSON (since block YAML is a superset of
/// JSON), which means it round-trips through [modelFromYaml].
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
/// Uses [dart:convert] only (no external package:yaml dep). Handles JSON
/// (fast path) and simple flat key: value block-YAML (slow path).
///
/// Returns an empty map if [yaml] is null or contains only whitespace.
Map<String, dynamic> modelFromYaml(String yaml) {
  final trimmed = yaml.trim();
  if (trimmed.isEmpty) return {};
  try {
    final decoded = jsonDecode(trimmed);
    if (decoded is Map<String, dynamic>) return decoded;
    return {};
  } catch (_) {
    return _parseSimpleBlockYaml(trimmed);
  }
}

/// Minimal block-YAML → Map parser for simple flat documents.
Map<String, dynamic> _parseSimpleBlockYaml(String yaml) {
  final result = <String, dynamic>{};
  for (final line in yaml.split('\n')) {
    final trimmed = line.trim();
    if (trimmed.isEmpty || trimmed.startsWith('#')) continue;
    final colonIdx = trimmed.indexOf(':');
    if (colonIdx < 0) continue;
    final key = trimmed.substring(0, colonIdx).trim();
    final rawValue = trimmed.substring(colonIdx + 1).trim();
    if (key.isEmpty) continue;
    result[key] = _parseScalarYaml(rawValue);
  }
  return result;
}

dynamic _parseScalarYaml(String raw) {
  if (raw == 'null' || raw == '~') return null;
  if (raw == 'true') return true;
  if (raw == 'false') return false;
  final n = double.tryParse(raw);
  if (n != null) return n % 1 == 0 ? n.toInt() : n;
  if (raw.length >= 2) {
    if (raw.startsWith('"') && raw.endsWith('"')) {
      return raw.substring(1, raw.length - 1).replaceAll(r'\"', '"');
    }
    if (raw.startsWith("'") && raw.endsWith("'")) {
      return raw.substring(1, raw.length - 1);
    }
  }
  return raw;
}
