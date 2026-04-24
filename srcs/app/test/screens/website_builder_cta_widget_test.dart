import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/website_builder_cta_widget.dart';
import 'package:mockito/mockito.dart';

void main() {
  testWidgets('WebsiteBuilderCtaWidget renders and taps correctly', (WidgetTester tester) async {
    bool didNavigate = false;

    final router = GoRouter(
      initialLocation: '/',
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const Scaffold(
            body: WebsiteBuilderCtaWidget(),
          ),
        ),
        GoRoute(
          path: '/wizards/website',
          builder: (context, state) {
            didNavigate = true;
            return const Scaffold(body: Text('Wizard'));
          },
        ),
      ],
    );

    await tester.pumpWidget(
      MaterialApp.router(
        routerConfig: router,
      ),
    );

    await tester.pumpAndSettle();

    expect(find.text('Build Your Website'), findsOneWidget);
    expect(find.text('Start Now'), findsOneWidget);

    await tester.tap(find.text('Start Now'));
    await tester.pumpAndSettle();

    expect(didNavigate, isTrue);
  });
}
