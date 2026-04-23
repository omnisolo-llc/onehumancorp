import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/welcome_checklist_widget.dart';
import 'package:go_router/go_router.dart';
import 'package:shared_preferences/shared_preferences.dart';

void main() {
  testWidgets('WelcomeChecklistWidget renders correctly and responds to interactions', (WidgetTester tester) async {
    SharedPreferences.setMockInitialValues({});

    String? navigatedPath;

    final router = GoRouter(
      initialLocation: '/',
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const Scaffold(body: WelcomeChecklistWidget()),
        ),
        GoRoute(
          path: '/service',
          builder: (context, state) {
            navigatedPath = '/service';
            return const SizedBox();
          },
        ),
        GoRoute(
          path: '/channels',
          builder: (context, state) {
            navigatedPath = '/channels';
            return const SizedBox();
          },
        ),
      ],
    );

    await tester.pumpWidget(MaterialApp.router(
      routerConfig: router,
    ));
    await tester.pumpAndSettle();

    // Verify widget is visible
    expect(find.text("You're set up! Here's what to do next"), findsOneWidget);
    expect(find.text("Business live"), findsOneWidget);
    expect(find.text("Add 3 more products"), findsOneWidget);
    expect(find.text("Connect Instagram"), findsOneWidget);
    expect(find.text("Share your link with a friend"), findsOneWidget);

    // Test Navigation to /service
    await tester.tap(find.text("Add 3 more products"));
    await tester.pumpAndSettle();
    expect(navigatedPath, '/service');

    // Test Navigation to /channels
    // Need to reset or re-pump depending on go_router behavior, but let's just go back
    router.go('/');
    await tester.pumpAndSettle();
    await tester.tap(find.text("Connect Instagram"));
    await tester.pumpAndSettle();
    expect(navigatedPath, '/channels');

    // Test clipboard / snackbar interaction
    router.go('/');
    await tester.pumpAndSettle();

    // We can't easily test the actual clipboard without a mock, but we can verify the snackbar shows up.
    await tester.tap(find.text("Share your link with a friend"));
    await tester.pump();
    expect(find.text('Link copied to clipboard!'), findsOneWidget);
    await tester.pumpAndSettle(); // dismiss snackbar

    // Test dismiss button
    expect(find.byType(WelcomeChecklistWidget), findsOneWidget);
    await tester.tap(find.byIcon(Icons.close));
    await tester.pumpAndSettle();

    expect(find.text("You're set up! Here's what to do next"), findsNothing);
  });

  testWidgets('WelcomeChecklistWidget is hidden if dismissed previously', (WidgetTester tester) async {
    SharedPreferences.setMockInitialValues({'welcome_checklist_dismissed': true});

    final router = GoRouter(
      initialLocation: '/',
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const Scaffold(body: WelcomeChecklistWidget()),
        ),
      ],
    );

    await tester.pumpWidget(MaterialApp.router(
      routerConfig: router,
    ));
    await tester.pumpAndSettle();

    expect(find.text("You're set up! Here's what to do next"), findsNothing);
  });
}
