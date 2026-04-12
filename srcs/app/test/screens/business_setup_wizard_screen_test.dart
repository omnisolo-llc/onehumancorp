import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:http/http.dart' as http;
import 'package:ohc_app/services/auth_service.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen full flow test', (WidgetTester tester) async {
    final mockApiService = ApiService(baseUrl: 'http://test', token: 'test-token', client: http.Client());

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApiService),
        ],
        child: const MaterialApp(
          home: BusinessSetupWizardScreen(),
        ),
      ),
    );

    // Step 0: Welcome
    expect(find.text('Business Setup'), findsOneWidget);
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 1: Profile
    expect(find.text('Company Name'), findsOneWidget);
    await tester.enterText(find.byType(TextField).first, 'Test Company');
    await tester.enterText(find.byType(TextField).last, 'Tech');
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Goals
    expect(find.text('Select Goals'), findsOneWidget);
    await tester.tap(find.text('Support'));
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: Deployment
    expect(find.text('Deployment Preference'), findsOneWidget);
    await tester.tap(find.text('Desktop'));
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: Admin
    expect(find.text('Admin Name'), findsOneWidget);
    await tester.enterText(find.byType(TextField).at(0), 'Admin User');
    await tester.enterText(find.byType(TextField).at(1), 'admin@test.com');
    await tester.enterText(find.byType(TextField).at(2), 'password123');
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 5: Review & Launch
    expect(find.text('Review & Launch'), findsOneWidget);
    expect(find.text('Company: Test Company'), findsOneWidget);
    expect(find.text('Industry: Tech'), findsOneWidget);
    expect(find.text('Deployment: Desktop'), findsOneWidget);

    await tester.tap(find.text('Launch My AI Team →'));
    await tester.pumpAndSettle();

    // SnackBar should appear (we mocked the api service)
    expect(find.text('Setup complete!'), findsOneWidget);
  });
}
