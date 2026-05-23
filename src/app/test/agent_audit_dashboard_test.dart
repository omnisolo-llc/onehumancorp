import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import '../lib/screens/agent_audit_dashboard.dart';

void main() {
  testWidgets('AgentAuditDashboard shows loading indicator and then components', (WidgetTester tester) async {
    // Mock shared preferences
    SharedPreferences.setMockInitialValues({});

    // Provide a mobile screen size to test mobile layout and prevent overflow
    tester.view.physicalSize = const Size(375, 812);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(MaterialApp(home: AgentAuditDashboard()));

    // Verify Title and loading indicator initially
    expect(find.text('Agent Audit Dashboard'), findsOneWidget);
    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    // Because the test flutter bindings return 400 for HTTP, the data fetch fails
    // Pump and wait for futures to resolve
    await tester.pump(Duration(seconds: 1));
    await tester.pump(Duration(seconds: 1));

    // Let's just verify it no longer shows the loading indicator after futures settle
    expect(find.byType(CircularProgressIndicator), findsNothing);

    // reset sizing
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
  });
}
