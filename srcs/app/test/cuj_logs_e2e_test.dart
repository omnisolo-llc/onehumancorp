// CUJ: Logs Screen
//
// Covers the logs CUJ using seeded ApiService subclass (no direct HTTP mocks).

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/logs_screen.dart';
import 'package:ohc_app/services/api_service.dart';

class _SeededLogsApiService extends ApiService {
  final List<String> _lines;
  _SeededLogsApiService(this._lines)
      : super(baseUrl: 'http://test-host', token: 'seed-token');

  @override
  Future<List<String>> getLogs({int lines = 100}) async => _lines;
}

Widget _wrapLogs(List<String> lines) {
  final api = _SeededLogsApiService(lines);
  return ProviderScope(
    overrides: [apiServiceProvider.overrideWithValue(api)],
    child: const MaterialApp(home: LogsScreen()),
  );
}

void main() {
  group('CUJ: Logs Screen', () {
    testWidgets('logs screen renders Scaffold', (tester) async {
      await tester.pumpWidget(_wrapLogs([]));
      await tester.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('logs AppBar title contains Logs', (tester) async {
      await tester.pumpWidget(_wrapLogs([]));
      await tester.pump();
      expect(find.textContaining('Log'), findsAtLeastNWidgets(1));
    });

    testWidgets('empty logs renders without crash', (tester) async {
      await tester.pumpWidget(_wrapLogs([]));
      await tester.pump();
      expect(find.byType(MaterialApp), findsOneWidget);
    });

    testWidgets('single log line seeded renders Scaffold', (tester) async {
      await tester.pumpWidget(_wrapLogs(['INFO server started']));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('multiple log lines seeded renders without crash', (tester) async {
      final lines = List.generate(20, (i) => 'INFO line $i');
      await tester.pumpWidget(_wrapLogs(lines));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('logs screen narrow viewport renders without overflow', (tester) async {
      tester.view.physicalSize = const Size(360, 640);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      await tester.pumpWidget(_wrapLogs([]));
      await tester.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('logs screen 3 pumps without crash', (tester) async {
      await tester.pumpWidget(_wrapLogs([]));
      await tester.pump();
      await tester.pump(const Duration(milliseconds: 100));
      await tester.pump(const Duration(milliseconds: 200));
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('logs screen error log line renders Scaffold', (tester) async {
      await tester.pumpWidget(_wrapLogs(['ERROR something failed']));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('logs screen warn log line renders Scaffold', (tester) async {
      await tester.pumpWidget(_wrapLogs(['WARN low memory']));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('logs screen 100 log lines renders without crash', (tester) async {
      final lines = List.generate(100, (i) => 'INFO log line $i');
      await tester.pumpWidget(_wrapLogs(lines));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });
  });
}
