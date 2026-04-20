// CUJ: Meetings Screen (Additional)
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/meetings_screen.dart';
import 'package:ohc_app/services/api_service.dart';

class _SeededMeetingsApi2 extends ApiService {
  _SeededMeetingsApi2() : super(baseUrl: 'http://test-host', token: 'tok');
  @override
  Future<List<Map<String, dynamic>>> listMeetings() async => [
    {'id': 'm1', 'name': 'Alpha', 'created_at': DateTime(2025).toIso8601String(), 'transcript': []},
    {'id': 'm2', 'name': 'Beta', 'created_at': DateTime(2025).toIso8601String(), 'transcript': []},
  ];
}

Widget _wrap() => ProviderScope(
  overrides: [apiServiceProvider.overrideWithValue(_SeededMeetingsApi2())],
  child: const MaterialApp(home: MeetingsScreen()),
);

void main() {
  group('CUJ: Meetings Screen Additional', () {
    testWidgets('renders Scaffold', (t) async { await t.pumpWidget(_wrap()); await t.pumpAndSettle(); expect(find.byType(Scaffold), findsOneWidget); });
    testWidgets('renders AppBar', (t) async { await t.pumpWidget(_wrap()); await t.pumpAndSettle(); expect(find.byType(AppBar), findsAtLeastNWidgets(1)); });
    testWidgets('Alpha room renders', (t) async { await t.pumpWidget(_wrap()); await t.pumpAndSettle(); expect(find.textContaining('Alpha'), findsOneWidget); });
    testWidgets('Beta room renders', (t) async { await t.pumpWidget(_wrap()); await t.pumpAndSettle(); expect(find.textContaining('Beta'), findsOneWidget); });
    testWidgets('narrow viewport', (t) async {
      t.view.physicalSize = const Size(360, 640); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap()); await t.pumpAndSettle(); expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('wide viewport', (t) async {
      t.view.physicalSize = const Size(1280, 800); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap()); await t.pumpAndSettle(); expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('ProviderScope present', (t) async { await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(ProviderScope), findsOneWidget); });
    testWidgets('rebuild no crash', (t) async { await t.pumpWidget(_wrap()); await t.pump(); await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(Scaffold), findsOneWidget); });
    testWidgets('MaterialApp present', (t) async { await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(MaterialApp), findsOneWidget); });
    testWidgets('medium viewport', (t) async {
      t.view.physicalSize = const Size(768, 1024); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(Scaffold), findsOneWidget);
    });
  });
}
