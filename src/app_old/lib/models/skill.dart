import 'yaml_utils.dart';
/// Skill / plugin model.
class Skill {
  final String name;
  final String version;
  final String description;
  final String category;
  final bool installed;
  final bool enabled;

  const Skill({
    required this.name,
    required this.version,
    required this.description,
    required this.category,
    required this.installed,
    required this.enabled,
  });

  factory Skill.fromJson(Map<String, dynamic> json) {
    return Skill(
      name: json['name'] as String? ?? '',
      version: json['version'] as String? ?? '0.0.0',
      description: json['description'] as String? ?? '',
      category: json['category'] as String? ?? 'community',
      installed: json['installed'] as bool? ?? false,
      enabled: json['enabled'] as bool? ?? false,
    );
  }

  Skill copyWith({bool? installed, bool? enabled}) {
    return Skill(
      name: name,
      version: version,
      description: description,
      category: category,
      installed: installed ?? this.installed,
      enabled: enabled ?? this.enabled,
    );
  }

  Map<String, dynamic> toJson() => {
    'name': name,
    'version': version,
    'description': description,
    'category': category,
    'installed': installed,
    'enabled': enabled,
  };

  /// Serializes this Skill to a YAML string.
  String toYaml() => modelToYaml(toJson());

  /// Deserializes a YAML string to a [Skill].
  static Skill fromYaml(String yaml) => Skill.fromJson(modelFromYaml(yaml));
}
