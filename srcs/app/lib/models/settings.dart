/// Application settings domain model.
class Settings {
  final String? minimaxApiKey;
  final String? theme;
  final String? backendUrl;

  const Settings({
    this.minimaxApiKey,
    this.theme,
    this.backendUrl,
  });

  factory Settings.fromJson(Map<String, dynamic> json) {
    return Settings(
      minimaxApiKey:
          json['minimax_api_key'] as String? ??
          json['minimaxApiKey'] as String?,
      theme: json['theme'] as String?,
      backendUrl: json['backend_url'] as String?,
    );
  }

  Map<String, dynamic> toJson() => {
    'minimax_api_key': minimaxApiKey,
    'theme': theme,
    'backend_url': backendUrl,
  };
}
