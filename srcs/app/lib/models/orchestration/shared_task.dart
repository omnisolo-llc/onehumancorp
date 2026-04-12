class SharedTask {
  final String id;
  final String title;
  final String description;
  final String status;
  final String? assignedAgent;
  final List<String> dependencies;

  SharedTask({
    required this.id,
    required this.title,
    required this.description,
    required this.status,
    this.assignedAgent,
    required this.dependencies,
  });
}
