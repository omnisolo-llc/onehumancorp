import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:app/main.dart';

void main() {
  group('E2E Full Feature CUJ', () {
    testWidgets('Journey 1: Onboarding with valid input -> Dashboard -> Empty state validation', (WidgetTester tester) async {
      await tester.pumpWidget(const OHCApp());

      // Start at intake screen
      expect(find.text('What do you want to build today?'), findsOneWidget);
      await tester.enterText(find.byType(TextField), 'I sell vegan cakes');
      await tester.tap(find.text('Continue'));
      await tester.pump();

      // Loading screen
      expect(find.text('Designing storefront...'), findsOneWidget);
      await tester.pump(const Duration(seconds: 3));

      // Review screen
      expect(find.text('Review Your Business'), findsOneWidget);
      await tester.tap(find.text('Launch Business'));
      await tester.pump();

      // Dashboard
      expect(find.text("Dashboard"), findsOneWidget);
      expect(find.text("Draft Quote for John Doe"), findsOneWidget);
      expect(find.text("New DM from Sarah"), findsOneWidget);
    });

    testWidgets('Journey 2: Onboarding with empty input validation', (WidgetTester tester) async {
      await tester.pumpWidget(const OHCApp());

      expect(find.text('What do you want to build today?'), findsOneWidget);

      // Tap continue without entering text
      await tester.tap(find.text('Continue'));
      await tester.pump();

      // Should remain on the intake screen
      expect(find.text('What do you want to build today?'), findsOneWidget);
      expect(find.text('Designing storefront...'), findsNothing);
    });

    testWidgets('Journey 3: Dashboard task approval optimistic UI', (WidgetTester tester) async {
      await tester.pumpWidget(const OHCApp());

      // Start at intake screen
      expect(find.text('What do you want to build today?'), findsOneWidget);
      await tester.enterText(find.byType(TextField), 'I sell vegan cakes');
      await tester.tap(find.text('Continue'));
      await tester.pump();

      // Loading screen
      expect(find.text('Designing storefront...'), findsOneWidget);
      await tester.pump(const Duration(seconds: 3));

      // Review screen
      expect(find.text('Review Your Business'), findsOneWidget);
      await tester.tap(find.text('Launch Business'));
      await tester.pump();

      // Wait for rendering
      await tester.pumpAndSettle();

      expect(find.text("Draft Quote for John Doe"), findsOneWidget);
      expect(find.text("New DM from Sarah"), findsOneWidget);

      // Approve first task
      await tester.tap(find.widgetWithText(ElevatedButton, 'Approve').first);
      await tester.pumpAndSettle();

      // The approved task should be gone, the other should remain
      expect(find.text("Draft Quote for John Doe"), findsNothing);
      expect(find.text("New DM from Sarah"), findsOneWidget);
    });

    testWidgets('Journey 4: Dashboard task rejection optimistic UI', (WidgetTester tester) async {
      await tester.pumpWidget(const OHCApp());

      // Start at intake screen
      expect(find.text('What do you want to build today?'), findsOneWidget);
      await tester.enterText(find.byType(TextField), 'I sell vegan cakes');
      await tester.tap(find.text('Continue'));
      await tester.pump();

      // Loading screen
      expect(find.text('Designing storefront...'), findsOneWidget);
      await tester.pump(const Duration(seconds: 3));

      // Review screen
      expect(find.text('Review Your Business'), findsOneWidget);
      await tester.tap(find.text('Launch Business'));
      await tester.pump();

      await tester.pumpAndSettle();

      expect(find.text("Draft Quote for John Doe"), findsOneWidget);
      expect(find.text("New DM from Sarah"), findsOneWidget);

      // Reject first task
      await tester.tap(find.widgetWithText(TextButton, 'Reject').first);
      await tester.pumpAndSettle();

      // The rejected task should be gone
      expect(find.text("Draft Quote for John Doe"), findsNothing);
      expect(find.text("New DM from Sarah"), findsOneWidget);
    });

    testWidgets('Journey 5: Dashboard all tasks completed empty state', (WidgetTester tester) async {
      await tester.pumpWidget(const OHCApp());

      // Start at intake screen
      expect(find.text('What do you want to build today?'), findsOneWidget);
      await tester.enterText(find.byType(TextField), 'I sell vegan cakes');
      await tester.tap(find.text('Continue'));
      await tester.pump();

      // Loading screen
      expect(find.text('Designing storefront...'), findsOneWidget);
      await tester.pump(const Duration(seconds: 3));

      // Review screen
      expect(find.text('Review Your Business'), findsOneWidget);
      await tester.tap(find.text('Launch Business'));
      await tester.pump();

      await tester.pumpAndSettle();

      // Approve all
      await tester.tap(find.widgetWithText(ElevatedButton, 'Approve').first);
      await tester.pumpAndSettle();
      await tester.tap(find.widgetWithText(ElevatedButton, 'Approve').first);
      await tester.pumpAndSettle();

      // Empty state should appear
      expect(find.text('No pending approvals.'), findsOneWidget);
      expect(find.text("Draft Quote for John Doe"), findsNothing);
      expect(find.text("New DM from Sarah"), findsNothing);
    });
  });
}
