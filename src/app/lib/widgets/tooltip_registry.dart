import 'package:flutter/material.dart';

// Tooltip Registry allows agents to add or update tooltips without touching UI code.
// The registry maps a unique string key to the tooltip message.

class TooltipRegistry {
  static final Map<String, String> _tooltips = {
    'dashboard_refresh': 'Refresh dashboard data',
    'agents_hire_new': 'Hire a new agent for your swarm',
    // Add new tooltips here
  };

  static String get(String key, {String fallback = ''}) {
    return _tooltips[key] ?? fallback;
  }
}
