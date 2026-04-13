class SharedTask {
  final String id;
  final String organizationId;
  final String parentPlanId;
  final List<String> dependencies;
  final String title;
  final String description;
  final String assignedAgentId;
  final String status;
  final String priority;
  final String payload;
  final DateTime? lockedUntil;
  final DateTime createdAt;
  final DateTime updatedAt;

  SharedTask({
    required this.id,
    required this.organizationId,
    required this.parentPlanId,
    required this.dependencies,
    required this.title,
    required this.description,
    required this.assignedAgentId,
    required this.status,
    required this.priority,
    required this.payload,
    this.lockedUntil,
    required this.createdAt,
    required this.updatedAt,
  });

  factory SharedTask.fromJson(Map<String, dynamic> json) {
    return SharedTask(
      id: json['id'] as String,
      organizationId: json['organizationId'] as String,
      parentPlanId: json['parentPlanId'] as String,
      dependencies: (json['dependencies'] as List<dynamic>?)?.cast<String>() ?? [],
      title: json['title'] as String,
      description: json['description'] as String,
      assignedAgentId: json['assignedAgentId'] as String? ?? '',
      status: json['status'] as String,
      priority: json['priority'] as String,
      payload: json['payload'] as String,
      lockedUntil: json['lockedUntil'] != null ? DateTime.parse(json['lockedUntil']) : null,
      createdAt: DateTime.parse(json['createdAt']),
      updatedAt: DateTime.parse(json['updatedAt']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'organizationId': organizationId,
      'parentPlanId': parentPlanId,
      'dependencies': dependencies,
      'title': title,
      'description': description,
      'assignedAgentId': assignedAgentId,
      'status': status,
      'priority': priority,
      'payload': payload,
      'lockedUntil': lockedUntil?.toIso8601String(),
      'createdAt': createdAt.toIso8601String(),
      'updatedAt': updatedAt.toIso8601String(),
    };
  }
}
