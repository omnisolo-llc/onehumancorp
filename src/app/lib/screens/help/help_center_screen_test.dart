import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/help/help_center_screen.dart';

void main() {
  testWidgets('HelpCenterScreen renders categories and search bar', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: HelpCenterScreen(),
      ),
    );

    // Verify search bar
    expect(find.byType(TextField), findsOneWidget);
    expect(find.text('Search for articles...'), findsOneWidget);

    // Verify categories
    expect(find.text('Getting Started'), findsOneWidget);
    expect(find.text('My Store'), findsOneWidget);
    expect(find.text('Payments'), findsOneWidget);
    expect(find.text('AI Agents'), findsOneWidget);
    expect(find.text('Marketing'), findsOneWidget);
    expect(find.text('Account & Billing'), findsOneWidget);

    // Verify other resources
    expect(find.text('Other Resources'), findsOneWidget);
    expect(find.text('Video Tutorials'), findsOneWidget);
    expect(find.text("What's New"), findsOneWidget);
    expect(find.text('API Reference'), findsOneWidget);
  });
}
