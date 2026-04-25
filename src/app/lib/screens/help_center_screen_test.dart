import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/help_center_screen.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('HelpCenterScreen renders search bar and categories', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: HelpCenterScreen(),
      ),
    );

    // Verify title
    expect(find.text('Help Center'), findsOneWidget);

    // Verify search bar
    expect(find.byType(TextField), findsOneWidget);

    // Verify categories
    expect(find.text('Browse by Topic'), findsOneWidget);
    expect(find.text('Getting Started'), findsOneWidget);
    expect(find.text('Account & Billing'), findsOneWidget);
  });

  testWidgets('HelpCenterScreen search filters articles', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: HelpCenterScreen(),
      ),
    );

    // Enter text into search bar
    await tester.enterText(find.byType(TextField), 'Apple Pay');
    await tester.pumpAndSettle();

    // Verify search results section appears
    expect(find.text('Search Results'), findsOneWidget);
    expect(find.text('Search result for "Apple Pay"'), findsOneWidget);
  });

  testWidgets('HelpCenterScreen renders Ask AI Support FAB', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: HelpCenterScreen(),
      ),
    );

    expect(find.text('Ask AI Support'), findsOneWidget);
    expect(find.byIcon(Icons.chat), findsOneWidget);
  });
}
