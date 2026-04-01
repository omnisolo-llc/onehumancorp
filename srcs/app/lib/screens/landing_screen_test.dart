import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/landing_screen.dart';
import 'package:ohc_app/services/settings_service.dart';

// We need to provide a mock state notifier that overrides the SharedPreferences logic entirely.
class MockClientSettingsNotifier extends StateNotifier<AsyncValue<ClientSettings>> implements ClientSettingsNotifier {
  MockClientSettingsNotifier() : super(const AsyncData(ClientSettings(backendUrl: 'http://localhost', standaloneMode: true)));

  @override
  Future<void> updateBackendUrl(String url) async {}
  @override
  Future<void> updateStandaloneMode(bool enabled) async {}
}

void main() {
  testWidgets('Landing screen displays key local-first features', (tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          clientSettingsProvider.overrideWith((ref) => MockClientSettingsNotifier()),
        ],
        child: const MaterialApp(
          home: LandingScreen(),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('The Hybrid Agentic OS'), findsOneWidget);
    expect(find.text('Zero Data Leakage'), findsOneWidget);
    expect(find.text('Air-Gapped Autonomy'), findsOneWidget);
    expect(find.text('Viral Referral Loop'), findsOneWidget);
    expect(find.text('Launch OHC Desktop'), findsOneWidget);
  });
}
