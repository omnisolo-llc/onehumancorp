import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/help_center_screen.dart';

void main() {
  testWidgets('HelpCenterScreen renders sections correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(
      home: HelpCenterScreen(),
    ));

    // Verify Title
    expect(find.text('Help Center'), findsOneWidget);
    expect(find.text('How can we help you today?'), findsOneWidget);

    // Verify Search Field
    expect(find.byType(TextField), findsOneWidget);

    // Verify Video Tutorials section
    expect(find.text('Video Tutorials'), findsOneWidget);
    expect(find.text('Tutorial 1: Basics'), findsOneWidget);

    // Verify Browse Topics section
    expect(find.text('Browse Topics'), findsOneWidget);
    expect(find.text('Getting Started'), findsOneWidget);
    expect(find.text('My Store'), findsOneWidget);
    expect(find.text('Payments'), findsOneWidget);
    expect(find.text('AI Agents'), findsOneWidget);
    expect(find.text('Marketing'), findsOneWidget);
    expect(find.text('Account & Billing'), findsOneWidget);
  });
}
