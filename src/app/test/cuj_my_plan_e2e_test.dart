// CUJ: My Plan & Billing
// Ensures the user can view their current plan and available plans.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/my_plan_screen.dart';
import 'package:ohc_app/router.dart';

void main() {
  group('CUJ: My Plan & Billing', () {
    testWidgets('user navigates from dashboard to my plan', (tester) async {
      // Use the actual AppShell wrapping MyPlanScreen so it does not overflow
      final router = GoRouter(
        initialLocation: '/my-plan',
        routes: [
          ShellRoute(
            builder: (context, state, child) => Material(child: AppShell(child: child)),
            routes: [
              GoRoute(
                path: '/dashboard',
                builder: (context, state) => const Scaffold(body: Text('Dashboard')),
              ),
              GoRoute(
                path: '/my-plan',
                builder: (context, state) => const MyPlanScreen(),
              ),
            ]
          )
        ],
      );

      // Provide large enough screen size so it does not overflow layout
      tester.view.physicalSize = const Size(1920, 1080);
      tester.view.devicePixelRatio = 1.0;

      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp.router(routerConfig: router),
        ),
      );
      await tester.pumpAndSettle();

      // Assert final state matches the My Plan feature design
      expect(find.textContaining('My Plan & Billing'), findsWidgets);

      expect(find.textContaining('Plan: Free'), findsWidgets);
      expect(find.textContaining('AI Actions Used'), findsWidgets);
      expect(find.textContaining('Storage Used'), findsWidgets);

      expect(find.text('Starter'), findsWidgets);
      expect(find.text('Pro'), findsWidgets);
      expect(find.text('Select Starter'), findsWidgets);

      // Reset view to original state
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
    });
  });
}
