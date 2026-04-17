import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/agent_hire_wizard_screen.dart';
import 'package:ohc_app/models/agent.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:mocktail/mocktail.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  late MockApiService mockApiService;

  setUp(() {
    mockApiService = MockApiService();
    when(() => mockApiService.listAgentProviders()).thenAnswer((_) async => [
      AgentProvider(type: 'openai', description: 'Test', supportedRoles: ['Developer'], isAuthenticated: true),
    ]);
  });

  testWidgets('AgentHireWizardScreen full E2E flow', (WidgetTester tester) async {

    tester.view.physicalSize = const Size(1200, 1600);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(() => tester.view.resetPhysicalSize());
    addTearDown(() => tester.view.resetDevicePixelRatio());

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApiService),
        ],
        child: const MaterialApp(
          home: AgentHireWizardScreen(),
        ),
      ),
    );

    await tester.pumpAndSettle();

    // Step 1: Role
    expect(find.text('Step 1 — Select Agent Role'), findsOneWidget);

    // Tap on the developer role card
    final roleFinder = find.text('Developer');
    expect(roleFinder, findsOneWidget);
    await tester.tap(roleFinder);
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Provider
    expect(find.text('Step 2 — Choose AI Provider'), findsOneWidget);
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: Details
    expect(find.text('Step 3 — Agent Details'), findsOneWidget);
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: Topology
    expect(find.text('Step 4 — Sub-agent Topology'), findsOneWidget);
    expect(find.text('Independent Worker (No Sub-agents)'), findsOneWidget);
    expect(find.text('Delegator / Supervisor (Manages Sub-agents)'), findsOneWidget);

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 5: Capabilities
    expect(find.text('Step 5 — Select Capabilities'), findsOneWidget);
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 6: Limits
    expect(find.text('Step 6 — Resource Limits'), findsOneWidget);

    // Test the Advanced Settings Switch visibility rule
    final switchFinder = find.byType(Switch);
    await tester.ensureVisible(switchFinder);
    await tester.tap(switchFinder);
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 7: Confirm
    expect(find.text('Step 7 — Confirm Deployment'), findsOneWidget);

    // Deploy Agent button
    final deployFinder = find.text('Deploy Agent');
    await tester.ensureVisible(deployFinder);
    expect(deployFinder, findsOneWidget);
  });
}
