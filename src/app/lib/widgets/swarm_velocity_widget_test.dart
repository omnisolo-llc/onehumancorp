import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/swarm_velocity_widget.dart';
import 'package:ohc_app/widgets/swarm_observability_widget.dart';

void main() {
  late StreamController<MeshMessage> streamController;

  setUp(() {
    streamController = StreamController<MeshMessage>();
  });

  tearDown(() {
    streamController.close();
  });

  testWidgets('SwarmVelocityWidget renders initial metrics', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          meshStreamProvider.overrideWith((ref) => streamController.stream),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: SwarmVelocityWidget(),
          ),
        ),
      ),
    );

    await tester.pump(); // Start listening

    expect(find.text('Swarm Velocity'), findsOneWidget);
    expect(find.text('0/min'), findsOneWidget);
    expect(find.text('0 ms'), findsOneWidget);
    expect(find.text('0'), findsOneWidget);
  });

  testWidgets('SwarmVelocityWidget updates metrics on message', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          meshStreamProvider.overrideWith((ref) => streamController.stream),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: SwarmVelocityWidget(),
          ),
        ),
      ),
    );

    await tester.pump();

    final msg = MeshMessage('Agent A', 'completed task', DateTime.now());
    streamController.add(msg);
    await tester.pump(const Duration(milliseconds: 500)); // Process stream and state update

    expect(find.text('1/min'), findsOneWidget);
    expect(find.text('121 ms'), findsOneWidget); // 120.0 + (1 % 50) = 121
    expect(find.text('1'), findsOneWidget); // Active threads
  });
}
