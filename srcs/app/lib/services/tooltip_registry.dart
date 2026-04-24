class TooltipRegistry {
  static const Map<String, String> _tooltips = {
    'dashboard_velocity': 'Shows how fast your swarm is completing tasks.',
    'dashboard_queue': 'Shows the number of tasks waiting for an agent.',
    'dashboard_hybrid': 'Shows sync status between local and cloud.',
    'dashboard_memory': 'Shows the latest insights saved by AutoDream.',
    'settings_api_key': 'Your secure key to access external services.',
    'agent_status': 'The current state of the agent.',
  };

  static String get(String key) {
    return _tooltips[key] ?? 'No tooltip available for $key';
  }
}
