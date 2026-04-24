import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/models/tier.dart';
import 'package:ohc_app/screens/manage_products_screen.dart';

void main() {
  testWidgets('E2E: Free Tier Limit Exhaustion triggers UpgradeBottomSheet', (WidgetTester tester) async {
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;

    // We will test the ManageProductsScreen in isolation but with a pre-configured ProviderScope
    final router = GoRouter(
      initialLocation: '/products',
      routes: [
        GoRoute(
          path: '/products',
          builder: (context, state) => const ManageProductsScreen(),
        ),
      ],
    );

    // Override the tierProvider to start with an exhausted Free tier
    final container = ProviderContainer();
    container.read(tierProvider.notifier).mockFreeTierLimitExceeded();

    await tester.pumpWidget(
      UncontrolledProviderScope(
        container: container,
        child: MaterialApp.router(
          routerConfig: router,
        ),
      ),
    );

    await tester.pumpAndSettle();

    // Verify limit is exhausted in UI (10 / 10)
    expect(find.text('Your Products (10 / 10)'), findsOneWidget);

    // Try adding an 11th product
    await tester.tap(find.text('Add Product'));
    await tester.pumpAndSettle();

    // Verify the UpgradeBottomSheet is shown
    expect(find.text('Your store is growing fast!'), findsOneWidget);
    expect(find.text('Starter Plan'), findsOneWidget);
    expect(find.text('\$9/mo'), findsOneWidget);

    // Tap the upgrade button
    await tester.tap(find.text('Upgrade with Apple / Google Pay'));
    await tester.pump(); // Start the loading state

    // Check for CircularProgressIndicator
    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    // Wait for the simulated delay in _handleUpgrade
    await tester.pump(const Duration(seconds: 2));
    await tester.pumpAndSettle();

    // Verify the bottom sheet is dismissed and the user is on the Starter tier
    expect(find.text('Your store is growing fast!'), findsNothing);
    final tierState = container.read(tierProvider);
    expect(tierState.tierName, 'Starter');
    expect(tierState.maxProducts, 100);
    expect(find.text('Your Products (10 / 100)'), findsOneWidget);

    addTearDown(() => tester.view.resetPhysicalSize());
  });
}
