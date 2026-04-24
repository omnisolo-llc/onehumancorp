import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/website_builder_wizard_screen.dart';

void main() {
  Widget createWidgetUnderTest() {
    return const ProviderScope(
      child: MaterialApp(
        home: WebsiteBuilderWizardScreen(),
      ),
    );
  }

  testWidgets('WebsiteBuilderWizardScreen initial state renders Template gallery', (WidgetTester tester) async {
    tester.view.physicalSize = const Size(1080, 2400);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(createWidgetUnderTest());
    await tester.pumpAndSettle();

    expect(find.text('Choose a Template'), findsOneWidget);
    expect(find.text('Modern minimal'), findsOneWidget);
    expect(find.text('Bold storefront'), findsOneWidget);
    await tester.drag(find.byType(ListView), const Offset(0, -500));
    await tester.pumpAndSettle();
    expect(find.text('Elegant portfolio'), findsOneWidget);

    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });

  testWidgets('WebsiteBuilderWizardScreen flow works correctly', (WidgetTester tester) async {
    tester.view.physicalSize = const Size(1080, 2400);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(createWidgetUnderTest());
    await tester.pumpAndSettle();

    // Step 0: Choose Template
    await tester.tap(find.text('Modern minimal'));
    await tester.pumpAndSettle();

    final nextButton1 = find.widgetWithText(FilledButton, 'Next');
    await tester.tap(nextButton1);
    await tester.pumpAndSettle();

    // Step 1: Brand & Logo
    expect(find.text('Brand & Logo'), findsOneWidget);
    await tester.tap(find.text('Ocean Blue'));
    await tester.pumpAndSettle();

    final nextButton2 = find.widgetWithText(FilledButton, 'Next');
    await tester.tap(nextButton2);
    await tester.pumpAndSettle();

    // Step 2: Add Product
    expect(find.text('Add Your First Product'), findsOneWidget);
    await tester.enterText(find.byType(TextField).at(0), 'Test Product');
    await tester.enterText(find.byType(TextField).at(1), '10');
    await tester.enterText(find.byType(TextField).at(2), 'Description');
    await tester.pumpAndSettle();

    final nextButton3 = find.widgetWithText(FilledButton, 'Next');
    await tester.ensureVisible(nextButton3);
    await tester.tap(nextButton3);
    await tester.pumpAndSettle();

    // Step 3: Connect Domain
    expect(find.text('Connect a Domain'), findsOneWidget);
    await tester.tap(find.text('Use a free OHC subdomain'));
    await tester.pumpAndSettle();

    final nextButton4 = find.widgetWithText(FilledButton, 'Next');
    await tester.tap(nextButton4);
    await tester.pumpAndSettle();

    // Step 4: Publish
    expect(find.text('Ready to go live!'), findsOneWidget);

    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });
}
