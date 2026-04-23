import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/help_center_screen.dart';

void main() {
  testWidgets('HelpCenterScreen renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: HelpCenterScreen(),
      ),
    );

    // Verify main components are rendered
    expect(find.text('Help Center'), findsOneWidget);
    expect(find.text('How can we help you today?'), findsOneWidget);
    expect(find.byType(TextField), findsOneWidget);

    // Verify topics
    expect(find.text('Getting Started'), findsOneWidget);
    expect(find.text('My Store'), findsOneWidget);

    // Verify articles
    expect(find.text('How to set up your first product'), findsOneWidget);
  });
}
