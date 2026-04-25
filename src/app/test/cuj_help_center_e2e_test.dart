import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/main.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:go_router/go_router.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  testWidgets('E2E: User navigates to Help Center and asks a question', (WidgetTester tester) async {
    final mockApi = MockApiService();

    // Setup mock data
    when(() => mockApi.getDashboard()).thenAnswer((_) async => DashboardSnapshot.fromJson({
      "organization": {"id": "1", "name": "Org 1", "industry": "retail"},
      "costs": {"total_cost": 10.0, "currency": "USD", "items": []},
      "agents": [],
      "statuses": [],
      "meetings": [],
      "updatedAt": DateTime.now().toIso8601String(),
    }));

    when(() => mockApi.listAiProviders()).thenAnswer((_) async => []);

    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApi),
        ],
        child: const OhcApp(),
      ),
    );

    await tester.pumpAndSettle();

    if (find.text('Login to Workspace').evaluate().isNotEmpty) {
        await tester.enterText(find.byType(TextField).first, 'test@example.com');
        await tester.enterText(find.byType(TextField).last, 'password');
        await tester.tap(find.text('Login to Workspace'));
        await tester.pumpAndSettle();
    }

    await tester.pumpAndSettle(const Duration(seconds: 2));

    // Get a valid context for GoRouter
    final BuildContext context = tester.element(find.byType(Scaffold).first);
    GoRouter.of(context).go('/help-center');
    await tester.pumpAndSettle();

    expect(find.text('How can we help you run your business today?'), findsOneWidget);

    final askAnythingFinder = find.text('Ask anything');
    expect(askAnythingFinder, findsOneWidget);

    await tester.ensureVisible(askAnythingFinder);
    await tester.tap(askAnythingFinder);
    await tester.pumpAndSettle();

    expect(find.text('How can we help you run your business today?'), findsNothing);

    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });
}
