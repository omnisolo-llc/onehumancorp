// CUJ: Growth Features
//
// Covers growth critical user journeys:
//   1. Share my business card shows up
//   2. Social Media Drafts show up and can be approved/rejected

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/services/api_service.dart';

class MockHttpClient extends Mock implements http.Client {}

void main() {
  group('CUJ: Growth Features (Share & Embed, Social Auto-Posting)', () {
    Widget wrapScreen(Widget child, ApiService api) {
      return ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(api),
        ],
        child: MaterialApp(home: Scaffold(body: child)),
      );
    }

    testWidgets('Growth features render without crashing', (tester) async {
      final mockClient = MockHttpClient();

      // Mock Dashboard API response
      when(() => mockClient.get(Uri.parse('http://localhost/api/dashboard'), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response(jsonEncode({
                'agents': [],
                'statuses': [],
                'meetings': [],
                'organization': {'members': [], 'roleProfiles': []},
              }), 200));

      // Mock Quota API for GrowthReferralWidget
      when(() => mockClient.get(Uri.parse('http://localhost/api/quota'), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response(jsonEncode({'used': 5, 'max': 100}), 200));

      final api = ApiService(baseUrl: 'http://localhost', token: 'tok', client: mockClient);

      tester.view.physicalSize = const Size(1920, 3000);
      tester.view.devicePixelRatio = 1.0;

      await tester.pumpWidget(wrapScreen(const DashboardScreen(), api));

      // Let the FutureProvider finish without using pumpAndSettle due to infinite animations
      await tester.pump();
      await tester.pump(const Duration(seconds: 2));
      await tester.pump(const Duration(seconds: 2));

      expect(find.textContaining('Share my business', skipOffstage: false), findsWidgets);
      expect(find.textContaining('Draft-for-Review', skipOffstage: false), findsWidgets);

      tester.view.resetPhysicalSize();
      tester.view.resetDevicePixelRatio();
    });
  });
}
