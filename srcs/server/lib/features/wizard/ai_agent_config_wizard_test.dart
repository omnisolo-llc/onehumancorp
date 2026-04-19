import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'ai_agent_config_wizard.dart';

void main() {
  testWidgets('AiAgentConfigWizard renders stepper', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: AiAgentConfigWizard(),
          ),
        ),
      ),
    );
    expect(find.byType(Stepper), findsOneWidget);
    expect(find.text('Step 1 — Choose an Agent'), findsOneWidget);
  });
}
