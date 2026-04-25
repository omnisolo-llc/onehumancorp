import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/release_notes_screen.dart';

void main() {
  testWidgets('ReleaseNotesScreen renders notes', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: ReleaseNotesScreen(),
        ),
      ),
    );

    expect(find.text("What's New in OHC"), findsOneWidget);
    expect(find.text('AI Marketing Agent Update'), findsOneWidget);
    expect(find.text('v1.4.0'), findsOneWidget);
  });
}
