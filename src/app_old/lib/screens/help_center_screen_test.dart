import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/help_center_screen.dart';

void main() {
  testWidgets('HelpCenterScreen renders search and articles', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: HelpCenterScreen()));
    expect(find.text('Help Center'), findsOneWidget);
    expect(find.text('Getting Started'), findsOneWidget);
  });
}
