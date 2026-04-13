import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/diagnostics_screen.dart';

void main() {
  testWidgets('DiagnosticsScreen displays health checks', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: DiagnosticsScreen(),
        ),
      ),
    );

    expect(find.text('Diagnostics Dashboard'), findsOneWidget);
    expect(find.text('Day One Setup Health Check'), findsOneWidget);
    expect(find.text('Database (PostgreSQL / SQLite)'), findsOneWidget);
    expect(find.text('Redis Cache'), findsOneWidget);
    expect(find.text('AI Provider APIs'), findsOneWidget);
    expect(find.text('Run Diagnostics'), findsOneWidget);
  });
}
