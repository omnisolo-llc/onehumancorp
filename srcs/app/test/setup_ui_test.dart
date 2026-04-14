import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/setup_ui.dart';

void main() {
  testWidgets('SetupUI has correct text and checklists', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: Scaffold(body: SetupUI())));
    expect(find.text('OHC Hybrid OS Setup'), findsOneWidget);
    expect(find.text('1. Setup PostgreSQL'), findsOneWidget);
    expect(find.text('2. Configure Redis'), findsOneWidget);
    expect(find.text('3. Hire Initial Agent'), findsOneWidget);
    expect(find.text('4. Launch Standalone Mode'), findsOneWidget);
  });
}
