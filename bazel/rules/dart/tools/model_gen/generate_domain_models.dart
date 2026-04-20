// Proto Domain Model Generator
// Parses a .proto file and generates a comprehensive Dart domain model file.
// Generated models include: fromProto/toProto, fromJson/toJson, fromYaml/toYaml,
// copyWith, ==, hashCode, toString, and gRPC client wrappers per service.
//
// Usage: generate_domain_models.dart <input.proto> <output.domain.dart>

// ignore_for_file: avoid_print

import 'dart:io';

// ─────────────────────────── Entry point ────────────────────────────────────

void main(List<String> args) {
  if (args.length < 2) {
    stderr.writeln(
      'Usage: generate_domain_models.dart <input.proto> <output.domain.dart>',
    );
    exit(1);
  }

  final inputPath = args[0];
  final outputPath = args[1];

  final inputFile = File(inputPath);
  if (!inputFile.existsSync()) {
    stderr.writeln('Input file $inputPath not found');
    exit(1);
  }

  final content = inputFile.readAsStringSync();
  final parsed = _parseProto(content);
  final code = _generateCode(parsed, inputPath);
  File(outputPath).writeAsStringSync(code);
}

// ─────────────────────────── AST ────────────────────────────────────────────

class _ProtoFile {
  String package = '';
  List<String> imports = [];
  List<_Message> messages = [];
  List<_Enum> enums = [];
  List<_Service> services = [];
}

class _Field {
  final String type; // proto type (e.g. "string", "google.protobuf.Timestamp", "Agent")
  final String name; // proto field name (snake_case)
  final bool repeated;
  final bool isMap;
  final String mapKeyType; // only for map fields
  final String mapValueType; // only for map fields
  final bool optional;

  _Field({
    required this.type,
    required this.name,
    this.repeated = false,
    this.isMap = false,
    this.mapKeyType = '',
    this.mapValueType = '',
    this.optional = false,
  });
}

class _Message {
  final String name;
  final List<_Field> fields;
  final List<_Enum> nestedEnums;
  final List<_Message> nestedMessages;

  _Message(this.name, this.fields, this.nestedEnums, this.nestedMessages);
}

class _EnumValue {
  final String name;
  final int number;

  _EnumValue(this.name, this.number);
}

class _Enum {
  final String name;
  final List<_EnumValue> values;

  _Enum(this.name, this.values);
}

class _Rpc {
  final String name;
  final String inputType;
  final bool inputStreaming;
  final String outputType;
  final bool outputStreaming;

  _Rpc(
    this.name,
    this.inputType,
    this.inputStreaming,
    this.outputType,
    this.outputStreaming,
  );
}

class _Service {
  final String name;
  final List<_Rpc> rpcs;

  _Service(this.name, this.rpcs);
}

// ─────────────────────────── Parser ─────────────────────────────────────────

_ProtoFile _parseProto(String content) {
  final result = _ProtoFile();
  final cleaned = _stripComments(content);
  final tokens = _tokenize(cleaned);
  _parseTokens(tokens, result);
  return result;
}

/// Remove block comments and line comments.
String _stripComments(String src) {
  final buf = StringBuffer();
  var i = 0;
  while (i < src.length) {
    if (i + 1 < src.length && src[i] == '/' && src[i + 1] == '*') {
      // Block comment
      i += 2;
      while (i + 1 < src.length && !(src[i] == '*' && src[i + 1] == '/')) {
        if (src[i] == '\n') buf.write('\n');
        i++;
      }
      i += 2;
    } else if (i + 1 < src.length && src[i] == '/' && src[i + 1] == '/') {
      // Line comment
      while (i < src.length && src[i] != '\n') {
        i++;
      }
    } else {
      buf.write(src[i]);
      i++;
    }
  }
  return buf.toString();
}

List<String> _tokenize(String src) {
  // Split into tokens: identifiers, numbers, punctuation, string literals
  final tokens = <String>[];
  var i = 0;
  while (i < src.length) {
    final c = src[i];
    if (c == ' ' || c == '\t' || c == '\n' || c == '\r') {
      i++;
      continue;
    }
    if (c == '"' || c == "'") {
      // String literal
      final quote = c;
      final sb = StringBuffer(c);
      i++;
      while (i < src.length && src[i] != quote) {
        sb.write(src[i]);
        i++;
      }
      sb.write(quote);
      i++;
      tokens.add(sb.toString());
      continue;
    }
    if (_isAlphaNum(c) || c == '_' || c == '.') {
      final sb = StringBuffer();
      while (
        i < src.length && (_isAlphaNum(src[i]) || src[i] == '_' || src[i] == '.')
      ) {
        sb.write(src[i]);
        i++;
      }
      tokens.add(sb.toString());
      continue;
    }
    // Single-char punctuation
    tokens.add(c);
    i++;
  }
  return tokens;
}

bool _isAlphaNum(String c) {
  final code = c.codeUnitAt(0);
  return (code >= 65 && code <= 90) || // A-Z
      (code >= 97 && code <= 122) || // a-z
      (code >= 48 && code <= 57); // 0-9
}

