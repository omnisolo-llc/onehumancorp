import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter/foundation.dart';
import 'package:ohc_app/screens/landing_screen.dart';

void main() {
  testWidgets('LandingScreen renders with recommended OS highlighted', (WidgetTester tester) async {
    debugDefaultTargetPlatformOverride = TargetPlatform.macOS;

    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: LandingScreen(),
        ),
      ),
    );

    expect(find.text('One Human Corp'), findsOneWidget);

    // Instead of textContaining checking the entire exact string format which might have spans,
    // we'll just check for fragments of the strings
    expect(find.textContaining('Recommended'), findsOneWidget);
    expect(find.textContaining('Download for Mac'), findsOneWidget);
    expect(find.textContaining('Download for Windows'), findsOneWidget);
    expect(find.textContaining('Download for Linux'), findsOneWidget);

    debugDefaultTargetPlatformOverride = null;
  });
}
