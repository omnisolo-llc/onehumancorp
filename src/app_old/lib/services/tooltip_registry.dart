class TooltipRegistry {
  static const Map<String, String> _tooltips = {
    'dashboard_agents': 'See what your AI agents are currently working on.',
    'dashboard_revenue': 'Your total revenue for the current week.',
    'sidebar_help': 'Get help, watch tutorials, or contact support.',
    'sidebar_changelog': 'See what new features we have added recently.',
    'ai_chat_button': 'Ask our AI Help Agent anything about using OHC.',
  };

  static String get(String key) {
    return _tooltips[key] ?? 'Help information unavailable.';
  }
}
