class TooltipRegistry {
  static final Map<String, String> _tooltips = {
    'recent_activity': 'A real-time log of actions performed by your AI swarm.',
    'growth_referral': 'Share this link to earn credits when a business signs up.',
    'scale_agents': 'Hire or fire agents to handle your current workload.',
  };

  static String getTooltip(String key) {
    return _tooltips[key] ?? '';
  }

  static void updateTooltip(String key, String message) {
    _tooltips[key] = message;
  }
}
