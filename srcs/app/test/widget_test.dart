import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/main.dart';

void main() {
  testWidgets('Business Setup Wizard E2E test', (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(const ProviderScope(child: OHCApp()));

    // Verify we are on the Welcome screen
    expect(find.text('Welcome to OHC'), findsOneWidget);
    expect(find.byKey(const Key('signupEmailField')), findsOneWidget);
    expect(find.byKey(const Key('signupPasswordField')), findsOneWidget);
  });
}
