import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/landing_screen.dart';

void main() {
  testWidgets('Landing screen displays key local-first features', (tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: LandingScreen(),
        ),
      ),
    );

    expect(find.text('The Ultimate Business OS'), findsOneWidget);
    expect(find.text('Automated Operations'), findsOneWidget);
    expect(find.text('Total Data Privacy'), findsOneWidget);
    expect(find.text('Unlimited Growth'), findsOneWidget);
    expect(find.text('Download for Mac'), findsOneWidget);
    expect(find.text('Download for Windows'), findsOneWidget);
    expect(find.text('Download for Linux'), findsOneWidget);
  });
}
