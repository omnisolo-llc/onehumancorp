/// Agent domain model.
class Agent {
  final String id;
  final String name;
  final String role;
  final String status;
  final String organizationId;
  final DateTime createdAt;
  final bool svidVerified;

  const Agent({
    required this.id,
    required this.name,
    required this.role,
    required this.status,
    required this.organizationId,
    required this.createdAt,
    this.svidVerified = false,
  });

  factory Agent.fromJson(Map<String, dynamic> json) {
    return Agent(
      id: json['id'] as String,
      name: json['name'] as String,
      role: json['role'] as String? ?? '',
      status: json['status'] as String? ?? 'pending',
      organizationId: json['organization_id'] as String? ?? '',
      createdAt:
          json['created_at'] != null
              ? DateTime.parse(json['created_at'] as String)
              : DateTime.now(),
      svidVerified: json['svid_verified'] as bool? ?? false,
    );
  }

  Map<String, dynamic> toJson() => {
    'id': id,
    'name': name,
    'role': role,
    'status': status,
    'organization_id': organizationId,
    'created_at': createdAt.toIso8601String(),
    'svid_verified': svidVerified,
  };

  bool get isRunning => status.toLowerCase() == 'running' || status.toUpperCase() == 'ACTIVE';
  bool get isPending => status.toLowerCase() == 'pending' || status.toUpperCase() == 'IDLE';

  String get formattedRole {
    if (role.isEmpty) return role;
    return role
        .replaceAll('_', ' ')
        .toLowerCase()
        .split(' ')
        .map((word) {
          if (word == 'ai') return 'AI';
          if (word == 'ceo') return 'CEO';
          if (word == 'qa') return 'QA';
          if (word == 'cfo') return 'CFO';
          if (word == 'seo') return 'SEO';
          if (word == 'llm') return 'LLM';
          if (word.isEmpty) return word;
          return word[0].toUpperCase() + word.substring(1);
        })
        .join(' ');
  }
}

/// Information about an AI agent provider.
class AgentProvider {
  final String type;
  final String description;
  final List<String> supportedRoles;
  final bool isAuthenticated;

  const AgentProvider({
    required this.type,
    required this.description,
    required this.supportedRoles,
    required this.isAuthenticated,
  });

  factory AgentProvider.fromJson(Map<String, dynamic> json) {
    return AgentProvider(
      type: json['type'] as String,
      description: json['description'] as String? ?? '',
      supportedRoles: List<String>.from(json['supportedRoles'] ?? []),
      isAuthenticated: json['isAuthenticated'] as bool? ?? false,
    );
  }

  String get label {
    switch (type) {
      case 'claude':
        return 'Claude (Anthropic)';
      case 'gemini':
        return 'Gemini (Google)';
      case 'openclaw':
        return 'OpenClaw';
      case 'opencode':
        return 'OpenCode';
      case 'ironclaw':
        return 'IronClaw';
      case 'builtin':
        return 'Built-in';
      default:
        return type.toUpperCase();
    }
  }
}
