import 'package:app/providers/wizard_provider.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/main.dart';
import 'package:app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('Onboarding E2E: Standard Path (Modern Template)', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: OHCApp()));

    final emailField = find.byKey(const Key('signupEmailField'));
    await tester.ensureVisible(emailField);
    await tester.enterText(emailField, 'test1@example.com');
    await tester.enterText(find.byKey(const Key('signupPasswordField')), 'pass1');
    await tester.ensureVisible(find.byKey(const Key('signupBtn')));
    await tester.tap(find.byKey(const Key('signupBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    await tester.enterText(find.byKey(const Key('companyNameField')), 'Company 1');
    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.text('Build software'));
    await tester.pump(const Duration(milliseconds: 500));
    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.text('Cloud'));
    await tester.pump(const Duration(milliseconds: 500));
    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.enterText(find.byKey(const Key('adminNameField')), 'Admin 1');
    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.text('Modern'));
    await tester.pump(const Duration(milliseconds: 500));
    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.enterText(find.byKey(const Key('productNameField')), 'Prod 1');
    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.byKey(const Key('launchAIBtn')));
    await tester.pump(const Duration(seconds: 3));


  });

  testWidgets('Onboarding E2E: Minimum Inputs', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: OHCApp()));

    await tester.enterText(find.byKey(const Key('signupEmailField')), 'min@example.com');
    await tester.enterText(find.byKey(const Key('signupPasswordField')), 'p');
    await tester.ensureVisible(find.byKey(const Key('signupBtn')));
    await tester.tap(find.byKey(const Key('signupBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    for (int i = 0; i < 8; i++) {
      await tester.tap(find.byType(ElevatedButton).last);
      await tester.pump(const Duration(milliseconds: 500));
    }

    await tester.tap(find.byKey(const Key('launchAIBtn')));
    await tester.pump(const Duration(seconds: 3));


  });

  testWidgets('Onboarding E2E: Back and Forth Navigation', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: OHCApp()));

    await tester.enterText(find.byKey(const Key('signupEmailField')), 'back@example.com');
    await tester.enterText(find.byKey(const Key('signupPasswordField')), 'p');
    await tester.ensureVisible(find.byKey(const Key('signupBtn')));
    await tester.tap(find.byKey(const Key('signupBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    await tester.enterText(find.byKey(const Key('companyNameField')), 'A');
    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.text('Back').last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.enterText(find.byKey(const Key('companyNameField')), 'B');
    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.text('Back').last);
    await tester.pump(const Duration(milliseconds: 500));

    expect(find.text('What are your goals?'), findsOneWidget);
  });

  testWidgets('Onboarding E2E: Cozy Template', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: OHCApp()));

    await tester.enterText(find.byKey(const Key('signupEmailField')), 'cozy@example.com');
    await tester.enterText(find.byKey(const Key('signupPasswordField')), 'p');
    await tester.ensureVisible(find.byKey(const Key('signupBtn')));
    await tester.tap(find.byKey(const Key('signupBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    for (int i = 0; i < 5; i++) {
      await tester.tap(find.byType(ElevatedButton).last);
      await tester.pump(const Duration(milliseconds: 500));
    }

    await tester.tap(find.text('Cozy'));
    await tester.pump(const Duration(milliseconds: 500));
    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.enterText(find.byKey(const Key('productNameField')), 'Cake');
    await tester.enterText(find.byKey(const Key('productPriceField')), '15');
    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    expect(find.text('Cake'), findsOneWidget);
    await tester.tap(find.byKey(const Key('launchAIBtn')));
    await tester.pump(const Duration(seconds: 3));

  });

  testWidgets('Onboarding E2E: Cross Device Resume Simulation', (WidgetTester tester) async {
    final container = ProviderContainer(
      overrides: [
        wizardProvider.overrideWith(() => PreloadedWizardNotifier()),
      ],
    );

    await tester.pumpWidget(
      UncontrolledProviderScope(
        container: container,
        child: const MaterialApp(
          home: BusinessSetupWizardScreen(environmentMode: EnvironmentMode.cloud),
        ),
      ),
    );

    expect(find.text('Add your first product or service'), findsOneWidget);
    await tester.enterText(find.byKey(const Key('productNameField')), 'Resumed Product');
    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(const Duration(milliseconds: 500));

    expect(find.text('Resumed Product'), findsOneWidget);
    expect(find.text('Acme Resumed'), findsOneWidget);

    await tester.tap(find.byKey(const Key('launchAIBtn')));
    await tester.pump(const Duration(seconds: 3));

  });
}

class PreloadedWizardNotifier extends WizardNotifier {
  @override
  WizardState build() {
    return WizardState(
      currentStep: 7,
      companyName: 'Acme Resumed',
      industry: 'Technology',
      templateSelection: 'modern',
    );
  }
}
