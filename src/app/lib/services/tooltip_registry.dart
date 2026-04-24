/// A centralized registry for all in-app plain-language tooltips.
/// This allows documentation updates without searching through the UI code.
class TooltipRegistry {
  static const Map<String, String> _tooltips = {
    'dashboard_growth': 'See how your business is growing over time.',
    'dashboard_revenue': 'Your total sales revenue this month, before expenses.',
    'dashboard_help': 'Open the Help Center for guides and support.',
    'agent_status': 'Shows if your AI employee is currently working, resting, or blocked.',
    'payment_link': 'A sharable link you can send to customers to collect payment securely.',
  };

  /// Retrieves a tooltip by key. Fallback to key name if not found to ensure it's visible to developers.
  static String get(String key) {
    return _tooltips[key] ?? 'Tooltip text missing for: $key';
  }
}
