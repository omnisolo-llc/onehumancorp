import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'prompt_tuning_wizard.dart';

void main() {
  testWidgets('PromptTuningWizard renders stepper', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: PromptTuningWizard(),
          ),
        ),
      ),
    );
    expect(find.byType(Stepper), findsOneWidget);
    expect(find.text('Step 1 — Personality & Tone'), findsOneWidget);
  });
}