enum AgentState { idle, working, blocked }

class AgentStatus {
  final String id;
  final String name;
  final String currentTask;
  final AgentState state;
  final String avatarUrl;

  AgentStatus({
    required this.id,
    required this.name,
    required this.currentTask,
    required this.state,
    this.avatarUrl = '',
  });

  AgentStatus copyWith({
    String? id,
    String? name,
    String? currentTask,
    AgentState? state,
    String? avatarUrl,
  }) {
    return AgentStatus(
      id: id ?? this.id,
      name: name ?? this.name,
      currentTask: currentTask ?? this.currentTask,
      state: state ?? this.state,
      avatarUrl: avatarUrl ?? this.avatarUrl,
    );
  }
}
