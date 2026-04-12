class TaskModel {
  final String id;
  final String title;
  final String status;
  final String assignedAgent;
  final List<String> dependencies;

  TaskModel({
    required this.id,
    required this.title,
    required this.status,
    required this.assignedAgent,
    required this.dependencies,
  });

  factory TaskModel.fromJson(Map<String, dynamic> json) {
    return TaskModel(
      id: json['id'] as String,
      title: json['title'] as String,
      status: json['status'] as String,
      assignedAgent: json['assigned_agent'] as String,
      dependencies: (json['dependencies'] as List<dynamic>?)?.map((e) => e.toString()).toList() ?? [],
    );
  }
}
