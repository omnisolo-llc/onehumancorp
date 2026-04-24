import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/setup_ui.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:shared_preferences/shared_preferences.dart';

void main() {
  testWidgets('SetupUI renders standalone checklist when isStandalone is true', (WidgetTester tester) async {
    SharedPreferences.setMockInitialValues({
      'client_settings': jsonEncode({
        'backendUrl': 'http://localhost',
        'standaloneMode': true,
        'expertMode': false,
      }),
    });

    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: SetupUI(),
          ),
        ),
      ),
    );

    await tester.pumpAndSettle(); // Wait for SharedPreferences to load and state to update

    expect(find.text('OHC Hybrid OS Setup'), findsOneWidget);
    expect(find.text('1. Initialize Local SQLite'), findsOneWidget);
    expect(find.text('2. Bypassed Redis & Postgres'), findsOneWidget);
  });

  testWidgets('SetupUI renders full checklist when isStandalone is false', (WidgetTester tester) async {
    SharedPreferences.setMockInitialValues({
      'client_settings': jsonEncode({
        'backendUrl': 'http://localhost',
        'standaloneMode': false,
        'expertMode': false,
      }),
    });

    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: SetupUI(),
          ),
        ),
      ),
    );

    await tester.pumpAndSettle();

    expect(find.text('OHC Hybrid OS Setup'), findsOneWidget);
    expect(find.text('1. Setup PostgreSQL'), findsOneWidget);
    expect(find.text('2. Configure Redis'), findsOneWidget);
  });
}
