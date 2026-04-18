/// ProviderType mirrors the ohc.model.ProviderType proto enum.
/// Values must stay in sync with model.proto and wizardpb/wizard.go.
enum ProviderType {
  unspecified,
  openai,
  anthropic,
  google,
  groq,
  ollama,
  openrouter,
  kilo,
  azure,
  amazonBedrock,
  minimax,
  custom;

  static ProviderType fromValue(int v) {
    switch (v) {
      case 1: return ProviderType.openai;
      case 2: return ProviderType.anthropic;
      case 3: return ProviderType.google;
      case 4: return ProviderType.groq;
      case 5: return ProviderType.ollama;
      case 6: return ProviderType.openrouter;
      case 7: return ProviderType.kilo;
      case 8: return ProviderType.azure;
      case 9: return ProviderType.amazonBedrock;
      case 10: return ProviderType.minimax;
      case 99: return ProviderType.custom;
      default: return ProviderType.unspecified;
    }
  }

  int get value {
    switch (this) {
      case ProviderType.openai: return 1;
      case ProviderType.anthropic: return 2;
      case ProviderType.google: return 3;
      case ProviderType.groq: return 4;
      case ProviderType.ollama: return 5;
      case ProviderType.openrouter: return 6;
      case ProviderType.kilo: return 7;
      case ProviderType.azure: return 8;
      case ProviderType.amazonBedrock: return 9;
      case ProviderType.minimax: return 10;
      case ProviderType.custom: return 99;
      default: return 0;
    }
  }

  String get displayName {
    switch (this) {
      case ProviderType.openai: return 'OpenAI';
      case ProviderType.anthropic: return 'Anthropic';
      case ProviderType.google: return 'Google';
      case ProviderType.groq: return 'Groq';
      case ProviderType.ollama: return 'Ollama';
      case ProviderType.openrouter: return 'OpenRouter';
      case ProviderType.kilo: return 'Kilo';
      case ProviderType.azure: return 'Azure OpenAI';
      case ProviderType.amazonBedrock: return 'Amazon Bedrock';
      case ProviderType.minimax: return 'MiniMax';
      case ProviderType.custom: return 'Custom';
      default: return 'Unknown';
    }
  }
}

/// AI provider configuration model.
class AiProvider {
  final String id;
  final String name;
  final ProviderType providerType;
  final String baseUrl;
  final String apiKey;
  final List<String> models;
  final bool isOfficial;

  const AiProvider({
    required this.id,
    required this.name,
    this.providerType = ProviderType.unspecified,
    required this.baseUrl,
    required this.apiKey,
    required this.models,
    required this.isOfficial,
  });

  factory AiProvider.fromJson(Map<String, dynamic> json) {
    return AiProvider(
      id: json['id'] as String? ?? '',
      name: json['name'] as String? ?? '',
      providerType: ProviderType.fromValue(json['provider_type'] as int? ?? 0),
      baseUrl: json['base_url'] as String? ?? '',
      apiKey: json['api_key'] as String? ?? '',
      models: (json['models'] as List<dynamic>?)?.cast<String>() ?? [],
      isOfficial: json['is_official'] as bool? ?? false,
    );
  }

  AiProvider copyWith({String? apiKey, ProviderType? providerType}) {
    return AiProvider(
      id: id,
      name: name,
      providerType: providerType ?? this.providerType,
      baseUrl: baseUrl,
      apiKey: apiKey ?? this.apiKey,
      models: models,
      isOfficial: isOfficial,
    );
  }

  Map<String, dynamic> toJson() => {
    'id': id,
    'name': name,
    'provider_type': providerType.value,
    'base_url': baseUrl,
    'api_key': apiKey,
    'models': models,
    'is_official': isOfficial,
  };
}
