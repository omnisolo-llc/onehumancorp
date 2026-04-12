class SwarmTask {
  final String id;
  final String title;
  final String description;
  final String status;
  final String? assignedAgent;
  final List<String> dependencies;

  SwarmTask({
    required this.id,
    required this.title,
    required this.description,
    required this.status,
    this.assignedAgent,
    this.dependencies = const [],
  });

  factory SwarmTask.fromJson(Map<String, dynamic> json) {
    return SwarmTask(
      id: json['id'] as String,
      title: json['title'] as String,
      description: json['description'] as String,
      status: json['status'] as String,
      assignedAgent: json['assigned_agent'] as String?,
      dependencies: (json['dependencies'] as List<dynamic>?)?.map((e) => e as String).toList() ?? [],
    );
  }
}
