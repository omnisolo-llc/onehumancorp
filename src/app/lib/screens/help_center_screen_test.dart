import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/help_center_screen.dart';

void main() {
  testWidgets('HelpCenterScreen renders and shows categories', (WidgetTester tester) async {
    final router = GoRouter(
      initialLocation: '/help-center',
      routes: [
        GoRoute(
          path: '/help-center',
          builder: (context, state) => const HelpCenterScreen(),
        ),
        GoRoute(
          path: '/chat',
          builder: (context, state) => const Scaffold(body: Text('Chat Screen Mock')),
        ),
      ],
    );

    await tester.pumpWidget(
      ProviderScope(
        child: MaterialApp.router(
          routerConfig: router,
        ),
      ),
    );

    await tester.pumpAndSettle();

    expect(find.text('Help Center'), findsOneWidget);
    expect(find.text('How can we help you run your business today?'), findsOneWidget);
    expect(find.text('Getting Started'), findsOneWidget);
    expect(find.text('My Store'), findsOneWidget);
    expect(find.text('Ask anything'), findsOneWidget);
  });
}