void _parseTokens(List<String> tokens, _ProtoFile result) {
  var i = 0;
  while (i < tokens.length) {
    final t = tokens[i];
    if (t == 'syntax' || t == 'edition') {
      // syntax = "proto3"; or edition = "2024";
      while (i < tokens.length && tokens[i] != ';') {
        i++;
      }
      i++;
    } else if (t == 'package') {
      i++;
      result.package = tokens[i];
      i++;
      // skip semicolon
      if (i < tokens.length && tokens[i] == ';') i++;
    } else if (t == 'import') {
      i++;
      // skip "public"/"weak" if present
      if (tokens[i] == 'public' || tokens[i] == 'weak') i++;
      var imp = tokens[i];
      if (imp.startsWith('"') || imp.startsWith("'")) {
        imp = imp.substring(1, imp.length - 1);
      }
      result.imports.add(imp);
      i++;
      if (i < tokens.length && tokens[i] == ';') i++;
    } else if (t == 'option') {
      // skip option lines
      while (i < tokens.length && tokens[i] != ';') i++;
      i++;
    } else if (t == 'message') {
      i++;
      final r = _parseMessage(tokens, i);
      result.messages.add(r.$1);
      i = r.$2;
    } else if (t == 'enum') {
      i++;
      final r = _parseEnum(tokens, i);
      result.enums.add(r.$1);
      i = r.$2;
    } else if (t == 'service') {
      i++;
      final r = _parseService(tokens, i);
      result.services.add(r.$1);
      i = r.$2;
    } else {
      i++;
    }
  }
}

(
  _Message,
  int,
)
_parseMessage(List<String> tokens, int start) {
  final name = tokens[start];
  start++;
  // expect '{'
  if (start < tokens.length && tokens[start] == '{') start++;

  final fields = <_Field>[];
  final nestedEnums = <_Enum>[];
  final nestedMessages = <_Message>[];

  while (start < tokens.length && tokens[start] != '}') {
    final t = tokens[start];
    if (t == 'message') {
      start++;
      final r = _parseMessage(tokens, start);
      nestedMessages.add(r.$1);
      start = r.$2;
    } else if (t == 'enum') {
      start++;
      final r = _parseEnum(tokens, start);
      nestedEnums.add(r.$1);
      start = r.$2;
    } else if (t == 'option' || t == 'reserved' || t == 'extensions') {
      while (start < tokens.length && tokens[start] != ';') start++;
      start++;
    } else if (t == 'oneof') {
      // Parse oneof as individual optional fields
      start++; // name
      start++; // skip name
      // expect '{'
      if (start < tokens.length && tokens[start] == '{') start++;
      while (start < tokens.length && tokens[start] != '}') {
        if (tokens[start] == 'option') {
          while (start < tokens.length && tokens[start] != ';') start++;
          start++;
        } else {
          final fr = _parseField(tokens, start, optional: true);
          if (fr.$1 != null) fields.add(fr.$1!);
          start = fr.$2;
        }
      }
      if (start < tokens.length && tokens[start] == '}') start++;
    } else if (t == 'repeated') {
      final fr = _parseField(tokens, start, optional: false);
      if (fr.$1 != null) fields.add(fr.$1!);
      start = fr.$2;
    } else if (t == 'map') {
      final fr = _parseMapField(tokens, start);
      if (fr.$1 != null) fields.add(fr.$1!);
      start = fr.$2;
    } else if (t == 'optional') {
      final fr = _parseField(tokens, start, optional: true);
      if (fr.$1 != null) fields.add(fr.$1!);
      start = fr.$2;
    } else if (t == 'required') {
      final fr = _parseField(tokens, start, optional: false);
      if (fr.$1 != null) fields.add(fr.$1!);
      start = fr.$2;
    } else if (_isProtoType(t) || _isIdentifier(t)) {
      final fr = _parseField(tokens, start, optional: false);
      if (fr.$1 != null) fields.add(fr.$1!);
      start = fr.$2;
    } else {
      start++;
    }
  }
  if (start < tokens.length && tokens[start] == '}') start++;
  return (_Message(name, fields, nestedEnums, nestedMessages), start);
}

(_Field?, int) _parseField(
  List<String> tokens,
  int start, {
  required bool optional,
}) {
  var repeated = false;
  if (start < tokens.length && tokens[start] == 'repeated') {
    repeated = true;
    start++;
  } else if (start < tokens.length && tokens[start] == 'optional') {
    optional = true;
    start++;
  } else if (start < tokens.length && tokens[start] == 'required') {
    start++;
  }

  if (start >= tokens.length) return (null, start);
  final type = tokens[start];
  start++;
  if (start >= tokens.length) return (null, start);
  final fieldName = tokens[start];
  start++;

  // skip '= number [options];'
  while (start < tokens.length && tokens[start] != ';' && tokens[start] != '}') {
    start++;
  }
  if (start < tokens.length && tokens[start] == ';') start++;

  if (!_isIdentifier(fieldName) && !_isProtoType(fieldName)) {
    return (null, start);
  }
  return (
    _Field(
      type: type,
      name: fieldName,
      repeated: repeated,
      optional: optional,
    ),
    start,
  );
}

