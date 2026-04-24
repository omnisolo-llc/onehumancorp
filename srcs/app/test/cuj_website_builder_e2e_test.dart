import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/website_builder_wizard_screen.dart';

void main() {
  testWidgets('E2E Website Builder Wizard Flow', (WidgetTester tester) async {
    final container = ProviderContainer();
    tester.view.physicalSize = const Size(1080, 2400);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(
      UncontrolledProviderScope(
        container: container,
        child: const MaterialApp(
          home: WebsiteBuilderWizardScreen(),
        ),
      ),
    );

    await tester.pumpAndSettle();

    expect(find.text('Choose a Template'), findsOneWidget);
    await tester.tap(find.text('Modern minimal'));
    await tester.pumpAndSettle();

    final nextButton1 = find.widgetWithText(FilledButton, 'Next');
    await tester.tap(nextButton1);
    await tester.pumpAndSettle();

    expect(find.text('Brand & Logo'), findsOneWidget);
    await tester.tap(find.text('Ocean Blue'));
    await tester.pumpAndSettle();

    final nextButton2 = find.widgetWithText(FilledButton, 'Next');
    await tester.tap(nextButton2);
    await tester.pumpAndSettle();

    expect(find.text('Add Your First Product'), findsOneWidget);
    await tester.enterText(find.byType(TextField).at(0), 'Test Product');
    await tester.enterText(find.byType(TextField).at(1), '10');
    await tester.enterText(find.byType(TextField).at(2), 'Description');
    await tester.pumpAndSettle();

    final nextButton3 = find.widgetWithText(FilledButton, 'Next');
    await tester.ensureVisible(nextButton3);
    await tester.tap(nextButton3);
    await tester.pumpAndSettle();

    expect(find.text('Connect a Domain'), findsOneWidget);
    await tester.tap(find.text('Use a free OHC subdomain'));
    await tester.pumpAndSettle();

    final nextButton4 = find.widgetWithText(FilledButton, 'Next');
    await tester.tap(nextButton4);
    await tester.pumpAndSettle();

    expect(find.text('Ready to go live!'), findsOneWidget);

    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });
}
