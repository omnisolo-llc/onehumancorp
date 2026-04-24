import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/release_notes_screen.dart';

void main() {
  testWidgets('ReleaseNotesScreen renders sections correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(
      home: ReleaseNotesScreen(),
    ));

    // Verify Title
    expect(find.text("What's New"), findsOneWidget);
    expect(find.text('Latest Updates in One Human Corp'), findsOneWidget);

    // Verify specific release
    expect(find.text('Version 0.4.2'), findsOneWidget);
    expect(find.text('A Smarter Help Center & In-App Assistant'), findsOneWidget);

    // Verify features list
    expect(find.text('New searchable Help Center'), findsOneWidget);

    // Verify history link
    expect(find.text('View Full History on Website'), findsOneWidget);
  });
}
