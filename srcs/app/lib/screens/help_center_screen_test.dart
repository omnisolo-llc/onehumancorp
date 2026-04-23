import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/help_center_screen.dart';

void main() {
  testWidgets('HelpCenterScreen builds and filters articles', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: HelpCenterScreen()));

    expect(find.text('Help Center'), findsWidgets);
    expect(find.text('How can we help you today?'), findsOneWidget);
    expect(find.byType(TextField), findsOneWidget);

    expect(find.text('Getting Started: Launch your store in 5 minutes'), findsOneWidget);
    expect(find.text('How to accept payments with Stripe'), findsOneWidget);

    await tester.enterText(find.byType(TextField), 'Stripe');
    await tester.pumpAndSettle();

    expect(find.text('How to accept payments with Stripe'), findsOneWidget);
    expect(find.text('Getting Started: Launch your store in 5 minutes'), findsNothing);

    await tester.tap(find.byIcon(Icons.clear));
    await tester.pumpAndSettle();

    expect(find.text('Getting Started: Launch your store in 5 minutes'), findsOneWidget);
  });
}
