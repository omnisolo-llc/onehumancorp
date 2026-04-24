import 'yaml_utils.dart';
/// Application settings domain model.
class Settings {
  final String? minimaxApiKey;
  final String? theme;
  final String? backendUrl;
  final bool standaloneMode;

  const Settings({
    this.minimaxApiKey,
    this.theme,
    this.backendUrl,
    this.standaloneMode = false,
  });

  factory Settings.fromJson(Map<String, dynamic> json) {
    return Settings(
      minimaxApiKey:
          json['minimax_api_key'] as String? ??
          json['minimaxApiKey'] as String?,
      theme: json['theme'] as String?,
      backendUrl: json['backend_url'] as String?,
      standaloneMode: json['standalone_mode'] as bool? ?? false,
    );
  }

  Map<String, dynamic> toJson() => {
    'minimax_api_key': minimaxApiKey,
    'theme': theme,
    'backend_url': backendUrl,
    'standalone_mode': standaloneMode,
  };

  /// Serializes this Settings to a YAML string.
  String toYaml() => modelToYaml(toJson());

  /// Deserializes a YAML string to a [Settings].
  static Settings fromYaml(String yaml) => Settings.fromJson(modelFromYaml(yaml));
}
