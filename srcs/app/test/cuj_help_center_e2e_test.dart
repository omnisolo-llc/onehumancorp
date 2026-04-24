import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/help/help_center_screen.dart';
import 'package:ohc_app/screens/help/video_tutorials_screen.dart';
import 'package:ohc_app/screens/help/help_article_screen.dart';
import 'package:ohc_app/screens/help/changelog_screen.dart';
import 'package:ohc_app/screens/help/api_docs_screen.dart';

Widget _wrapScreen() {
  final router = GoRouter(
    initialLocation: '/help',
    routes: [
      GoRoute(path: '/help', builder: (context, state) => const HelpCenterScreen()),
      GoRoute(path: '/help/videos', builder: (context, state) => const VideoTutorialsScreen()),
      GoRoute(path: '/help/changelog', builder: (context, state) => const ChangelogScreen()),
      GoRoute(path: '/help/api-docs', builder: (context, state) => const ApiDocsScreen()),
      GoRoute(
        path: '/help/article/:id',
        builder: (context, state) => HelpArticleScreen(
          articleId: state.pathParameters['id'] ?? 'unknown',
        ),
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
  testWidgets('CUJ: Help Center - Can navigate to video tutorials', (WidgetTester tester) async {
    tester.view.physicalSize = const Size(1080, 2400);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(_wrapScreen());
    await tester.pumpAndSettle();

    expect(find.text('Help Center'), findsOneWidget);
    expect(find.text('Video Tutorials'), findsOneWidget);

    await tester.tap(find.text('Video Tutorials'));
    await tester.pumpAndSettle();

    expect(find.text('Set up your store in 5 minutes'), findsOneWidget);

    await tester.tap(find.text('Set up your store in 5 minutes'));
    await tester.pumpAndSettle();

    expect(find.text('Video Player Placeholder'), findsOneWidget);

    await tester.tap(find.byIcon(Icons.close));
    await tester.pumpAndSettle();

    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });
}
