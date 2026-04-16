import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/widgets/growth_referral_widget.dart';
import 'package:ohc_app/services/api_service.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  late MockApiService mockApiService;

  setUpAll(() {
    registerFallbackValue(<String>[]);
  });

  setUp(() {
    mockApiService = MockApiService();
  });

  Widget buildTestWidget() {
    return ProviderScope(
      overrides: [
        apiServiceProvider.overrideWithValue(mockApiService),
      ],
      child: const MaterialApp(
        home: Scaffold(
          body: SingleChildScrollView(
            child: GrowthReferralWidget(),
          ),
        ),
      ),
    );
  }

  testWidgets('displays widget and quota', (tester) async {
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    when(() => mockApiService.getQuota()).thenAnswer((_) async => {'used': 5, 'max': 100});

    await tester.pumpWidget(buildTestWidget());
    await tester.pumpAndSettle();

    expect(find.text('Grow Your Swarm. Maintain Sovereignty.'), findsOneWidget);
    expect(find.text('5 / 100 missions used'), findsOneWidget);
    expect(find.text('Invite Team to Expand Quota'), findsOneWidget);
    expect(find.text('Bulk Invite'), findsOneWidget);
  });

  testWidgets('opens bulk invite dialog', (tester) async {
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    when(() => mockApiService.getQuota()).thenAnswer((_) async => {'used': 5, 'max': 100});

    await tester.pumpWidget(buildTestWidget());
    await tester.pumpAndSettle();

    await tester.tap(find.text('Bulk Invite'));
    await tester.pumpAndSettle();

    expect(find.text('Bulk Invite Team'), findsOneWidget);
    expect(find.byType(TextField), findsOneWidget);
  });

  testWidgets('validates emails and calls bulkInviteTeam', (tester) async {
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    when(() => mockApiService.getQuota()).thenAnswer((_) async => {'used': 5, 'max': 100});
    when(() => mockApiService.bulkInviteTeam(any(), any())).thenAnswer((_) async {});

    await tester.pumpWidget(buildTestWidget());
    await tester.pumpAndSettle();

    await tester.tap(find.text('Bulk Invite'));
    await tester.pumpAndSettle();

    await tester.enterText(find.byType(TextField), 'test1@example.com, invalid_email, test2@example.com\ntest3@example.com');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Send Invites'));
    await tester.pumpAndSettle();

    verify(() => mockApiService.bulkInviteTeam(
      "xYz8vQ_local_sovereign",
      any(that: isA<List<String>>().having(
        (list) => list,
        'list',
        containsAllInOrder(['test1@example.com', 'test2@example.com', 'test3@example.com'])
      )),
    )).called(1);
    expect(find.textContaining('Successfully invited 3 team members!'), findsOneWidget);
  });
}
