import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/secure_agent_config_screen.dart';
import 'package:ohc_app/widgets/secure_input_field.dart';
import 'package:shared_preferences/shared_preferences.dart';

void main() {
  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  testWidgets('SecureAgentConfigScreen visibility toggle test', (WidgetTester tester) async {
    // Build the SecureAgentConfigScreen widget
    await tester.pumpWidget(
      const MaterialApp(
        home: SecureAgentConfigScreen(),
      ),
    );

    // Verify initial state: obscureText is true, icon is visibility_off
    final secureFieldFinder = find.byType(SecureInputField);
    expect(secureFieldFinder, findsOneWidget);

    final textFieldFinder = find.byType(TextField);
    expect(textFieldFinder, findsOneWidget);

    final TextField textField = tester.widget<TextField>(textFieldFinder);
    expect(textField.obscureText, isTrue);

    expect(find.byIcon(Icons.visibility_off), findsOneWidget);
    expect(find.byIcon(Icons.visibility), findsNothing);

    // Tap the suffix icon to toggle visibility
    await tester.tap(find.byType(IconButton));
    await tester.pumpAndSettle();

    // Verify state after tap: obscureText is false, icon is visibility
    final TextField updatedTextField = tester.widget<TextField>(textFieldFinder);
    expect(updatedTextField.obscureText, isFalse);

    expect(find.byIcon(Icons.visibility), findsOneWidget);
    expect(find.byIcon(Icons.visibility_off), findsNothing);

    // Tap again to toggle back
    await tester.tap(find.byType(IconButton));
    await tester.pumpAndSettle();

    // Verify state after second tap: obscureText is true, icon is visibility_off
    final TextField finalTextField = tester.widget<TextField>(textFieldFinder);
    expect(finalTextField.obscureText, isTrue);

    expect(find.byIcon(Icons.visibility_off), findsOneWidget);
    expect(find.byIcon(Icons.visibility), findsNothing);
  });

  testWidgets('SecureAgentConfigScreen save functionality test', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: SecureAgentConfigScreen(),
        ),
      ),
    );

    // Enter a token
    await tester.enterText(find.byType(TextField), 'spiffe://test.token');
    await tester.pump();

    // Tap the save button
    await tester.tap(find.widgetWithText(ElevatedButton, 'Save Configuration'));

    // We need to pump multiple times because of the async nature of SharedPreferences
    // and the Snackbar/Navigator pop
    await tester.pumpAndSettle();

    // Verify SharedPreferences were updated
    final prefs = await SharedPreferences.getInstance();
    expect(prefs.getString('spiffe_enrollment_token'), 'spiffe://test.token');

    // Since we pop the route, the screen might not be there anymore to show the snackbar,
    // let's just make sure it popped by expecting nothing if it was the only route,
    // or let's wrap it in a router or check the snackbar.
    // Actually the pumpAndSettle will wait for all animations, including snackbar and pop.
    // If it popped, the snackbar might be on the previous route (or gone if it was root).
  });
}
