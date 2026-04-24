import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/secure_input_field.dart';

void main() {
  testWidgets('SecureInputField renders with label and hint', (WidgetTester tester) async {
    final controller = TextEditingController();
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SecureInputField(
            controller: controller,
            labelText: 'Password',
            hintText: 'Enter password',
          ),
        ),
      ),
    );

    expect(find.text('Password'), findsOneWidget);
    expect(find.text('Enter password'), findsOneWidget);
  });

  testWidgets('SecureInputField is obscure by default', (WidgetTester tester) async {
    final controller = TextEditingController();
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SecureInputField(
            controller: controller,
            labelText: 'Password',
          ),
        ),
      ),
    );

    final textFieldFinder = find.byType(TextField);
    expect(textFieldFinder, findsOneWidget);

    final textField = tester.widget<TextField>(textFieldFinder);
    expect(textField.obscureText, isTrue);
  });

  testWidgets('SecureInputField toggles visibility on icon tap', (WidgetTester tester) async {
    final controller = TextEditingController();
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SecureInputField(
            controller: controller,
            labelText: 'Password',
          ),
        ),
      ),
    );

    final textFieldFinder = find.byType(TextField);
    final textField1 = tester.widget<TextField>(textFieldFinder);
    expect(textField1.obscureText, isTrue);

    // Tap the toggle icon
    await tester.tap(find.byIcon(Icons.visibility_off));
    await tester.pump();

    final textField2 = tester.widget<TextField>(textFieldFinder);
    expect(textField2.obscureText, isFalse);

    // Tap again
    await tester.tap(find.byIcon(Icons.visibility));
    await tester.pump();

    final textField3 = tester.widget<TextField>(textFieldFinder);
    expect(textField3.obscureText, isTrue);
  });

  testWidgets('SecureInputField calls onChanged', (WidgetTester tester) async {
    final controller = TextEditingController();
    String changedText = '';
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SecureInputField(
            controller: controller,
            labelText: 'Password',
            onChanged: (val) {
              changedText = val;
            },
          ),
        ),
      ),
    );

    await tester.enterText(find.byType(TextField), 'my-password');
    expect(changedText, 'my-password');
  });
}