(_Field?, int) _parseMapField(List<String> tokens, int start) {
  // map<KeyType, ValueType> name = number;
  start++; // skip 'map'
  if (start < tokens.length && tokens[start] == '<') start++;
  final keyType = start < tokens.length ? tokens[start] : 'string';
  start++;
  if (start < tokens.length && tokens[start] == ',') start++;
  final valueType = start < tokens.length ? tokens[start] : 'string';
  start++;
  if (start < tokens.length && tokens[start] == '>') start++;
  final fieldName = start < tokens.length ? tokens[start] : '';
  start++;
  // skip '= number;'
  while (start < tokens.length && tokens[start] != ';') start++;
  if (start < tokens.length && tokens[start] == ';') start++;

  return (
    _Field(
      type: '',
      name: fieldName,
      isMap: true,
      mapKeyType: keyType,
      mapValueType: valueType,
    ),
    start,
  );
}

(_Enum, int) _parseEnum(List<String> tokens, int start) {
  final name = tokens[start];
  start++;
  if (start < tokens.length && tokens[start] == '{') start++;

  final values = <_EnumValue>[];
  while (start < tokens.length && tokens[start] != '}') {
    final t = tokens[start];
    if (t == 'option' || t == 'reserved') {
      while (start < tokens.length && tokens[start] != ';') start++;
      start++;
      continue;
    }
    if (_isIdentifier(t)) {
      final valueName = t;
      start++;
      if (start < tokens.length && tokens[start] == '=') start++;
      var num = 0;
      if (start < tokens.length) {
        num = int.tryParse(tokens[start]) ?? 0;
        start++;
      }
      // skip options [(...)] and semicolon
      while (start < tokens.length && tokens[start] != ';') start++;
      if (start < tokens.length && tokens[start] == ';') start++;
      values.add(_EnumValue(valueName, num));
    } else {
      start++;
    }
  }
  if (start < tokens.length && tokens[start] == '}') start++;
  return (_Enum(name, values), start);
}

(_Service, int) _parseService(List<String> tokens, int start) {
  final name = tokens[start];
  start++;
  if (start < tokens.length && tokens[start] == '{') start++;

  final rpcs = <_Rpc>[];
  while (start < tokens.length && tokens[start] != '}') {
    final t = tokens[start];
    if (t == 'rpc') {
      start++;
      final rpcName = tokens[start];
      start++;
      // '(' [stream] InputType ')'
      if (start < tokens.length && tokens[start] == '(') start++;
      var inputStreaming = false;
      if (start < tokens.length && tokens[start] == 'stream') {
        inputStreaming = true;
        start++;
      }
      final inputType = _lastIdentPart(tokens[start]);
      start++;
      if (start < tokens.length && tokens[start] == ')') start++;
      // 'returns' '(' [stream] OutputType ')'
      if (start < tokens.length && tokens[start] == 'returns') start++;
      if (start < tokens.length && tokens[start] == '(') start++;
      var outputStreaming = false;
      if (start < tokens.length && tokens[start] == 'stream') {
        outputStreaming = true;
        start++;
      }
      final outputType = _lastIdentPart(tokens[start]);
      start++;
      if (start < tokens.length && tokens[start] == ')') start++;
      // optional body or semicolon
      if (start < tokens.length && tokens[start] == '{') {
        var depth = 1;
        start++;
        while (start < tokens.length && depth > 0) {
          if (tokens[start] == '{') depth++;
          if (tokens[start] == '}') depth--;
          start++;
        }
      } else if (start < tokens.length && tokens[start] == ';') {
        start++;
      }
      rpcs.add(
        _Rpc(
          rpcName,
          inputType,
          inputStreaming,
          outputType,
          outputStreaming,
        ),
      );
    } else if (t == 'option') {
      while (start < tokens.length && tokens[start] != ';') start++;
      start++;
    } else {
      start++;
    }
  }
  if (start < tokens.length && tokens[start] == '}') start++;
  return (_Service(name, rpcs), start);
}

/// Extract the last part of a fully-qualified name: "ohc.common.Role" → "Role"
String _lastIdentPart(String s) {
  final parts = s.split('.');
  return parts.last;
}

bool _isProtoType(String t) {
  const scalars = {
    'double',
    'float',
    'int32',
    'int64',
    'uint32',
    'uint64',
    'sint32',
    'sint64',
    'fixed32',
    'fixed64',
    'sfixed32',
    'sfixed64',
    'bool',
    'string',
    'bytes',
  };
  return scalars.contains(t);
}

bool _isIdentifier(String t) {
  if (t.isEmpty) return false;
  final first = t.codeUnitAt(0);
  return (first >= 65 && first <= 90) ||
      (first >= 97 && first <= 122) ||
      first == 95;
}

// ─────────────────────────── Type helpers ───────────────────────────────────

/// Proto type → Dart domain type
String _dartType(String protoType, {bool repeated = false}) {
  final base = _dartBaseType(protoType);
  return repeated ? 'List<$base>' : base;
}

