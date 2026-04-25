import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/help/help_center_screen.dart';

void main() {
  testWidgets('HelpCenterScreen renders topics and search bar correctly', (WidgetTester tester) async {
    // Set a large viewport so the list doesn't scroll offscreen
    tester.view.physicalSize = const Size(1000, 2000);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(
      const MaterialApp(
        home: HelpCenterScreen(),
      ),
    );

    // Reset view size
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    // Verify Title
    expect(find.text('Help Center'), findsOneWidget);
    expect(find.text('How can we help you today?'), findsOneWidget);

    // Verify Search Bar
    expect(find.byType(TextField), findsOneWidget);
    expect(find.text('Search help articles...'), findsOneWidget);

    // Verify Topics
    expect(find.text('Getting Started'), findsOneWidget);
    expect(find.text('My Store'), findsOneWidget);
    expect(find.text('Payments'), findsOneWidget);

    // In ListView, items might be scrolled out of view.
    // Use scrollUntilVisible if we want to test them all, or just skip checking the ones off-screen in a simple test.
    // For now we test just the visible ones or use a larger screen size.
  });
}
