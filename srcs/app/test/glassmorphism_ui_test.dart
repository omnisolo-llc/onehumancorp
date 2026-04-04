import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/models/handoff.dart';
import 'package:ohc_app/models/channel.dart';
import 'package:ohc_app/screens/handoffs_screen.dart';
import 'package:ohc_app/screens/channels_screen.dart';
import 'package:ohc_app/services/api_service.dart';

// Mock Api Service
class MockApiService implements ApiService {
  @override
  Future<List<HandoffPackage>> listHandoffs() async {
    return [
      HandoffPackage(
        id: '1',
        fromAgentId: 'agent-x',
        intent: 'escalation',
        currentState: 'stuck',
        toHumanRole: 'operator',
        failedAttempts: 0,
        status: 'pending',
        createdAt: DateTime.now(),
      )
    ];
  }

  @override
  Future<void> resolveHandoff(String handoffId, String resolution) async {}

  @override
  Future<List<ChatChannel>> listChannels() async {
    return [
      ChatChannel(
        id: '1',
        name: 'test-channel',
        organizationId: 'org-1',
        backend: ChatBackend(type: ChatBackendType.webhook),
        config: {},
        enabled: true,
        createdAt: DateTime.now(),
      )
    ];
  }

  @override
  Future<ChatChannel> addChannel({required String name, required String backend, required Map<String, dynamic> config}) async {
    return ChatChannel(
        id: '1',
        name: name,
        organizationId: 'org-1',
        backend: ChatBackend(type: ChatBackendType.webhook),
        config: {},
        enabled: true,
        createdAt: DateTime.now(),
    );
  }

  // Implement other methods as no-ops if required by interface
  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

void main() {
  testWidgets('HandoffsScreen uses Glassmorphism tokens', (tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(MockApiService()),
        ],
        child: const MaterialApp(
          home: HandoffsScreen(),
        ),
      ),
    );

    // Wait for FutureBuilder
    await tester.pumpAndSettle();

    // Verify BackdropFilter is used
    expect(find.byType(BackdropFilter), findsWidgets);

    // Verify Flat Cards are removed
    expect(find.byType(Card), findsNothing);
  });

  testWidgets('ChannelsScreen uses Glassmorphism tokens', (tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(MockApiService()),
        ],
        child: const MaterialApp(
          home: ChannelsScreen(),
        ),
      ),
    );

    // Wait for FutureBuilder
    await tester.pumpAndSettle();

    // Verify BackdropFilter is used
    expect(find.byType(BackdropFilter), findsWidgets);

    // Verify Flat Cards are removed
    expect(find.byType(Card), findsNothing);
  });
}