String _dartBaseType(String protoType) {
  switch (protoType) {
    case 'string':
      return 'String';
    case 'bool':
      return 'bool';
    case 'int32':
    case 'sint32':
    case 'sfixed32':
    case 'uint32':
    case 'fixed32':
      return 'int';
    case 'int64':
    case 'sint64':
    case 'sfixed64':
    case 'uint64':
    case 'fixed64':
      return 'int';
    case 'float':
    case 'double':
      return 'double';
    case 'bytes':
      return 'List<int>';
    case 'google.protobuf.Timestamp':
    case 'Timestamp':
      return 'DateTime';
    case 'google.protobuf.Duration':
    case 'Duration':
      return 'Duration';
    default:
      // Strip package prefix for user-defined types
      return _lastIdentPart(protoType);
  }
}

/// Default value for a Dart type in fromJson / constructor
String _defaultForType(String protoType, {bool repeated = false}) {
  if (repeated) return 'const []';
  switch (protoType) {
    case 'string':
      return "''";
    case 'bool':
      return 'false';
    case 'int32':
    case 'sint32':
    case 'sfixed32':
    case 'uint32':
    case 'fixed32':
    case 'int64':
    case 'sint64':
    case 'sfixed64':
    case 'uint64':
    case 'fixed64':
      return '0';
    case 'float':
    case 'double':
      return '0.0';
    case 'bytes':
      return 'const []';
    case 'google.protobuf.Timestamp':
    case 'Timestamp':
      return 'DateTime(0)';
    case 'google.protobuf.Duration':
    case 'Duration':
      return 'Duration.zero';
    default:
      return 'null';
  }
}

bool _isScalar(String protoType) {
  const scalars = {
    'string',
    'bool',
    'int32',
    'sint32',
    'sfixed32',
    'uint32',
    'fixed32',
    'int64',
    'sint64',
    'sfixed64',
    'uint64',
    'fixed64',
    'float',
    'double',
    'bytes',
  };
  return scalars.contains(protoType);
}

bool _isTimestamp(String t) =>
    t == 'google.protobuf.Timestamp' || t == 'Timestamp';

bool _isDuration(String t) =>
    t == 'google.protobuf.Duration' || t == 'Duration';

/// Convert proto snake_case to Dart camelCase
String _camelCase(String s) {
  final parts = s.split('_');
  if (parts.length == 1) return s;
  return parts.first +
      parts.skip(1).map((p) => p.isEmpty ? '' : p[0].toUpperCase() + p.substring(1)).join('');
}

// ─────────────────────────── Code generator ─────────────────────────────────

String _generateCode(_ProtoFile proto, String inputPath) {
  final buf = StringBuffer();
  final baseName = inputPath.split('/').last.replaceAll('.proto', '');

  buf.writeln('// Generated by generate_domain_models.dart. DO NOT EDIT.');
  buf.writeln('// Source: $inputPath');
  buf.writeln();
  buf.writeln("// ignore_for_file: always_use_package_imports, directives_ordering");
  buf.writeln();
  buf.writeln("import 'dart:convert';");
  buf.writeln();
  buf.writeln("import 'package:fixnum/fixnum.dart' show Int64;");
  buf.writeln("import 'package:protobuf/protobuf.dart' as \$pb;");
  buf.writeln("import 'package:grpc/grpc.dart' as grpc;");
  buf.writeln("import 'package:yaml/yaml.dart' as \$yaml;");
  buf.writeln();
  buf.writeln("import '${baseName}.pb.dart' as pb;");
  if (proto.services.isNotEmpty) {
    buf.writeln("import '${baseName}.pbgrpc.dart' as pb_grpc;");
  }
  buf.writeln();

  // Collect all known message and enum names for type lookup
  final allMessageNames = <String>{};
  final allEnumNames = <String>{};
  for (final m in proto.messages) {
    _collectNames(m, allMessageNames, allEnumNames);
  }
  for (final e in proto.enums) {
    allEnumNames.add(e.name);
  }

  // Generate top-level enums
  for (final e in proto.enums) {
    _generateEnum(buf, e);
  }

  // Generate message classes
  for (final msg in proto.messages) {
    _generateMessage(buf, msg, allMessageNames, allEnumNames);
  }

  // Generate gRPC client wrappers
  for (final svc in proto.services) {
    _generateGrpcClient(buf, svc, allMessageNames);
  }

  // YAML helper
  buf.writeln('// ─── YAML serialization helper ───');
  buf.writeln('String _toYamlString(Object? value, {int indent = 0}) {');
  buf.writeln("  final pad = '  ' * indent;");
  buf.writeln('  if (value == null) return "\${pad}null";');
  buf.writeln('  if (value is Map) {');
  buf.writeln('    if (value.isEmpty) return "\${pad}{}";');
  buf.writeln('    final lines = value.entries.map((e) {');
  buf.writeln("      final v = e.value;");
  buf.writeln("      if (v is Map || v is List) {");
  buf.writeln("        return '\${pad}\${e.key}:\\n\${_toYamlString(v, indent: indent + 1)}';");
  buf.writeln("      }");
  buf.writeln("      return '\${pad}\${e.key}: \${_toYamlString(v, indent: indent)}';");
  buf.writeln('    });');
  buf.writeln("    return lines.join('\\n');");
  buf.writeln('  }');
  buf.writeln('  if (value is List) {');
  buf.writeln('    if (value.isEmpty) return "\${pad}[]";');
  buf.writeln('    return value');
  buf.writeln("        .map((e) => '\${pad}- \${_toYamlString(e, indent: indent + 1)}')");
  buf.writeln("        .join('\\n');");
  buf.writeln('  }');
  buf.writeln('  if (value is String) {');
  buf.writeln("    if (value.contains('\\n') || value.contains(':')) {");
  buf.writeln("      return '\${pad}>\\'\\n\${value.split(\"\\n\").map((l) => \"\${pad}  \$l\").join(\"\\n\")}';");
  buf.writeln('    }');
  buf.writeln("    return '\${pad}\${value}';");
  buf.writeln('  }');
  buf.writeln("  return '\${pad}\${value}';");
  buf.writeln('}');
  buf.writeln();

  return buf.toString();
}

