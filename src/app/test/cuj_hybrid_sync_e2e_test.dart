// CUJ: Hybrid Sync – Local-to-Cloud State Synchronization
//
// Covers the synchronization critical user journey:
//   1. Screen renders with "Settings" title
//   2. "Trigger Hybrid Sync" button is present
//   3. Clicking the sync button triggers a sync and shows success

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/settings_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/local_manager_service.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/models/settings.dart';

// ── Mocks & Fakes ─────────────────────────────────────────────────────────

class MockHttpClient extends Mock implements http.Client {}

class FakeUri extends Fake implements Uri {}

class _FakeLocalManagerService implements LocalManagerService {
  bool _running = false;

  @override
  Future<bool> isServiceRunning() async => _running;

  @override
  Future<void> startService() async => _running = true;

  @override
  Future<void> stopService() async => _running = false;

  @override
  Future<String> runDoctor() async => 'flutter doctor: OK';

  @override
  Future<void> restartService() async {
    await stopService();
    await startService();
  }

  @override
  Future<Map<String, dynamic>> readConfig() async => {};

  @override
  Future<void> writeConfig(Map<String, dynamic> config) async {}

  @override
  Future<String?> getEnvValue(String key) async => null;

  @override
  Future<void> saveEnvValue(String key, String value) async {}

  @override
  Future<Map<String, dynamic>> getSystemInfo() async => {};

  @override
  Future<ProcessResult> processRun(
      String executable, List<String> arguments,
      {String? workingDirectory, Map<String, String>? environment, bool runInShell = false}) async {
    throw UnimplementedError();
  }

  @override
  Future<Process> processStart(
      String executable, List<String> arguments,
      {String? workingDirectory, Map<String, String>? environment, bool runInShell = false}) async {
    throw UnimplementedError();
  }
}

Widget _wrapScreen(Widget screen, ApiService api) {
  final router = GoRouter(
    routes: [
      GoRoute(path: '/', builder: (context, state) => screen),
    ],
  );
  return ProviderScope(
    overrides: [
      apiServiceProvider.overrideWithValue(api),
      localManagerServiceProvider.overrideWithValue(_FakeLocalManagerService()),
      clientSettingsProvider.overrideWith(
        (ref) => ClientSettingsNotifier(ref)
          ..state = const AsyncData(ClientSettings(backendUrl: 'http://localhost', standaloneMode: false)),
      ),
      authStateProvider.overrideWith(() => AuthNotifier()
        ..state = const AsyncData(AuthUser(id: '1', name: 'Test', email: 'test@example.com', role: 'admin', organizationId: 'org-1', token: 'test-token'))),
    ],
    child: MaterialApp.router(routerConfig: router),
  );
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: Hybrid Sync', () {
    testWidgets('renders Settings title in AppBar', (tester) async {
      final mockClient = MockHttpClient();

      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response('{}', 200));

      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const SettingsScreen(), api));
      await tester.pump();

      expect(find.textContaining('Settings'), findsOneWidget);
    });

    testWidgets('Trigger Hybrid Sync button is present and works', (tester) async {
      final mockClient = MockHttpClient();

      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response('{}', 200));

      when(() => mockClient.post(any(), headers: any(named: 'headers'), body: any(named: 'body')))
          .thenAnswer((_) async => http.Response('{"status": "synced"}', 200));

      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const SettingsScreen(), api));
      await tester.pumpAndSettle();

      // Find the listview to ensure scrolling to the button if it is out of view
      final listView = find.byType(ListView).first;
      await tester.drag(listView, const Offset(0.0, -1000.0));
      await tester.pumpAndSettle();

      // Ensure button exists
      final syncBtn = find.text('Trigger Hybrid Sync');
      expect(syncBtn, findsOneWidget);

      // Tap button
      await tester.tap(syncBtn);
      await tester.pumpAndSettle();

      // Check for success text or snackbar (implementation specific)
      expect(find.textContaining('Sync successful'), findsOneWidget);
    });
  });
}
