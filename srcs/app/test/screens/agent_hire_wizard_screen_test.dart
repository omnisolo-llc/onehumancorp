import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/agent_hire_wizard_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/models/agent.dart';
import 'package:mocktail/mocktail.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  late MockApiService mockApiService;

  setUp(() {
    mockApiService = MockApiService();
    when(() => mockApiService.listAgentProviders()).thenAnswer((_) async => [
      AgentProvider(
        type: 'test',
        description: 'Test Provider',
        supportedRoles: ['Support'],
        isAuthenticated: false,
      )
    ]);
  });

  testWidgets('AgentHireWizardScreen test API key visibility toggle', (WidgetTester tester) async {
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApiService),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: AgentHireWizardScreen(),
          ),
        ),
      ),
    );

    await tester.pumpAndSettle();

    // Select role so 'Next' button becomes enabled
    final choiceChip = find.text('Support');
    expect(choiceChip, findsOneWidget);
    await tester.tap(choiceChip);
    await tester.pumpAndSettle();

    // Advance to Limits step (Step 6 / index 5)
    for (int i = 0; i < 5; i++) {
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();
    }

    // Toggle advanced settings
    final switchFinder = find.byType(Switch).last;
    await tester.ensureVisible(switchFinder);
    await tester.tap(switchFinder);
    await tester.pumpAndSettle();

    // Verify API Key input exists
    final apiKeyField = find.ancestor(
      of: find.text('API Key (Optional)'),
      matching: find.byType(TextField),
    );
    expect(apiKeyField, findsOneWidget);

    final textField = tester.widget<TextField>(apiKeyField);
    expect(textField.obscureText, isTrue);

    // Toggle visibility
    final visibilityBtn = find.byTooltip('Toggle API Key Visibility');
    expect(visibilityBtn, findsOneWidget);
    await tester.tap(visibilityBtn);
    await tester.pumpAndSettle();

    final textFieldToggled = tester.widget<TextField>(apiKeyField);
    expect(textFieldToggled.obscureText, isFalse);
  });
}
