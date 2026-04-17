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
import 'package:mocktail/mocktail.dart';
import 'package:centrifuge/centrifuge.dart';

class MockApiService extends Mock implements ApiService {}
class MockCentrifugeService extends Mock implements CentrifugeService {}

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
    final mockCentrifuge = MockCentrifugeService();
    when(() => mockCentrifuge.subscribe(any())).thenAnswer((_) => const Stream.empty());

    // Create a large simulated desktop view so nothing overflows and fields are accessible
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApi),
          centrifugeServiceProvider.overrideWithValue(mockCentrifuge),
        ],
        child: const OhcApp(),
      ),
    );

    // Initial load
    await tester.pumpAndSettle();

    // Start from landing, go to login
    final continueToCloudBtn = find.text('Or continue to Cloud Dashboard');
    if (continueToCloudBtn.evaluate().isNotEmpty) {
       await tester.ensureVisible(continueToCloudBtn);
       await tester.tap(continueToCloudBtn, warnIfMissed: false);
       await tester.pumpAndSettle();
    }

    // On LoginScreen
    final emailField = find.widgetWithText(TextFormField, 'Email');
    await tester.ensureVisible(emailField);
    await tester.enterText(emailField, 'user@example.com');
    await tester.enterText(find.widgetWithText(TextFormField, 'Password'), 'correctpw');
    await tester.tap(find.text('Sign In'));

    // Instead of pumpAndSettle which might get stuck or not find nav components quickly,
    // wait for authentication response and navigation
    for(int i=0; i<30; i++) {
        await tester.pump(const Duration(milliseconds: 500));
        if (find.text('Swarm Memory').evaluate().isNotEmpty) {
            break;
        }
    }

    // Navigate to Swarm Memory screen
    final navItemElements = find.text('Swarm Memory').evaluate().toList();
    if(navItemElements.isNotEmpty) {
       await tester.tap(find.byWidget(navItemElements.first.widget), warnIfMissed: false);
    }

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
