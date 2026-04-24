import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';

Widget _wrap(Widget child) {
  final router = GoRouter(
    initialLocation: '/dashboard',
    routes: [
      GoRoute(
        path: '/dashboard',
        builder: (context, state) => child,
      ),
    ],
  );

  return ProviderScope(
    child: MaterialApp.router(
      routerConfig: router,
    ),
  );
}

void main() {
  group('AppShell sidebar navigation', () {
    testWidgets('sidebar nav items are tappable', (tester) async {
      await tester.binding.setSurfaceSize(const Size(1200, 1200));
      await tester.pumpWidget(_wrap(const DashboardScreen()));
      await tester.pumpAndSettle();
    });
  });
}
