class Task {
  final String id;
  final String title;
  final String? assignedAgent;
  final String status;
  final List<String> dependencies;

  Task({
    required this.id,
    required this.title,
    this.assignedAgent,
    required this.status,
    required this.dependencies,
  });

  factory Task.fromJson(Map<String, dynamic> json) {
    return Task(
      id: json['id'] as String,
      title: json['title'] as String,
      assignedAgent: json['assigned_agent'] as String?,
      status: json['status'] as String,
      dependencies: (json['dependencies'] as List<dynamic>?)
              ?.map((e) => e as String)
              .toList() ??
          [],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'title': title,
      'assigned_agent': assignedAgent,
      'status': status,
      'dependencies': dependencies,
    };
  }
}