void _collectNames(
  _Message msg,
  Set<String> msgNames,
  Set<String> enumNames,
) {
  msgNames.add(msg.name);
  for (final e in msg.nestedEnums) {
    enumNames.add(e.name);
  }
  for (final m in msg.nestedMessages) {
    _collectNames(m, msgNames, enumNames);
  }
}

void _generateEnum(StringBuffer buf, _Enum e) {
  buf.writeln('/// Domain enum for ${e.name}.');
  buf.writeln('enum ${e.name} {');
  for (final v in e.values) {
    final dartName = _camelCase(v.name.toLowerCase());
    buf.writeln('  $dartName,');
  }
  buf.writeln('}');
  buf.writeln();
  // Helper extension: fromProto / toProto
  buf.writeln('extension ${e.name}X on ${e.name} {');
  buf.writeln('  pb.${e.name} toProto() {');
  buf.writeln('    switch (this) {');
  for (final v in e.values) {
    final dartName = _camelCase(v.name.toLowerCase());
    buf.writeln('      case ${e.name}.$dartName: return pb.${e.name}.${v.name};');
  }
  buf.writeln('    }');
  buf.writeln('  }');
  buf.writeln();
  buf.writeln('  static ${e.name} fromProto(pb.${e.name} v) {');
  buf.writeln('    switch (v) {');
  for (final v in e.values) {
    final dartName = _camelCase(v.name.toLowerCase());
    buf.writeln('      case pb.${e.name}.${v.name}: return ${e.name}.$dartName;');
  }
  buf.writeln('      default: return ${e.name}.${_camelCase(e.values.first.name.toLowerCase())};');
  buf.writeln('    }');
  buf.writeln('  }');
  buf.writeln('}');
  buf.writeln();
}

