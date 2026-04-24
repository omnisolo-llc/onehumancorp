import 'dart:async';
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/swarm_observability_widget.dart';
import 'package:ohc_app/widgets/pulse_animation.dart';

void main() {
  late StreamController<MeshMessage> streamController;

  setUp(() {
    streamController = StreamController<MeshMessage>();
  });

  tearDown(() {
    streamController.close();
  });

  testWidgets('SwarmObservabilityWidget renders listening state initially', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          meshStreamProvider.overrideWith((ref) => streamController.stream),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: SwarmObservabilityWidget(),
          ),
        ),
      ),
    );

    await tester.pump(); // Start listening

    expect(find.text('Listening for swarm activity...'), findsOneWidget);
    expect(find.text('Teammate Mesh Live Feed'), findsOneWidget);
    expect(find.text('Live'), findsOneWidget);
  });

  testWidgets('SwarmObservabilityWidget renders messages from stream', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          meshStreamProvider.overrideWith((ref) => streamController.stream),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: SwarmObservabilityWidget(),
          ),
        ),
      ),
    );

    await tester.pump();

    final msg = MeshMessage('Agent A', 'Started Task', DateTime.now());
    streamController.add(msg);
    await tester.pump(const Duration(milliseconds: 500)); // Wait for message to animate in

    expect(find.text('Agent A'), findsOneWidget);
    expect(find.text('Started Task'), findsOneWidget);
  });

  testWidgets('SwarmObservabilityWidget uses PulseAnimation for priority messages', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          meshStreamProvider.overrideWith((ref) => streamController.stream),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: SwarmObservabilityWidget(),
          ),
        ),
      ),
    );

    await tester.pump();

    final msg = MeshMessage('Agent B', 'CRITICAL Error', DateTime.now());
    streamController.add(msg);
    await tester.pump(const Duration(milliseconds: 500));

    expect(find.text('Agent B'), findsOneWidget);
    expect(find.text('CRITICAL Error'), findsOneWidget);

    expect(find.byType(PulseAnimation), findsOneWidget);
  });
}
