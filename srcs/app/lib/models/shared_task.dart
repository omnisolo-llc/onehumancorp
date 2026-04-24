import 'yaml_utils.dart';
class SharedTask {
  final String id;
  final String title;
  final String status;
  final String? agentId;
  final List<dynamic>? dependencies;
  final String? parentTaskId;
  final String? workflowState;

  SharedTask({
    required this.id,
    required this.title,
    required this.status,
    this.agentId,
    this.dependencies,
    this.parentTaskId,
    this.workflowState,
  });

  factory SharedTask.fromJson(Map<String, dynamic> json) {
    return SharedTask(
      id: json['id'] as String,
      title: json['title'] as String,
      status: json['status'] as String,
      agentId: json['agent_id'] as String?,
      dependencies: json['dependencies'] as List<dynamic>?,
      parentTaskId: json['parent_task_id'] as String?,
      workflowState: json['workflow_state'] as String?,
    );
  }
}
