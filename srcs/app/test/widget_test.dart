import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/main.dart';
import 'package:http/testing.dart';
import 'package:http/http.dart' as http;
import 'package:app/services/api_service.dart';
import 'package:app/providers/wizard_provider.dart';

void main() {
  testWidgets('Business Setup Wizard E2E test', (WidgetTester tester) async {
    // Provide a MockClient for tests
    final mockClient = MockClient((request) async {
      return http.Response('{}', 202);
    });

    // Build our app and trigger a frame with the mocked ApiService
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(ApiService(client: mockClient)),
        ],
        child: const OHCApp(),
      ),
    );

    // 1. Welcome Screen
    expect(find.text('Welcome to One Human Corp'), findsOneWidget);
    await tester.tap(find.text('Get Started'));
    await tester.pump(const Duration(milliseconds: 500));

    // 2. Business Profile Screen
    expect(find.text('Business Profile'), findsOneWidget);
    await tester.enterText(find.byKey(const Key('companyNameField')), 'Acme Corp');

    // Select Industry
    await tester.tap(find.byKey(const Key('industryDropdown')));
    await tester.pump(const Duration(milliseconds: 500));
    await tester.tap(find.text('Technology').last);
    await tester.pump(const Duration(milliseconds: 500));

    // Select Size
    await tester.tap(find.byKey(const Key('sizeDropdown')));
    await tester.pump(const Duration(milliseconds: 500));
    await tester.tap(find.text('11-50').last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.text('Next'));
    await tester.pump(const Duration(milliseconds: 500));

    // 3. Goal Selection Screen
    expect(find.text('What are your goals?'), findsOneWidget);
    await tester.tap(find.text('Build software'));
    await tester.pump(const Duration(milliseconds: 500));
    await tester.tap(find.text('Support'));
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.text('Next'));
    await tester.pump(const Duration(milliseconds: 500));

    // 4. Deployment Preference Screen
    expect(find.text('Deployment Preference'), findsOneWidget);
    await tester.tap(find.text('Cloud'));
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.text('Next'));
    await tester.pump(const Duration(milliseconds: 500));

    // 5. Administrator Account Screen
    expect(find.text('Administrator Account'), findsOneWidget);
    await tester.enterText(find.byKey(const Key('adminNameField')), 'John Doe');
    await tester.enterText(find.byKey(const Key('adminEmailField')), 'john@acme.com');
    await tester.enterText(find.byKey(const Key('adminPasswordField')), 'securePassword123');

    await tester.tap(find.text('Next'));
    await tester.pump(const Duration(milliseconds: 500));

    // 6. Review & Launch Screen
    expect(find.text('Review & Launch'), findsOneWidget);
    expect(find.text('Acme Corp'), findsOneWidget);
    expect(find.text('Technology'), findsOneWidget);
    expect(find.text('11-50'), findsOneWidget);
    expect(find.text('Cloud'), findsOneWidget);
    expect(find.text('John Doe'), findsOneWidget);

    await tester.tap(find.text('Launch My AI Team'));
    await tester.pump();

    // The API request doesn't take 2 seconds anymore since it's mocked, but let's wait a bit
    await tester.pump(const Duration(seconds: 1));

    // 7. Dashboard Screen
    expect(find.text("Dashboard"), findsOneWidget);
    expect(find.text("Pending Agent Approvals"), findsOneWidget);
  });
}
