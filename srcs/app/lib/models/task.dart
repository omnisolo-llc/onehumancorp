class SharedTask {
  final String id;
  final String title;
  final String? assignedAgent;
  final String status;
  final List<String> dependencies;

  const SharedTask({
    required this.id,
    required this.title,
    this.assignedAgent,
    required this.status,
    required this.dependencies,
  });

  factory SharedTask.fromJson(Map<String, dynamic> json) {
    return SharedTask(
      id: json['id'] as String? ?? '',
      title: json['title'] as String? ?? '',
      assignedAgent: json['assigned_agent'] as String?,
      status: json['status'] as String? ?? 'PENDING',
      dependencies: (json['dependencies'] as List<dynamic>?)?.map((e) => e as String).toList() ?? [],
    );
  }
}
