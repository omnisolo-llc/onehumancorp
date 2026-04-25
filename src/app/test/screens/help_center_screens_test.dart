import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/help_center_screen.dart';
import 'package:ohc_app/screens/api_documentation_screen.dart';
import 'package:ohc_app/screens/release_notes_screen.dart';

void main() {
  Widget buildTestWidget(Widget child) {
    return ProviderScope(
      child: MaterialApp(
        home: child,
      ),
    );
  }

  testWidgets('HelpCenterScreen renders topics and links', (WidgetTester tester) async {
    await tester.pumpWidget(buildTestWidget(const HelpCenterScreen()));

    expect(find.text('Help Center'), findsOneWidget);
    expect(find.text('Browse by Topic'), findsOneWidget);
    expect(find.text('Getting Started'), findsOneWidget);
    expect(find.text('My Store'), findsOneWidget);
    expect(find.text('Payments'), findsOneWidget);

    expect(find.text('Quick Links'), findsOneWidget);
    expect(find.text('API Documentation'), findsOneWidget);
    expect(find.text('Release Notes'), findsOneWidget);
  });

  testWidgets('ApiDocumentationScreen renders properly', (WidgetTester tester) async {
    await tester.pumpWidget(buildTestWidget(const ApiDocumentationScreen()));

    expect(find.text('OneHumanCorp API'), findsOneWidget);
    expect(find.text('Endpoints'), findsOneWidget);
    expect(find.text('GET'), findsOneWidget);
    expect(find.text('/api/v1/business'), findsOneWidget);
  });

  testWidgets('ReleaseNotesScreen renders properly', (WidgetTester tester) async {
    await tester.pumpWidget(buildTestWidget(const ReleaseNotesScreen()));

    expect(find.text("What's New"), findsOneWidget);
    expect(find.text('Version 1.2.0'), findsOneWidget);
    expect(find.text('New AI Help Center'), findsOneWidget);
  });
}