void _generateMessage(
  StringBuffer buf,
  _Message msg,
  Set<String> allMsgNames,
  Set<String> allEnumNames,
) {
  // Generate nested enums first
  for (final e in msg.nestedEnums) {
    _generateEnum(buf, e);
  }
  // Generate nested messages first
  for (final m in msg.nestedMessages) {
    _generateMessage(buf, m, allMsgNames, allEnumNames);
  }

  final fields = msg.fields;
  buf.writeln('/// Domain model for ${msg.name}.');
  buf.writeln('class ${msg.name} {');

  // Fields
  for (final f in fields) {
    final dartT = _fieldDartType(f, allEnumNames);
    final nullable = _isNullable(f, allMsgNames, allEnumNames);
    buf.writeln('  final ${nullable ? "$dartT?" : dartT} ${_camelCase(f.name)};');
  }
  buf.writeln();

  // Constructor
  buf.write('  const ${msg.name}({');
  if (fields.isNotEmpty) {
    buf.writeln();
    for (final f in fields) {
      final nullable = _isNullable(f, allMsgNames, allEnumNames);
      final req = nullable ? '' : 'required ';
      buf.writeln('    ${req}this.${_camelCase(f.name)},');
    }
    buf.write('  }');
  }
  buf.writeln(');');
  buf.writeln();

  // fromProto
  buf.writeln('  factory ${msg.name}.fromProto(pb.${msg.name} p) {');
  buf.writeln('    return ${msg.name}(');
  for (final f in fields) {
    final ccName = _camelCase(f.name);
    buf.writeln('      $ccName: ${_fromProtoExpr(f, allMsgNames, allEnumNames)},');
  }
  buf.writeln('    );');
  buf.writeln('  }');
  buf.writeln();

  // toProto
  buf.writeln('  pb.${msg.name} toProto() {');
  buf.writeln('    return pb.${msg.name}(');
  for (final f in fields) {
    buf.writeln('      ${f.name}: ${_toProtoExpr(f, allMsgNames, allEnumNames)},');
  }
  buf.writeln('    );');
  buf.writeln('  }');
  buf.writeln();

  // fromJson
  buf.writeln('  factory ${msg.name}.fromJson(Map<String, dynamic> json) {');
  buf.writeln('    return ${msg.name}(');
  for (final f in fields) {
    final ccName = _camelCase(f.name);
    buf.writeln("      $ccName: ${_fromJsonExpr(f, allMsgNames, allEnumNames)},");
  }
  buf.writeln('    );');
  buf.writeln('  }');
  buf.writeln();

  // toJson
  buf.writeln('  Map<String, dynamic> toJson() {');
  buf.writeln('    return {');
  for (final f in fields) {
    final ccName = _camelCase(f.name);
    buf.writeln("      '${f.name}': ${_toJsonExpr(f, allMsgNames, allEnumNames)},");
  }
  buf.writeln('    };');
  buf.writeln('  }');
  buf.writeln();

  // fromJsonString
  buf.writeln('  factory ${msg.name}.fromJsonString(String json) =>');
  buf.writeln('      ${msg.name}.fromJson(jsonDecode(json) as Map<String, dynamic>);');
  buf.writeln();

  // toJsonString
  buf.writeln('  String toJsonString() => jsonEncode(toJson());');
  buf.writeln();

  // fromYaml
  buf.writeln('  factory ${msg.name}.fromYaml(String yaml) {');
  buf.writeln('    final map = \$yaml.loadYaml(yaml);');
  buf.writeln('    return ${msg.name}.fromJson(');
  buf.writeln('      Map<String, dynamic>.from(map as Map),');
  buf.writeln('    );');
  buf.writeln('  }');
  buf.writeln();

  // toYaml
  buf.writeln('  String toYaml() => _toYamlString(toJson());');
  buf.writeln();

  // copyWith
  buf.writeln('  ${msg.name} copyWith({');
  for (final f in fields) {
    final dartT = _fieldDartType(f, allEnumNames);
    buf.writeln('    $dartT? ${_camelCase(f.name)},');
  }
  buf.writeln('  }) {');
  buf.writeln('    return ${msg.name}(');
  for (final f in fields) {
    final ccName = _camelCase(f.name);
    buf.writeln('      $ccName: $ccName ?? this.$ccName,');
  }
  buf.writeln('    );');
  buf.writeln('  }');
  buf.writeln();

  // == and hashCode
  buf.writeln('  @override');
  buf.writeln('  bool operator ==(Object other) {');
  buf.writeln('    if (identical(this, other)) return true;');
  buf.writeln('    return other is ${msg.name}');
  for (var i = 0; i < fields.length; i++) {
    final ccName = _camelCase(fields[i].name);
    final sep = i < fields.length - 1 ? '' : ';';
    buf.writeln('        && $ccName == other.$ccName$sep');
  }
  if (fields.isEmpty) {
    buf.writeln('    ;');
  }
  buf.writeln('  }');
  buf.writeln();

  buf.writeln('  @override');
  if (fields.isEmpty) {
    buf.writeln('  int get hashCode => runtimeType.hashCode;');
  } else if (fields.length == 1) {
    buf.writeln('  int get hashCode => ${_camelCase(fields[0].name)}.hashCode;');
  } else {
    final hashFields = fields.map((f) => _camelCase(f.name)).join(', ');
    buf.writeln('  int get hashCode => Object.hash($hashFields);');
  }
  buf.writeln();

  // toString
  final fieldStrs = fields
      .map((f) => '${_camelCase(f.name)}: \$${_camelCase(f.name)}')
      .join(', ');
  buf.writeln('  @override');
  buf.writeln("  String toString() => '${msg.name}($fieldStrs)';");
  buf.writeln('}');
  buf.writeln();
}

/// Determine if a field should be nullable in the domain model
bool _isNullable(
  _Field f,
  Set<String> allMsgNames,
  Set<String> allEnumNames,
) {
  if (f.isMap) return false;
  if (f.repeated) return false;
  if (f.optional) return true;
  // Message fields are nullable (may not be set)
  final baseType = _lastIdentPart(f.type);
  if (allMsgNames.contains(baseType)) return true;
  if (_isTimestamp(f.type) || _isDuration(f.type)) return true;
  return false;
}

String _fieldDartType(_Field f, Set<String> allEnumNames) {
  if (f.isMap) {
    final kType = _dartBaseType(f.mapKeyType);
    final vType = _dartBaseType(f.mapValueType);
    return 'Map<$kType, $vType>';
  }
  return _dartType(f.type, repeated: f.repeated);
}

String _fromProtoExpr(
  _Field f,
  Set<String> allMsgNames,
  Set<String> allEnumNames,
) {
  final ccName = _camelCase(f.name);
  if (f.isMap) {
    final baseType = _lastIdentPart(f.mapValueType);
    if (allMsgNames.contains(baseType)) {
      return 'p.${f.name}.map((k, v) => MapEntry(k, ${baseType}.fromProto(v)))';
    }
    return 'Map<${_dartBaseType(f.mapKeyType)}, ${_dartBaseType(f.mapValueType)}>.from(p.${f.name})';
  }
  if (f.repeated) {
    final baseType = _lastIdentPart(f.type);
    if (allMsgNames.contains(baseType)) {
      return 'p.${f.name}.map(${baseType}.fromProto).toList()';
    }
    if (allEnumNames.contains(baseType)) {
      return 'p.${f.name}.map((e) => ${baseType}X.fromProto(e)).toList()';
    }
    if (_isTimestamp(f.type)) {
      return 'p.${f.name}.map((t) => t.toDateTime()).toList()';
    }
    return 'List<${_dartBaseType(f.type)}>.from(p.${f.name})';
  }
  final baseType = _lastIdentPart(f.type);
  if (allMsgNames.contains(baseType)) {
    return 'p.has${_ucFirst(f.name)}() ? ${baseType}.fromProto(p.${f.name}) : null';
  }
  if (allEnumNames.contains(baseType)) {
    return '${baseType}X.fromProto(p.${f.name})';
  }
  if (_isTimestamp(f.type)) {
    return 'p.has${_ucFirst(f.name)}() ? p.${f.name}.toDateTime() : null';
  }
  if (_isDuration(f.type)) {
    return 'p.has${_ucFirst(f.name)}() ? Duration(microseconds: p.${f.name}.seconds.toInt() * 1000000 + p.${f.name}.nanos ~/ 1000) : null';
  }
  if (f.type == 'int64' || f.type == 'uint64' || f.type == 'sint64' || f.type == 'sfixed64' || f.type == 'fixed64') {
    return 'p.${f.name}.toInt()';
  }
  return 'p.${f.name}';
}

