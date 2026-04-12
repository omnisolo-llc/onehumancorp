class SharedTask {
  final String id;
  final String title;
  final String? assignedAgent;
  final String status;
  final List<String> dependencies;

  SharedTask({
    required this.id,
    required this.title,
    this.assignedAgent,
    required this.status,
    this.dependencies = const [],
  });

  factory SharedTask.fromJson(Map<String, dynamic> json) {
    return SharedTask(
      id: json['id'],
      title: json['title'],
      assignedAgent: json['assignedAgent'],
      status: json['status'],
      dependencies: List<String>.from(json['dependencies'] ?? []),
    );
  }
}
