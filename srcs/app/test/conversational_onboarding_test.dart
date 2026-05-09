import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/screens/conversational_onboarding_screen.dart';

import 'package:mockito/mockito.dart';
import 'package:app/services/api_service.dart';

// Create a simple manual mock to avoid build_runner dependency for now
class MockApiService extends Mock implements ApiService {
  @override
  Future<Map<String, dynamic>?> sendChatPrompt(String message) async {
    return {
      'name': 'Custom Bakery',
      'category': 'Food & Beverage',
      'description': 'I bake custom cakes',
    };
  }
}

void main() {
  testWidgets('ConversationalOnboardingScreen renders chat interface', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: ConversationalOnboardingScreen(),
        ),
      ),
    );

    expect(find.text('Start Your Business'), findsOneWidget);
    expect(find.text('Hi! I am The Promoter. What kind of business are you starting?'), findsOneWidget);
    expect(find.byKey(const Key('chatInput')), findsOneWidget);
    expect(find.byKey(const Key('chatSendBtn')), findsOneWidget);
  });
}
