class TooltipRegistry {
  static const Map<String, String> tooltips = {
    'ai_helpers': 'Manage your AI agents. They handle tasks automatically like support and billing.',
    'help_fix': 'Click here to start a wizard that will help you troubleshoot and repair this agent.',
    'ask_anything': 'Chat with our AI Help Agent to get instant answers about using One Human Corp.',
    'help_center': 'Open the Help Center for guides, tutorials, and release notes.',
  };

  static String get(String key) {
    return tooltips[key] ?? '';
  }
}
