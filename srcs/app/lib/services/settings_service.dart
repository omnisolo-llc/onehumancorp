import 'dart:convert';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';

/// Client-side settings for the OHC dashboard.
class ClientSettings {
  final String backendUrl;
  final bool standaloneMode;

  const ClientSettings({
    required this.backendUrl,
    required this.standaloneMode,
  });

  ClientSettings copyWith({String? backendUrl, bool? standaloneMode}) {
    return ClientSettings(
      backendUrl: backendUrl ?? this.backendUrl,
      standaloneMode: standaloneMode ?? this.standaloneMode,
    );
  }

  Map<String, dynamic> toJson() => {
    'backendUrl': backendUrl,
    'standaloneMode': standaloneMode,
  };

  factory ClientSettings.fromJson(Map<String, dynamic> json) {
    return ClientSettings(
      backendUrl: json['backendUrl'] as String? ?? 'http://localhost:18789',
      standaloneMode: json['standaloneMode'] as bool? ?? false,
    );
  }
}

final _prefsProvider = FutureProvider<SharedPreferences>(
  (_) => SharedPreferences.getInstance(),
);

final clientSettingsProvider =
    AsyncNotifierProvider<ClientSettingsNotifier, ClientSettings>(() {
      return ClientSettingsNotifier();
    });

class ClientSettingsNotifier extends AsyncNotifier<ClientSettings> {
  static const _key = 'client_settings';

  @override
  Future<ClientSettings> build() async {
    final prefs = await ref.watch(_prefsProvider.future);
    final json = prefs.getString(_key);
    if (json == null) {
      // Check environment variable if web/desktop supports it via string.fromEnvironment
      const envUrl = String.fromEnvironment(
        'BACKEND_URL',
        defaultValue: 'http://localhost:18789',
      );
      const envStandalone = bool.fromEnvironment(
        'OHC_STANDALONE',
        defaultValue: false,
      );
      return const ClientSettings(
        backendUrl: envUrl,
        standaloneMode: envStandalone,
      );
    }
    return ClientSettings.fromJson(jsonDecode(json) as Map<String, dynamic>);
  }

  Future<void> updateBackendUrl(String url) async {
    final current = state.valueOrNull;
    if (current == null) return;
    state = AsyncData(current.copyWith(backendUrl: url));
    await _save();
  }

  Future<void> updateStandaloneMode(bool enabled) async {
    final current = state.valueOrNull;
    if (current == null) return;
    state = AsyncData(current.copyWith(standaloneMode: enabled));
    await _save();
  }

  Future<void> _save() async {
    final current = state.valueOrNull;
    if (current == null) return;
    final prefs = await ref.read(_prefsProvider.future);
    await prefs.setString(_key, jsonEncode(current.toJson()));
  }
}
