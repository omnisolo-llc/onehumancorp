import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/help_center_screen.dart';

void main() {
  testWidgets('HelpCenterScreen renders search and topics', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: HelpCenterScreen(),
        ),
      ),
    );

    expect(find.text('Help Center'), findsOneWidget);
    expect(find.byType(TextField), findsOneWidget);
    expect(find.text('Getting Started'), findsOneWidget);
    expect(find.text('Video Tutorials'), findsOneWidget);
    expect(find.text('API Documentation (Advanced)'), findsOneWidget);
  });
}
