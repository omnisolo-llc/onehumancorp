import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/release_notes_screen.dart';

void main() {
  testWidgets('ReleaseNotesScreen builds correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: ReleaseNotesScreen()));

    expect(find.text('What\'s New'), findsWidgets);
    expect(find.text('Release Notes'), findsOneWidget);

    expect(find.text('v1.4.0'), findsOneWidget);
    expect(find.text('New AI Social Media Manager & UI Improvements'), findsOneWidget);
    expect(find.text('Added the Promoter Agent to automate your Instagram posts.'), findsOneWidget);
  });
}
