import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/website_builder_wizard_screen.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';

class FakeAuthNotifier extends AuthNotifier {
  @override
  Future<AuthUser?> build() async {
    return const AuthUser(id: 'u1', name: 'admin', token: 'fake', email: 'a@a.com', organizationId: 'o1', role: 'admin');
  }
}

void main() {
  testWidgets('CUJ: Website Builder Wizard Flow', (WidgetTester tester) async {

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          authStateProvider.overrideWith(() => FakeAuthNotifier()),
          backendUrlProvider.overrideWithValue('http://localhost:8080'),
        ],
        child: const MaterialApp(
          home: WebsiteBuilderWizardScreen(),
        ),
      ),
    );

    await tester.pumpAndSettle();

    // Verify we're in the Website Builder Step 0
    expect(find.text('Website Builder'), findsOneWidget);
    expect(find.text('Step 1: Choose a Template'), findsOneWidget);

    // Select 'Minimal'
    await tester.tap(find.text('Minimal'));
    await tester.pumpAndSettle();

    // Click 'Next'
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Verify Step 1
    expect(find.text('Step 2: Brand Colors & Logo'), findsOneWidget);

    // Click 'Next'
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Verify Step 2
    expect(find.text('Step 3: Add your first product or service'), findsOneWidget);

    // Click 'Next'
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Verify Step 3
    expect(find.text('Step 4: Connect a Domain'), findsOneWidget);

    // Click 'Next'
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Verify Step 4
    expect(find.text('Step 5: Go Live'), findsOneWidget);

  });
}
