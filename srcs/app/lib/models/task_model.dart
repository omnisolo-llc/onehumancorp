class Task {
  final String id;
  final String title;
  final String status;
  final String assignedAgent;
  final List<String> dependencies;

  Task({
    required this.id,
    required this.title,
    required this.status,
    required this.assignedAgent,
    required this.dependencies,
  });

  factory Task.fromJson(Map<String, dynamic> json) {
    return Task(
      id: json['id'] ?? '',
      title: json['title'] ?? '',
      status: json['status'] ?? 'PENDING',
      assignedAgent: json['assignedAgent'] ?? 'Unassigned',
      dependencies: (json['dependencies'] as List<dynamic>?)
              ?.map((e) => e.toString())
              .toList() ??
          [],
    );
  }
}
