import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/changelog_screen.dart';

void main() {
  testWidgets('ChangelogScreen renders updates', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: ChangelogScreen()));
    expect(find.text("What's New"), findsOneWidget);
    expect(find.text('Recent Updates'), findsOneWidget);
  });
}
