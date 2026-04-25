class TooltipRegistry {
  static const Map<String, String> messages = {
    'dashboard_hire': 'Hire a new AI agent to handle tasks automatically.',
    'dashboard_fire': 'Remove this AI agent from your team.',
    'dashboard_title': 'Your central hub to view your business health and manage operations.',
    'ai_agents_nav': 'Manage your AI assistants here.',
    'revenue_chart': 'Daily revenue tracked over the last 30 days.',
  };

  static String get(String key) => messages[key] ?? '';

  static String? getTooltip(String key) {
    return messages[key];
  }
}
