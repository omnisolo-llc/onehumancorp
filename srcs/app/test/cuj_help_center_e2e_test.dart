
// CUJ: Help Center E2E Navigation and Chat

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/help_center_screen.dart';
import 'package:ohc_app/screens/help_agent_chat_screen.dart';
import 'package:ohc_app/services/centrifuge_service.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:mocktail/mocktail.dart';
import 'dart:async';

class MockCentrifugeService extends Mock implements CentrifugeService {}

// Create a mock auth state provider directly for Riverpod
final mockAuthStateProvider = StateProvider<AsyncValue<AuthUser?>>((ref) => const AsyncValue.data(null));

void main() {
  testWidgets('CUJ: Help Center Renders', (tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(body: HelpCenterScreen()),
        ),
      ),
    );

    await tester.pumpAndSettle();

    expect(find.byType(Scaffold), findsWidgets);
    expect(find.byType(HelpCenterScreen), findsOneWidget);
  });

  testWidgets('CUJ: Help Agent Chat Renders', (tester) async {
    final mockCentrifuge = MockCentrifugeService();

    when(() => mockCentrifuge.connect()).thenAnswer((_) async {});
    when(() => mockCentrifuge.subscribe(any())).thenAnswer((_) => const Stream.empty());
    when(() => mockCentrifuge.unsubscribe(any())).thenAnswer((_) async {});

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          centrifugeServiceProvider.overrideWithValue(mockCentrifuge),
          // We override authStateProvider to just provide a state
        ],
        child: const MaterialApp(
          home: Scaffold(body: HelpAgentChatScreen()),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.byType(HelpAgentChatScreen), findsOneWidget);
  });
}
