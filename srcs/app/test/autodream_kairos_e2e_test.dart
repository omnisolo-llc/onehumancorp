import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'package:ohc_app/main.dart';
import 'package:ohc_app/screens/landing_screen.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/centrifuge_service.dart';
import 'package:ohc_app/services/powersync_service.dart';
import 'package:ohc_app/screens/swarm_memory_screen.dart';
import 'package:go_router/go_router.dart';
import 'package:mocktail/mocktail.dart';
import 'package:centrifuge/centrifuge.dart';

class MockApiService extends Mock implements ApiService {}

class _FakeAuthNotifier extends AuthNotifier {
  final AuthUser? initialUser;
  _FakeAuthNotifier([this.initialUser]);

  @override
  Future<AuthUser?> build() async => initialUser;

  @override
  Future<void> login(String email, String password) async {
    state = const AsyncValue.loading();
    await Future.delayed(const Duration(milliseconds: 50));
    state = const AsyncValue.data(AuthUser(id: '123', email: 'user@example.com', name: 'User', role: 'admin', organizationId: 'org1', token: 'fake_token'));
  }

  @override
  Future<void> logout() async {
    state = const AsyncValue.data(null);
  }

  @override
  Future<void> register(String email, String password, String orgName) async {}
}

void main() {
  testWidgets('E2E: AutoDream Vector Consolidation Pipeline', (WidgetTester tester) async {
    final mockApi = MockApiService();

    final authNotifier = _FakeAuthNotifier(const AuthUser(id: '123', email: 'user@example.com', name: 'User', role: 'admin', organizationId: 'org1', token: 'fake_token'));

    // Fix the flex overflow issue from before by wrapping in a large enough constrained box
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);

    // Mount directly with the user authenticated
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          authStateProvider.overrideWith(() => authNotifier),
          apiServiceProvider.overrideWithValue(mockApi),
          centrifugeServiceProvider.overrideWithValue(null), // disable websocket
        ],
        child: const MaterialApp(
          home: Scaffold(body: SwarmMemoryScreen()),
        ),
      ),
    );

    // We cannot use pumpAndSettle because of the repeating animation in VectorMemoryVisualizerWidget
    // So we pump specific durations until it is fully loaded
    await tester.pump(const Duration(milliseconds: 500));
    await tester.pump(const Duration(milliseconds: 500));
    await tester.pump(const Duration(milliseconds: 500));
    await tester.pump(const Duration(milliseconds: 500));

    // Verify UI matches the AutoDream Consolidation Pipeline
    expect(find.text('AutoDream Pipelines'), findsWidgets);
    expect(find.text('AutoDream Consolidation'), findsWidgets);
    expect(find.text('pgvector dimension: 1536'), findsWidgets);
  });
}
