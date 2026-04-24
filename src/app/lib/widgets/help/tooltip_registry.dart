class TooltipRegistry {
  static const Map<String, String> tooltips = {
    'dashboard_title': 'Your central command center. See how your business is performing today at a glance.',
    'agents_title': 'Your digital workforce. Hire and manage AI agents to automate tasks.',
    'help_center_search': 'Find answers instantly. Search for any topic related to running your business on OHC.',
    'agent_hire_btn': 'Ready to grow? Hire a new AI agent to take over a department of your business.',
    'cost_dashboard_title': 'Track your AI spending. See exactly how much your digital workforce costs and where you can optimize.',
  };

  static String get(String id) {
    return tooltips[id] ?? 'Help information coming soon.';
  }
}
