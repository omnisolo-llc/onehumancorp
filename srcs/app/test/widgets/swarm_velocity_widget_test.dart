import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import '../../lib/widgets/swarm_velocity_widget.dart';

void main() {
  testWidgets('SwarmVelocityWidget renders correctly with OHC tokens', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: SwarmVelocityWidget(),
          ),
        ),
      ),
    );

    expect(find.text('Swarm Velocity'), findsOneWidget);
    expect(find.text('Task Completion Rate'), findsOneWidget);
    expect(find.text('Average Latency'), findsOneWidget);
    expect(find.text('Active Threads'), findsOneWidget);

    final containerFinder = find.descendant(
      of: find.byType(BackdropFilter),
      matching: find.byType(Container).first,
    );

    final container = tester.widget<Container>(containerFinder);
    final decoration = container.decoration as BoxDecoration;
    expect(decoration.color, const Color.fromRGBO(255, 255, 255, 0.03));
  });
}
