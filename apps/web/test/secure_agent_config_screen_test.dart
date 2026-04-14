import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/secure_agent_config_screen.dart';

void main() {
  testWidgets('SecureAgentConfigScreen visibility toggle', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: SecureAgentConfigScreen()));

    final textFieldFinder = find.byType(TextField);
    expect(textFieldFinder, findsOneWidget);

    TextField textField = tester.widget<TextField>(textFieldFinder);
    expect(textField.obscureText, isTrue);

    final iconButtonFinder = find.byType(IconButton);
    expect(iconButtonFinder, findsOneWidget);

    await tester.tap(iconButtonFinder);
    await tester.pumpAndSettle();

    textField = tester.widget<TextField>(textFieldFinder);
    expect(textField.obscureText, isFalse);
  });
}
