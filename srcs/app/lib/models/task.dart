class SwarmTask {
  final String id;
  final String title;
  final String status;
  final String? assignedAgentId;
  final List<String> dependencies;

  const SwarmTask({
    required this.id,
    required this.title,
    required this.status,
    this.assignedAgentId,
    this.dependencies = const [],
  });

  factory SwarmTask.fromJson(Map<String, dynamic> json) {
    return SwarmTask(
      id: json['id'] as String? ?? '',
      title: json['title'] as String? ?? '',
      status: json['status'] as String? ?? 'PENDING',
      assignedAgentId: json['assigned_agent_id'] as String?,
      dependencies: (json['dependencies'] as List<dynamic>?)?.map((e) => e as String).toList() ?? [],
    );
  }
}
