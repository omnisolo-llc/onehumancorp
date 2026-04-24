import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/setup_ui.dart';
import 'package:ohc_app/services/settings_service.dart';

void main() {
  testWidgets('SetupUI has correct text and checklists for Cloud Mode', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          clientSettingsProvider.overrideWith(
            (ref) => ClientSettingsNotifier(ref)..state = const AsyncValue.data(
              ClientSettings(backendUrl: 'http://localhost', standaloneMode: false),
            ),
          ),
        ],
        child: const MaterialApp(home: Scaffold(body: SetupUI())),
      ),
    );

    expect(find.text('OHC Hybrid OS Setup'), findsOneWidget);
    expect(find.text('1. Setup PostgreSQL'), findsOneWidget);
    expect(find.text('2. Configure Redis'), findsOneWidget);
    expect(find.text('3. Hire Initial Agent'), findsOneWidget);
    expect(find.text('4. Launch Standalone Mode'), findsOneWidget);
  });

  testWidgets('SetupUI has correct text and checklists for Standalone Mode', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          clientSettingsProvider.overrideWith(
            (ref) => ClientSettingsNotifier(ref)..state = const AsyncValue.data(
              ClientSettings(backendUrl: 'http://localhost', standaloneMode: true),
            ),
          ),
        ],
        child: const MaterialApp(home: Scaffold(body: SetupUI())),
      ),
    );

    expect(find.text('OHC Hybrid OS Setup'), findsOneWidget);
    expect(find.text('1. Initialize Local SQLite'), findsOneWidget);
    expect(find.text('2. Bypassed Redis & Postgres'), findsOneWidget);
    expect(find.text('3. Hire Initial Agent'), findsOneWidget);
    expect(find.text('4. Launch Standalone Mode'), findsOneWidget);
  });
}
