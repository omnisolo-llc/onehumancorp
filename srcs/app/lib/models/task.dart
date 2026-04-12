class Task {
  final String id;
  final String title;
  final String status;
  final String? assignedAgent;
  final List<String> dependencies;

  const Task({
    required this.id,
    required this.title,
    required this.status,
    this.assignedAgent,
    this.dependencies = const [],
  });

  factory Task.fromJson(Map<String, dynamic> json) {
    return Task(
      id: json['id'] as String,
      title: json['title'] as String,
      status: json['status'] as String? ?? 'PENDING',
      assignedAgent: json['assigned_agent'] as String?,
      dependencies: List<String>.from(json['dependencies'] ?? []),
    );
  }

  Map<String, dynamic> toJson() => {
    'id': id,
    'title': title,
    'status': status,
    'assigned_agent': assignedAgent,
    'dependencies': dependencies,
  };
}
