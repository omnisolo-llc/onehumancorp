class TooltipRegistry {
  static const Map<String, String> tooltips = {
    // General
    'help_center_button': 'Open the Help Center for guides and support.',
    'ask_anything_button': 'Chat with our AI Support Agent.',

    // Dashboard
    'dashboard_stats': 'A quick overview of your key business numbers today.',
    'upcoming_meetings': 'Your scheduled appointments for the next few days.',

    // Settings
    'theme_toggle': 'Switch between light and dark appearance.',
    'payment_settings': 'Connect your bank account to start accepting payments.',

    // Agents
    'hire_agent_button': 'Add a new AI worker to your team.',
    'fire_agent_button': 'Remove this AI worker from your team.',
    'agent_status': 'Shows if this agent is currently working or idle.',
  };

  static String get(String key) {
    return tooltips[key] ?? '';
  }
}
