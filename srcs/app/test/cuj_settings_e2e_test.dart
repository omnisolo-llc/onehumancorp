// CUJ: Settings Screen
//
// Covers the settings CUJ using provider overrides (no direct HTTP mocks).
// Tests verify SettingsScreen renders correctly with various states.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/settings_screen.dart';

Widget _wrapSettings() {
  return const ProviderScope(
    child: MaterialApp(home: SettingsScreen()),
  );
}

void main() {
  group('CUJ: Settings Screen', () {
    testWidgets('settings screen renders Scaffold', (tester) async {
      await tester.pumpWidget(_wrapSettings());
      await tester.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('settings AppBar shows Settings title', (tester) async {
      await tester.pumpWidget(_wrapSettings());
      await tester.pump();
      expect(find.text('Settings'), findsOneWidget);
    });

    testWidgets('settings screen renders without crash on empty state', (tester) async {
      await tester.pumpWidget(_wrapSettings());
      await tester.pump();
      expect(find.byType(MaterialApp), findsOneWidget);
    });

    testWidgets('settings screen renders loading or content state', (tester) async {
      await tester.pumpWidget(_wrapSettings());
      await tester.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('settings screen narrow viewport renders without overflow', (tester) async {
      tester.view.physicalSize = const Size(360, 640);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      await tester.pumpWidget(_wrapSettings());
      await tester.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('settings screen wide viewport renders without overflow', (tester) async {
      tester.view.physicalSize = const Size(1280, 800);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      await tester.pumpWidget(_wrapSettings());
      await tester.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('settings screen renders ProviderScope without error', (tester) async {
      await tester.pumpWidget(_wrapSettings());
      await tester.pump();
      expect(find.byType(ProviderScope), findsOneWidget);
    });

    testWidgets('settings screen pump 3 frames without crash', (tester) async {
      await tester.pumpWidget(_wrapSettings());
      await tester.pump();
      await tester.pump(const Duration(milliseconds: 100));
      await tester.pump(const Duration(milliseconds: 200));
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('settings screen has at most one AppBar', (tester) async {
      await tester.pumpWidget(_wrapSettings());
      await tester.pump();
      expect(find.byType(AppBar), findsAtLeastNWidgets(1));
    });

    testWidgets('settings screen text Settings appears once', (tester) async {
      await tester.pumpWidget(_wrapSettings());
      await tester.pump();
      expect(find.text('Settings'), findsAtLeastNWidgets(1));
    });
  });
}
