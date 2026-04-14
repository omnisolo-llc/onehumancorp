import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_web_app/screens/prompt_tuning_wizard_screen.dart';

void main() {
  testWidgets('PromptTuningWizardScreen renders elements correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: PromptTuningWizardScreen()));

    // Verify title
    expect(find.text('Prompt Tuning Wizard'), findsOneWidget);
    expect(find.text('Tune Agent Prompt'), findsOneWidget);

    // Verify text field
    expect(find.byType(TextField), findsOneWidget);
    expect(find.text('Enter your prompt here...'), findsOneWidget);

    // Verify button
    expect(find.byType(ElevatedButton), findsOneWidget);
    expect(find.text('Tune Prompt'), findsOneWidget);
  });
}
