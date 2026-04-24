import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/website_builder_wizard_screen.dart';

void main() {
  group('WebsiteBuilderWizardScreen', () {
    testWidgets('renders initial step 0', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: WebsiteBuilderWizardScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      expect(find.text('Website Builder'), findsOneWidget);
      expect(find.text('Select a Template'), findsOneWidget);
      expect(find.text('Next'), findsOneWidget);
    });

    testWidgets('advances to step 1 when Next is pressed', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: WebsiteBuilderWizardScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Tap Next
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();

      expect(find.text('Brand Colors & Logo'), findsOneWidget);
      expect(find.text('Back'), findsOneWidget);
    });

    testWidgets('completes all steps and shows Publish', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: WebsiteBuilderWizardScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Step 0 -> 1
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();

      // Step 1 -> 2
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();

      // Step 2 -> 3
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();

      // Step 3 -> 4
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();

      expect(find.text('Ready to Go Live'), findsOneWidget);
      expect(find.text('Publish →'), findsOneWidget);
    });
  });

  group('WebsiteBuilderNotifier', () {
    test('initial state', () {
      final container = ProviderContainer();
      addTearDown(container.dispose);

      final state = container.read(websiteBuilderProvider);
      expect(state.step, 0);
      expect(state.template, '');
      expect(state.domainChoice, 'Free OHC subdomain');
    });

    test('updates fields correctly', () {
      final container = ProviderContainer();
      addTearDown(container.dispose);

      final notifier = container.read(websiteBuilderProvider.notifier);

      notifier.updateTemplate('Portfolio');
      notifier.updateProductName('My Cool Product');

      final state = container.read(websiteBuilderProvider);
      expect(state.template, 'Portfolio');
      expect(state.productName, 'My Cool Product');
    });
  });
}
