import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/chat_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/centrifuge_service.dart';

class MockApiService extends Mock implements ApiService {}
class MockCentrifugeService extends Mock implements CentrifugeService {}

void main() {
  late MockApiService mockApiService;
  late MockCentrifugeService mockCentrifugeService;

  setUp(() {
    mockApiService = MockApiService();
    mockCentrifugeService = MockCentrifugeService();
  });

  Widget buildTestWidget() {
    return ProviderScope(
      overrides: [
        apiServiceProvider.overrideWithValue(mockApiService),
        centrifugeServiceProvider.overrideWithValue(mockCentrifugeService),
      ],
      child: const MaterialApp(
        home: ChatScreen(),
      ),
    );
  }

  testWidgets('renders ChatScreen and triggers share cloud link', (tester) async {
    when(() => mockCentrifugeService.connect()).thenAnswer((_) async {});
    when(() => mockCentrifugeService.disconnect()).thenAnswer((_) async {});
    when(() => mockCentrifugeService.subscribe(any())).thenAnswer((_) => const Stream.empty());

    when(() => mockApiService.createShareLink(any())).thenAnswer(
      (_) async => {'shareCode': 'sh-20261234567'},
    );

    await tester.pumpWidget(buildTestWidget());
    await tester.pumpAndSettle();

    expect(find.text('Chat — #general'), findsOneWidget);

    final shareButton = find.byTooltip('Share via Cloud Link');
    expect(shareButton, findsOneWidget);

    await tester.tap(shareButton);
    await tester.pump();

    // Wait for async functions
    await tester.pump(const Duration(milliseconds: 100));

    // Ensure all microtasks complete
    await tester.pumpAndSettle();

    // Wait for the SnackBar message directly instead of pumpAndSettle
    await tester.pump(const Duration(seconds: 1));

    // Verify API call was made
    verify(() => mockApiService.createShareLink(any())).called(1);
  });
}
