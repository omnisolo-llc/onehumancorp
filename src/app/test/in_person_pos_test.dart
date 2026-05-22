import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';

import '../lib/screens/in_person_pos.dart';
import '../lib/screens/agent_dashboard.dart';
import '../lib/main.dart';

void main() {
  setUp(() {
    SharedPreferences.setMockInitialValues({'auth_token': 'test_token'});
  });

  testWidgets('1) Dashboard has a floating + button to open POS', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: AgentDashboard()));
    await tester.pumpAndSettle();

    final fab = find.byType(FloatingActionButton);
    expect(fab, findsOneWidget);
    expect(find.byIcon(Icons.add), findsOneWidget);

    await tester.tap(fab);
    await tester.pumpAndSettle();

    expect(find.text('In-Person Sale'), findsOneWidget);
  });

  testWidgets('2) POS Screen allows adding a custom amount to cart', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: InPersonPosScreen()));

    final textField = find.byType(TextField);
    expect(textField, findsOneWidget);

    await tester.enterText(textField, '150');
    await tester.tap(find.text('Add'));
    await tester.pump();

    expect(find.text('Custom Amount'), findsOneWidget);
    expect(find.text('\$150.00'), findsNWidgets(2)); // Once in list, once in total
  });

  testWidgets('3) POS Screen shows Tap to Pay modal on Checkout', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: InPersonPosScreen()));

    await tester.enterText(find.byType(TextField), '150');
    await tester.tap(find.text('Add'));
    await tester.pump();

    await tester.tap(find.text('Checkout (Tap to Pay)'));
    await tester.pump(); // Start dialog

    expect(find.text('Present card to device'), findsOneWidget);
    expect(find.text('\$150.00'), findsWidgets);

    // Fast forward to complete the timer to avoid unhandled pending timers exception
    await tester.pumpAndSettle(Duration(seconds: 2));
  });

  testWidgets('4) POS Screen shows success after mock payment delay', (WidgetTester tester) async {
    // We use a custom http client to mock the API emit so we don't hit real network
    final mockClient = MockClient((request) async {
      return http.Response('{}', 200);
    });

    await tester.pumpWidget(MaterialApp(home: InPersonPosScreen(httpClient: mockClient)));

    await tester.enterText(find.byType(TextField), '150');
    await tester.tap(find.text('Add'));
    await tester.pump();

    await tester.tap(find.text('Checkout (Tap to Pay)'));
    await tester.pump(); // Open dialog

    // Fast forward through the 2 second fake Stripe SDK delay
    await tester.pumpAndSettle(Duration(seconds: 2));

    expect(find.text('Payment Successful'), findsOneWidget);
    expect(find.text('\$150.00 has been recorded and inventory updated.'), findsOneWidget);
    expect(find.text('Email/SMS Receipt'), findsOneWidget);
  });

  testWidgets('5) POS Screen correctly emits OfflinePaymentCompleted event via API', (WidgetTester tester) async {
    bool apiCalled = false;
    double? receivedAmount;
    String? receivedEventType;

    final mockClient = MockClient((request) async {
      if (request.url.path.endsWith('/api/events')) {
        apiCalled = true;
        final body = jsonDecode(request.body);
        receivedEventType = body['event_type'];
        receivedAmount = (body['amount'] as num).toDouble();
      }
      return http.Response('{}', 200);
    });

    await tester.pumpWidget(MaterialApp(home: InPersonPosScreen(httpClient: mockClient)));

    await tester.enterText(find.byType(TextField), '150');
    await tester.tap(find.text('Add'));
    await tester.pump();

    await tester.tap(find.text('Checkout (Tap to Pay)'));
    await tester.pump();

    // Fast forward to complete the flow
    await tester.pumpAndSettle(Duration(seconds: 2));

    expect(apiCalled, isTrue);
    expect(receivedEventType, 'OfflinePaymentCompleted');
    expect(receivedAmount, 150.0);
  });
}
