import 'package:flutter_riverpod/flutter_riverpod.dart';

final tooltipRegistryProvider = Provider<TooltipRegistry>((ref) {
  return TooltipRegistry();
});

class TooltipRegistry {
  final Map<String, String> _tooltips = {
    // Dashboard & Orchestration
    'dashboard_scale_agents': 'Adjust the number of active AI agents to manage your workload.',
    'dashboard_swarm_velocity': 'Track how fast your agents are completing delegated tasks over time.',
    'dashboard_active_helpers': 'The number of AI agents currently running and working on tasks.',
    'task_list_delegation': 'Assign tasks directly to an agent role, like "Salesperson" or "Accountant".',

    // Help & Settings
    'help_center_fab': 'Ask the Help Agent any question or browse plain language guides.',
    'settings_ai_provider': 'Choose which AI brain (like Gemini or OpenAI) runs your agents.',
  };

  /// Returns the plain language tooltip text for a given [key].
  /// If the key is not found, returns a fallback empty string or the key itself in debug mode.
  String getTooltip(String key) {
    return _tooltips[key] ?? '';
  }

  /// Optional: Allow other parts of the app to dynamically register new tooltips
  void registerTooltip(String key, String message) {
    _tooltips[key] = message;
  }
}
