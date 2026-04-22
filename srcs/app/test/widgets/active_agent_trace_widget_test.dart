import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/widgets/active_agent_trace_widget.dart';

void main() {
  testWidgets('ActiveAgentTraceWidget renders active agents', (WidgetTester tester) async {
    final activeAgents = [
      Agent(
        id: 'agent-1',
        name: 'Agent One',
        role: 'subagent',
        status: 'running',
        budget: 100,
        model: 'model-a',
      ),
      Agent(
        id: 'agent-2',
        name: 'Agent Two',
        role: 'subagent',
        status: 'stopped',
        budget: 100,
        model: 'model-b',
      ),
      Agent(
        id: 'agent-3',
        name: 'Agent Three',
        role: 'teammate',
        status: 'running',
        budget: 100,
        model: 'model-c',
      ),
    ];

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: ActiveAgentTraceWidget(activeAgents: activeAgents),
        ),
      ),
    );

    // Should find the title
    expect(find.text('Active Agent Traces'), findsOneWidget);

    // Should find the running agents' IDs
    expect(find.text('agent-1'), findsOneWidget);
    expect(find.text('agent-3'), findsOneWidget);

    // Should NOT find the stopped agent's ID
    expect(find.text('agent-2'), findsNothing);
  });

  testWidgets('ActiveAgentTraceWidget renders empty state when no active agents', (WidgetTester tester) async {
    final activeAgents = [
      Agent(
        id: 'agent-2',
        name: 'Agent Two',
        role: 'subagent',
        status: 'stopped',
        budget: 100,
        model: 'model-b',
      ),
    ];

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: ActiveAgentTraceWidget(activeAgents: activeAgents),
        ),
      ),
    );

    expect(find.text('Active Agent Traces'), findsOneWidget);
    expect(find.text('No active agents currently tracing.'), findsOneWidget);
    expect(find.text('agent-2'), findsNothing);
  });
}
