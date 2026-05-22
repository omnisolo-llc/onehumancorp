import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import '../lib/screens/inbox.dart';
import 'dart:convert';
import 'package:http/testing.dart';

void main() {
  testWidgets('1) InboxScreen shows loading indicator initially', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: InboxScreen()));
    expect(find.byType(CircularProgressIndicator), findsOneWidget);
  });

  testWidgets('2) InboxScreen has Unified Inbox title', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: InboxScreen()));
    expect(find.text('Unified Inbox'), findsOneWidget);
  });

  testWidgets('3) InboxScreen shows No messages when list is empty after loading', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: InboxScreen()));
    await tester.pumpAndSettle();
    expect(find.text('No messages.'), findsOneWidget);
  });

  testWidgets('4) InboxScreen contains AppBar and layout builder', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: InboxScreen()));
    expect(find.byType(AppBar), findsOneWidget);
    expect(find.byType(LayoutBuilder), findsOneWidget);
  });

  testWidgets('5) InboxScreen sets background color properly', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: InboxScreen()));
    final Scaffold scaffold = tester.widget(find.byType(Scaffold));
    expect(scaffold.backgroundColor, Color(0xFFF5F5F7));
  });
}
