import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/widgets/swarm_context_overlay.dart';

void main() {
  testWidgets('SwarmContextOverlay renders title and agents', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SwarmContextOverlay(
            title: 'Active Agents',
            agents: ['Palette', 'Guide'],
            child: const Center(child: Text('Background Child')),
          ),
        ),
      ),
    );

    expect(find.text('Active Agents'), findsOneWidget);
    expect(find.text('Palette'), findsOneWidget);
    expect(find.text('Guide'), findsOneWidget);
    expect(find.text('Background Child'), findsOneWidget);
  });
}
