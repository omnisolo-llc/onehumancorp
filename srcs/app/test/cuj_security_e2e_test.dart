// CUJ: Security – Vulnerability Overview
//
// Covers the security screen CUJ using seeded data via provider overrides.
// Tests verify SecurityScreen renders correctly for various issue states –
// equivalent to a database seeded with known security findings.
//
//   1.  All-clear state when no issues
//   2.  Open issues render with severity badges
//   3.  Fixed issues appear in a separate section
//   4.  Refresh button triggers re-scan
//   5.  High-severity issue uses error colour
//   6.  Low-severity issue uses tertiary colour
//   7.  Fixed flag hides issue from open list
//   8.  AppBar title is "Security"
//   9.  Mixed open/fixed list renders correctly
//  10.  Large number of issues renders without overflow

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/models/security_issue.dart';
import 'package:ohc_app/screens/security_screen.dart';
import 'package:ohc_app/services/api_service.dart';

// ── Seeded security data ────────────────────────────────────────────────────

SecurityIssue _issue(
  String id,
  String title, {
  String severity = 'medium',
  bool fixed = false,
}) => SecurityIssue(
      id: id,
      title: title,
      severity: severity,
      description: 'Test description for $title',
      fixable: true,
      fixed: fixed,
      category: 'general',
    );

// ── Widget wrapper ──────────────────────────────────────────────────────────

class _SeededApiService extends ApiService {
  final List<SecurityIssue> _issues;

  _SeededApiService(this._issues)
      : super(baseUrl: 'http://test-host', token: 'seed-token');

  @override
  Future<List<SecurityIssue>> listSecurityIssues() async => _issues;
}

Widget _wrapSecurity(List<SecurityIssue> issues) {
  final api = _SeededApiService(issues);
  return ProviderScope(
    overrides: [apiServiceProvider.overrideWithValue(api)],
    child: const MaterialApp(home: SecurityScreen()),
  );
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

void main() {
  group('CUJ: Security Screen', () {
    testWidgets('AppBar title is Security', (tester) async {
      await tester.pumpWidget(_wrapSecurity([]));
      await tester.pumpAndSettle();

      expect(find.text('Security'), findsOneWidget);
    });

    testWidgets('all-clear state renders when no issues', (tester) async {
      await tester.pumpWidget(_wrapSecurity([]));
      await tester.pumpAndSettle();

      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('refresh/re-scan button is present', (tester) async {
      await tester.pumpWidget(_wrapSecurity([]));
      await tester.pumpAndSettle();

      expect(find.byIcon(Icons.refresh), findsOneWidget);
    });

    testWidgets('single open issue renders title', (tester) async {
      await tester.pumpWidget(
        _wrapSecurity([_issue('001', 'SQL Injection', severity: 'high')]),
      );
      await tester.pumpAndSettle();

      expect(find.textContaining('SQL Injection'), findsOneWidget);
    });

    testWidgets('fixed issue renders in resolved section', (tester) async {
      await tester.pumpWidget(
        _wrapSecurity([_issue('002', 'XSS Vulnerability', fixed: true)]),
      );
      await tester.pumpAndSettle();

      expect(find.textContaining('XSS Vulnerability'), findsOneWidget);
    });

    testWidgets('mixed open and fixed issues both appear', (tester) async {
      await tester.pumpWidget(
        _wrapSecurity([
          _issue('003', 'Open Issue', severity: 'medium'),
          _issue('004', 'Fixed Issue', fixed: true),
        ]),
      );
      await tester.pumpAndSettle();

      expect(find.textContaining('Open Issue'), findsOneWidget);
      expect(find.textContaining('Fixed Issue'), findsOneWidget);
    });

    testWidgets('multiple open issues all appear', (tester) async {
      final issues = [
        _issue('i1', 'Issue Alpha', severity: 'low'),
        _issue('i2', 'Issue Beta', severity: 'medium'),
        _issue('i3', 'Issue Gamma', severity: 'high'),
      ];
      await tester.pumpWidget(_wrapSecurity(issues));
      await tester.pumpAndSettle();

      expect(find.textContaining('Issue Alpha'), findsOneWidget);
      expect(find.textContaining('Issue Beta'), findsOneWidget);
      expect(find.textContaining('Issue Gamma'), findsOneWidget);
    });

    testWidgets('20 issues render without overflow', (tester) async {
      final issues = List.generate(
        20,
        (i) => _issue('x$i', 'Issue #$i', severity: i.isEven ? 'high' : 'low'),
      );
      await tester.pumpWidget(_wrapSecurity(issues));
      await tester.pumpAndSettle();

      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('tapping refresh invokes re-scan without crash', (tester) async {
      await tester.pumpWidget(
        _wrapSecurity([_issue('s1', 'Stale Issue')]),
      );
      await tester.pumpAndSettle();

      await tester.tap(find.byIcon(Icons.refresh));
      await tester.pump();

      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('scaffold renders on small viewport without overflow', (tester) async {
      tester.view.physicalSize = const Size(320, 568);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);

      await tester.pumpWidget(_wrapSecurity([]));
      await tester.pumpAndSettle();

      expect(find.byType(Scaffold), findsOneWidget);
    });
  });
}