String _toProtoExpr(
  _Field f,
  Set<String> allMsgNames,
  Set<String> allEnumNames,
) {
  final ccName = _camelCase(f.name);
  if (f.isMap) {
    final baseType = _lastIdentPart(f.mapValueType);
    if (allMsgNames.contains(baseType)) {
      return '$ccName.map((k, v) => MapEntry(k, v.toProto()))';
    }
    return ccName;
  }
  if (f.repeated) {
    final baseType = _lastIdentPart(f.type);
    if (allMsgNames.contains(baseType)) {
      return '$ccName.map((e) => e.toProto()).toList()';
    }
    if (allEnumNames.contains(baseType)) {
      return '$ccName.map((e) => e.toProto()).toList()';
    }
    if (_isTimestamp(f.type)) {
      return '$ccName.map((t) => \$pb.Timestamp.fromDateTime(t)).toList()';
    }
    return ccName;
  }
  final baseType = _lastIdentPart(f.type);
  if (allMsgNames.contains(baseType)) {
    return '$ccName?.toProto()';
  }
  if (allEnumNames.contains(baseType)) {
    return '$ccName.toProto()';
  }
  if (_isTimestamp(f.type)) {
    return '$ccName != null ? \$pb.Timestamp.fromDateTime($ccName!) : \$pb.Timestamp()';
  }
  if (_isDuration(f.type)) {
    return '$ccName != null ? \$pb.Duration(seconds: Int64($ccName!.inSeconds), nanos: ($ccName!.inMicroseconds % 1000000) * 1000) : \$pb.Duration()';
  }
  if (f.type == 'int64' || f.type == 'uint64' || f.type == 'sint64' || f.type == 'sfixed64' || f.type == 'fixed64') {
    return 'Int64($ccName)';
  }
  return ccName;
}

String _fromJsonExpr(
  _Field f,
  Set<String> allMsgNames,
  Set<String> allEnumNames,
) {
  final key = "'${f.name}'";
  final ccName = _camelCase(f.name);
  if (f.isMap) {
    final baseType = _lastIdentPart(f.mapValueType);
    final vDart = _dartBaseType(f.mapValueType);
    if (allMsgNames.contains(baseType)) {
      return "(json[$key] as Map<String, dynamic>? ?? {}).map((k, v) => MapEntry(k, ${baseType}.fromJson(v as Map<String, dynamic>)))";
    }
    return "(json[$key] as Map<String, dynamic>? ?? {}).cast<${_dartBaseType(f.mapKeyType)}, $vDart>()";
  }
  if (f.repeated) {
    final baseType = _lastIdentPart(f.type);
    if (allMsgNames.contains(baseType)) {
      return "(json[$key] as List<dynamic>? ?? []).map((e) => ${baseType}.fromJson(e as Map<String, dynamic>)).toList()";
    }
    if (allEnumNames.contains(baseType)) {
      return "(json[$key] as List<dynamic>? ?? []).map((e) => ${baseType}.values.firstWhere((v) => v.name == e as String, orElse: () => ${baseType}.values.first)).toList()";
    }
    if (f.type == 'string') {
      return "(json[$key] as List<dynamic>? ?? []).cast<String>()";
    }
    return "(json[$key] as List<dynamic>? ?? []).cast<${_dartBaseType(f.type)}>()";
  }
  final baseType = _lastIdentPart(f.type);
  if (allMsgNames.contains(baseType)) {
    return "json[$key] != null ? ${baseType}.fromJson(json[$key] as Map<String, dynamic>) : null";
  }
  if (allEnumNames.contains(baseType)) {
    return "${baseType}.values.firstWhere((e) => e.name == (json[$key] as String? ?? ''), orElse: () => ${baseType}.values.first)";
  }
  if (_isTimestamp(f.type)) {
    return "json[$key] != null ? DateTime.parse(json[$key] as String) : null";
  }
  if (_isDuration(f.type)) {
    return "json[$key] != null ? Duration(microseconds: int.parse(json[$key] as String)) : null";
  }
  switch (f.type) {
    case 'string':
      return "json[$key] as String? ?? ''";
    case 'bool':
      return "json[$key] as bool? ?? false";
    case 'int32':
    case 'sint32':
    case 'sfixed32':
    case 'uint32':
    case 'fixed32':
    case 'int64':
    case 'sint64':
    case 'sfixed64':
    case 'uint64':
    case 'fixed64':
      return "(json[$key] as num?)?.toInt() ?? 0";
    case 'float':
    case 'double':
      return "(json[$key] as num?)?.toDouble() ?? 0.0";
    case 'bytes':
      return "(json[$key] as List<dynamic>? ?? []).cast<int>()";
    default:
      return "json[$key]";
  }
}

