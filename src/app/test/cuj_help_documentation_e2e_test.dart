import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/help/help_center_screen.dart';
import 'package:ohc_app/screens/help/help_article_screen.dart';

Widget _wrapScreen(Widget screen) {
  final router = GoRouter(
    initialLocation: '/help',
    routes: [
      GoRoute(path: '/help', builder: (context, state) => screen),
      GoRoute(
        path: '/help/article/:id',
        builder: (context, state) => HelpArticleScreen(articleId: state.pathParameters['id']!),
      ),
    ],
  );

  return MaterialApp.router(
    routerConfig: router,
  );
}

void main() {
  testWidgets('E2E: CUJ Help Documentation Flow', (WidgetTester tester) async {
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(_wrapScreen(const HelpCenterScreen()));
    await tester.pumpAndSettle();

    expect(find.text('Help Center'), findsOneWidget);

    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });
}
