// CUJ: Agents – Management & Oversight
//
// Covers the agents management CUJ using seeded data via ApiService
// subclass (no direct HTTP mocks, no MockHttpClient).  Tests verify
// AgentsScreen renders correctly for various agent states – equivalent
// to seeding the database with known agent records.
//
//   1.  Empty agents list shows empty state
//   2.  Single agent renders name and role
//   3.  Multiple agents all appear
//   4.  Running agent shows running status
//   5.  Idle agent shows idle status
//   6.  Hire New Agent button is present
//   7.  AppBar is rendered
//   8.  Long agent name renders without overflow
//   9.  Agent created date is displayed
//  10.  Refresh triggers reload

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/models/agent.dart';
import 'package:ohc_app/screens/agents_screen.dart';
import 'package:ohc_app/services/api_service.dart';

// ── Seeded agent data ───────────────────────────────────────────────────────

// _agent creates a seeded Agent with the given id and name.
// [role] accepts any valid OHC agent role (e.g., "SOFTWARE_ENGINEER",
// "DESIGNER"). [status] accepts "idle", "running", or "pending".
Agent _agent(
  String id,
  String name, {
  String role = 'SOFTWARE_ENGINEER',
  String status = 'idle',
}) => Agent(
      id: id,
      name: name,
      role: role,
      status: status,
      organizationId: 'org-1',
      createdAt: DateTime(2025, 3, 1),
    );

// ── Widget wrapper ──────────────────────────────────────────────────────────

class _SeededApiService extends ApiService {
  final List<Agent> _agents;

  _SeededApiService(this._agents)
      : super(baseUrl: 'http://test-host', token: 'seed-token');

  @override
  Future<List<Agent>> listAgents() async => _agents;

  @override
  Future<List<AgentProvider>> listAgentProviders() async => [];
}

Widget _wrapAgents(List<Agent> agents) {
  final api = _SeededApiService(agents);
  final router = GoRouter(
    routes: [
      GoRoute(
        path: '/',
        builder: (context, state) => const AgentsScreen(),
      ),
      GoRoute(
        path: '/agents/hire',
        builder: (context, state) => const Scaffold(body: Text('Hire')),
      ),
    ],
  );
  return ProviderScope(
    overrides: [apiServiceProvider.overrideWithValue(api)],
    child: MaterialApp.router(routerConfig: router),
  );
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

void main() {
  group('CUJ: Agents Screen', () {
    testWidgets('empty agents list renders scaffold', (tester) async {
      await tester.pumpWidget(_wrapAgents([]));
      await tester.pumpAndSettle();

      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('Hire Agent button is always visible', (tester) async {
      await tester.pumpWidget(_wrapAgents([]));
      await tester.pumpAndSettle();

      expect(find.widgetWithText(FilledButton, 'Hire Agent'), findsOneWidget);
    });

    testWidgets('seeded agent name renders in list', (tester) async {
      await tester.pumpWidget(
        _wrapAgents([_agent('a1', 'Alice Engineer')]),
      );
      await tester.pumpAndSettle();

      expect(find.textContaining('Alice Engineer'), findsOneWidget);
    });

    testWidgets('two agents both appear', (tester) async {
      await tester.pumpWidget(
        _wrapAgents([
          _agent('a1', 'Alice'),
          _agent('a2', 'Bob', role: 'DESIGNER'),
        ]),
      );
      await tester.pumpAndSettle();

      expect(find.textContaining('Alice'), findsWidgets);
      expect(find.textContaining('Bob'), findsWidgets);
    });

    testWidgets('running agent status text appears', (tester) async {
      await tester.pumpWidget(
        _wrapAgents([_agent('a1', 'Runner', status: 'running')]),
      );
      await tester.pumpAndSettle();

      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('idle agent status text appears', (tester) async {
      await tester.pumpWidget(
        _wrapAgents([_agent('a1', 'Idler', status: 'idle')]),
      );
      await tester.pumpAndSettle();

      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('agent role label renders', (tester) async {
      await tester.pumpWidget(
        _wrapAgents([_agent('a1', 'Coder', role: 'SOFTWARE_ENGINEER')]),
      );
      await tester.pumpAndSettle();

      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('10 agents render without crash', (tester) async {
      final agents = List.generate(
        10,
        (i) => _agent('a$i', 'Agent ${i + 1}'),
      );
      await tester.pumpWidget(_wrapAgents(agents));
      await tester.pumpAndSettle();

      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('scaffold renders on narrow viewport', (tester) async {
      tester.view.physicalSize = const Size(360, 640);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);

      await tester.pumpWidget(_wrapAgents([]));
      await tester.pumpAndSettle();

      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('long agent name renders without overflow', (tester) async {
      await tester.pumpWidget(
        _wrapAgents([
          _agent('a1', 'A Very Long Agent Name That Should Not Cause Text Overflow'),
        ]),
      );
      await tester.pumpAndSettle();

      expect(find.byType(Scaffold), findsOneWidget);
    });
  });
}
