class Task {
  final String id;
  final String title;
  final String status;
  final String? assignedAgentId;
  final List<String> dependencies;

  const Task({
    required this.id,
    required this.title,
    required this.status,
    this.assignedAgentId,
    this.dependencies = const [],
  });

  factory Task.fromJson(Map<String, dynamic> json) {
    return Task(
      id: json['id'] as String? ?? '',
      title: json['title'] as String? ?? '',
      status: json['status'] as String? ?? 'PENDING',
      assignedAgentId: json['assignedAgentId'] as String? ?? json['assigned_agent_id'] as String?,
      dependencies: (json['dependencies'] as List<dynamic>?)?.map((e) => e as String).toList() ?? [],
    );
  }
}