String _toJsonExpr(
  _Field f,
  Set<String> allMsgNames,
  Set<String> allEnumNames,
) {
  final ccName = _camelCase(f.name);
  if (f.isMap) {
    final baseType = _lastIdentPart(f.mapValueType);
    if (allMsgNames.contains(baseType)) {
      return '$ccName.map((k, v) => MapEntry(k, v.toJson()))';
    }
    return ccName;
  }
  if (f.repeated) {
    final baseType = _lastIdentPart(f.type);
    if (allMsgNames.contains(baseType)) {
      return '$ccName.map((e) => e.toJson()).toList()';
    }
    if (allEnumNames.contains(baseType)) {
      return '$ccName.map((e) => e.name).toList()';
    }
    if (_isTimestamp(f.type)) {
      return '$ccName.map((t) => t.toIso8601String()).toList()';
    }
    return ccName;
  }
  final baseType = _lastIdentPart(f.type);
  if (allMsgNames.contains(baseType)) {
    return '$ccName?.toJson()';
  }
  if (allEnumNames.contains(baseType)) {
    return '$ccName.name';
  }
  if (_isTimestamp(f.type)) {
    return '$ccName?.toIso8601String()';
  }
  if (_isDuration(f.type)) {
    return '$ccName?.inMicroseconds.toString()';
  }
  return ccName;
}

void _generateGrpcClient(
  StringBuffer buf,
  _Service svc,
  Set<String> allMsgNames,
) {
  buf.writeln('/// gRPC client wrapper for ${svc.name}.');
  buf.writeln('class ${svc.name}Client {');
  buf.writeln('  final pb_grpc.${svc.name}Client _stub;');
  buf.writeln();
  buf.writeln('  ${svc.name}Client(grpc.ClientChannel channel)');
  buf.writeln('      : _stub = pb_grpc.${svc.name}Client(channel);');
  buf.writeln();
  buf.writeln('  ${svc.name}Client.withStub(this._stub);');
  buf.writeln();

  for (final rpc in svc.rpcs) {
    final inType = rpc.inputType;
    final outType = rpc.outputType;
    final inDart = allMsgNames.contains(inType) ? inType : 'pb.${inType}';
    final outDart = allMsgNames.contains(outType) ? outType : 'pb.${outType}';
    final methodName = _camelCase(
      rpc.name[0].toLowerCase() + rpc.name.substring(1),
    );

    if (!rpc.inputStreaming && !rpc.outputStreaming) {
      // Unary
      buf.writeln('  Future<$outDart> $methodName($inDart request) async {');
      final reqExpr = allMsgNames.contains(inType)
          ? 'request.toProto()'
          : 'request';
      buf.writeln('    final response = await _stub.$methodName($reqExpr);');
      final retExpr = allMsgNames.contains(outType)
          ? '${outType}.fromProto(response)'
          : 'response';
      buf.writeln('    return $retExpr;');
      buf.writeln('  }');
    } else if (!rpc.inputStreaming && rpc.outputStreaming) {
      // Server streaming
      buf.writeln('  Stream<$outDart> $methodName($inDart request) {');
      final reqExpr = allMsgNames.contains(inType)
          ? 'request.toProto()'
          : 'request';
      final mapExpr = allMsgNames.contains(outType)
          ? '.map(${outType}.fromProto)'
          : '';
      buf.writeln('    return _stub.$methodName($reqExpr)$mapExpr;');
      buf.writeln('  }');
    } else if (rpc.inputStreaming && !rpc.outputStreaming) {
      // Client streaming
      buf.writeln(
        '  Future<$outDart> $methodName(Stream<$inDart> requests) async {',
      );
      final reqExpr = allMsgNames.contains(inType)
          ? 'requests.map((r) => r.toProto())'
          : 'requests';
      buf.writeln('    final response = await _stub.$methodName($reqExpr);');
      final retExpr = allMsgNames.contains(outType)
          ? '${outType}.fromProto(response)'
          : 'response';
      buf.writeln('    return $retExpr;');
      buf.writeln('  }');
    } else {
      // Bidirectional streaming
      buf.writeln(
        '  Stream<$outDart> $methodName(Stream<$inDart> requests) {',
      );
      final reqExpr = allMsgNames.contains(inType)
          ? 'requests.map((r) => r.toProto())'
          : 'requests';
      final mapExpr = allMsgNames.contains(outType)
          ? '.map(${outType}.fromProto)'
          : '';
      buf.writeln('    return _stub.$methodName($reqExpr)$mapExpr;');
      buf.writeln('  }');
    }
    buf.writeln();
  }
  buf.writeln('}');
  buf.writeln();
}

String _ucFirst(String s) => s.isEmpty ? s : s[0].toUpperCase() + s.substring(1);
