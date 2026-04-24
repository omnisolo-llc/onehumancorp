import 'package:flutter/material.dart';

class TooltipRegistry {
  static final TooltipRegistry _instance = TooltipRegistry._internal();
  factory TooltipRegistry() => _instance;
  TooltipRegistry._internal();

  final Map<String, String> _tooltips = {};

  void registerTooltip(String key, String message) {
    _tooltips[key] = message;
  }

  String getTooltip(String key, {String? fallback}) {
    return _tooltips[key] ?? fallback ?? '';
  }
}
