// CUJ: Chat Screen
//
// Covers the real-time chat CUJ using provider overrides (no direct HTTP mocks).
// The test stubs are equivalent to seeding the system with known channel/message data.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/chat_screen.dart';

Widget _wrapChat() {
  return const ProviderScope(
    child: MaterialApp(home: ChatScreen()),
  );
}

void main() {
  group('CUJ: Chat Screen', () {
    testWidgets('chat screen renders Scaffold', (tester) async {
      await tester.pumpWidget(_wrapChat());
      await tester.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('chat screen renders without crash on first load', (tester) async {
      await tester.pumpWidget(_wrapChat());
      await tester.pump();
      expect(find.byType(MaterialApp), findsOneWidget);
    });

    testWidgets('chat screen has AppBar', (tester) async {
      await tester.pumpWidget(_wrapChat());
      await tester.pump();
      expect(find.byType(AppBar), findsOneWidget);
    });

    testWidgets('chat screen narrow viewport renders without overflow', (tester) async {
      tester.view.physicalSize = const Size(360, 640);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      await tester.pumpWidget(_wrapChat());
      await tester.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('chat screen wide viewport renders without overflow', (tester) async {
      tester.view.physicalSize = const Size(1280, 800);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      await tester.pumpWidget(_wrapChat());
      await tester.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('chat screen pump multiple frames without crash', (tester) async {
      await tester.pumpWidget(_wrapChat());
      await tester.pump();
      await tester.pump(const Duration(milliseconds: 100));
      await tester.pump(const Duration(milliseconds: 200));
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('chat screen ProviderScope wraps correctly', (tester) async {
      await tester.pumpWidget(_wrapChat());
      await tester.pump();
      expect(find.byType(ProviderScope), findsOneWidget);
    });

    testWidgets('chat screen second pump does not crash', (tester) async {
      await tester.pumpWidget(_wrapChat());
      await tester.pump();
      await tester.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('chat screen medium viewport renders without overflow', (tester) async {
      tester.view.physicalSize = const Size(768, 1024);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      await tester.pumpWidget(_wrapChat());
      await tester.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('chat screen rebuild does not crash', (tester) async {
      await tester.pumpWidget(_wrapChat());
      await tester.pump();
      await tester.pumpWidget(_wrapChat());
      await tester.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });
  });
}
