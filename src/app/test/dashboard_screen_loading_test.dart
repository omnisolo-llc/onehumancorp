import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/widgets/shimmer_loading.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:mocktail/mocktail.dart';
import 'dart:async';

class MockApiService extends Mock implements ApiService {}

void main() {
  testWidgets('DashboardScreen shows shimmer skeleton instead of CircularProgressIndicator', (WidgetTester tester) async {
    final mockApi = MockApiService();
    when(() => mockApi.getDashboard()).thenAnswer((_) => Completer<DashboardSnapshot>().future);

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApi),
        ],
        child: const MaterialApp(
          home: DashboardScreen(),
        ),
      ),
    );

    // Initial pump
    await tester.pump();

    // Verify CircularProgressIndicator is NOT present
    expect(find.byType(CircularProgressIndicator), findsNothing);

    // Verify ShimmerLoading IS present
    expect(find.byType(ShimmerLoading), findsWidgets);
  });
}
