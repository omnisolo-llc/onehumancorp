import 'dart:convert';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:http/http.dart' as http;
import 'package:ohc_app/services/settings_service.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'package:ohc_app/models/user.dart';

/// Currently authenticated user info.
class AuthUser {
  final User user;
  final String token;

  const AuthUser({
    required this.user,
    required this.token,
  });

  String get id => user.metadata.id;
  String get email => user.metadata.email;
  String get name => user.metadata.username;
  String get role => user.metadata.roles.isNotEmpty ? user.metadata.roles.first : 'viewer';
  String get organizationId => user.metadata.organizationId;

  factory AuthUser.fromJson(Map<String, dynamic> json, String token) {
    return AuthUser(
      user: User.fromJson(json),
      token: token,
    );
  }
}

/// Authentication service — communicates with the OHC backend.
class AuthService {
  final String baseUrl;
  final http.Client _client;

  AuthService({required this.baseUrl, http.Client? client})
    : _client = client ?? http.Client();

  Future<AuthUser> login(String identifier, String password) async {
    final response = await _client.post(
      Uri.parse('$baseUrl/api/auth/login'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode({'username': identifier, 'password': password}),
    );
    if (response.statusCode == 200) {
      final data = jsonDecode(response.body) as Map<String, dynamic>;
      final token = data['token'] as String;
      final user = data['user'] as Map<String, dynamic>;
      return AuthUser.fromJson(user, token);
    }
    throw Exception('Login failed: ${response.statusCode}');
  }

  Future<void> logout(String token) async {
    await _client.post(
      Uri.parse('$baseUrl/api/auth/logout'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer $token',
      },
    );
  }
}

// ── Providers ──────────────────────────────────────────────────────────────

final _prefsProvider = FutureProvider<SharedPreferences>(
  (_) => SharedPreferences.getInstance(),
);

final backendUrlProvider = Provider<String>((ref) {
  final settings = ref.watch(clientSettingsProvider).valueOrNull;
  if (settings != null) return settings.backendUrl;
  return defaultBackendUrl();
});

final authServiceProvider = Provider<AuthService>((ref) {
  final url = ref.watch(backendUrlProvider);
  return AuthService(baseUrl: url);
});

/// Emits the currently logged-in [AuthUser] or null when not authenticated.
final authStateProvider = AsyncNotifierProvider<AuthNotifier, AuthUser?>(() {
  return AuthNotifier();
});

class AuthNotifier extends AsyncNotifier<AuthUser?> {
  static const _tokenKey = 'auth_token';

  @override
  Future<AuthUser?> build() async {
    // Attempt to restore session from local storage.
    final prefs = await ref.watch(_prefsProvider.future);
    final token = prefs.getString(_tokenKey);
    if (token == null) return null;
    // Validate by fetching /api/auth/me.
    final url = ref.read(backendUrlProvider);
    try {
      final res = await http.get(
        Uri.parse('$url/api/auth/me'),
        headers: {'Authorization': 'Bearer $token'},
      );
      if (res.statusCode == 200) {
        final data = jsonDecode(res.body) as Map<String, dynamic>;
        return AuthUser.fromJson(data, token);
      }
    } catch (_) {}
    await prefs.remove(_tokenKey);
    return null;
  }

  Future<void> login(String email, String password) async {
    state = const AsyncLoading();
    final service = ref.read(authServiceProvider);
    state = await AsyncValue.guard(() async {
      final user = await service.login(email, password);
      final prefs = await ref.read(_prefsProvider.future);
      await prefs.setString(_tokenKey, user.token);
      return user;
    });
  }

  Future<void> logout() async {
    final user = state.valueOrNull;
    if (user != null) {
      await ref.read(authServiceProvider).logout(user.token);
    }
    final prefs = await ref.read(_prefsProvider.future);
    await prefs.remove(_tokenKey);
    state = const AsyncData(null);
  }
}
