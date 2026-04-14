import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/swarm_observability_widget.dart';

void main() {
  testWidgets('SwarmObservabilityWidget displays incoming messages', (WidgetTester tester) async {
    final streamController = StreamController<MeshMessage>.broadcast();

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

    // Initial state: waiting for messages
    await tester.pump();
    expect(find.text('Listening for swarm activity...'), findsOneWidget);

    // Add a new message
    streamController.add(MeshMessage('Agent X', 'Task Claimed', DateTime.now()));

    // We only pump the frame, not pumpAndSettle, because of the continuous _PulsingStatusIndicator animation
    await tester.pump(const Duration(milliseconds: 500));

    expect(find.text('Listening for swarm activity...'), findsNothing);
    expect(find.text('Agent X'), findsOneWidget);
    expect(find.text('Task Claimed'), findsOneWidget);

    streamController.close();
  });
}
