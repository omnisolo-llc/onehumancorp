import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/help/help_center_screen.dart';
import 'package:ohc_app/screens/help/help_article_screen.dart';
import 'package:ohc_app/screens/help/video_tutorials_screen.dart';
import 'package:ohc_app/models/video_tutorial.dart';
import 'package:ohc_app/services/api_service.dart';

class MockApiService extends Mock implements ApiService {}

Widget _wrapScreen(Widget screen) {
  final router = GoRouter(
    initialLocation: '/help',
    routes: [
      GoRoute(path: '/help', builder: (context, state) => screen),
      GoRoute(path: '/help/article/:id', builder: (context, state) => HelpArticleScreen(articleId: state.pathParameters['id'] ?? 'unknown')),
      GoRoute(path: '/help/video-tutorials', builder: (context, state) => const VideoTutorialsScreen()),
    ],
  );
  final mockApiService = MockApiService();
  when(() => mockApiService.getHelpVideos()).thenAnswer((_) async => [
    VideoTutorial(
      id: 'v1',
      title: 'How to set up your store',
      duration: '1:30',
      description: 'A quick guide to adding products, setting prices, and getting your storefront ready.',
      url: 'https://test.video/1.mp4',
      thumbnail: 'https://test.video/1.jpg',
    ),
  ]);

  return ProviderScope(
    overrides: [
      apiServiceProvider.overrideWithValue(mockApiService),
    ],
    child: MaterialApp.router(routerConfig: router),
  );
}

void main() {
  testWidgets('E2E: CUJ Help Documentation Flow', (WidgetTester tester) async {
    tester.view.physicalSize = const Size(1080, 2400);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(_wrapScreen(const HelpCenterScreen()));
    await tester.pumpAndSettle();

    // Verify Help Center screen
    expect(find.text('Help Center'), findsOneWidget);
    expect(find.text('Getting Started'), findsOneWidget);
    expect(find.text('Video Tutorials'), findsOneWidget);

    // Tap Getting Started article
    await tester.tap(find.text('Getting Started'));
    await tester.pumpAndSettle();

    // Verify Article screen
    expect(find.text('Getting Started'), findsWidgets);
    expect(find.textContaining('set up and ready'), findsOneWidget);

    // Go back
    await tester.tap(find.byType(BackButton));
    await tester.pumpAndSettle();

    // Tap Video Tutorials
    await tester.tap(find.text('Video Tutorials'));
    await tester.pump();
    await tester.pump(const Duration(seconds: 1)); // wait for future

    // Verify Video Tutorials screen
    expect(find.text('Video Tutorials'), findsWidgets);
    expect(find.text('How to set up your store'), findsOneWidget);

    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });
}
