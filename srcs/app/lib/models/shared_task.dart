class SharedTask {
  final String id;
  final String title;
  final String status;
  final String? agentId;
  final List<dynamic>? dependencies;

  SharedTask({
    required this.id,
    required this.title,
    required this.status,
    this.agentId,
    this.dependencies,
  });

  factory SharedTask.fromJson(Map<String, dynamic> json) {
    return SharedTask(
      id: json['id'] as String,
      title: json['title'] as String,
      status: json['status'] as String,
      agentId: json['agent_id'] as String?,
      dependencies: json['dependencies'] as List<dynamic>?,
    );
  }
}
