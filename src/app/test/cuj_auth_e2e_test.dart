import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/login_screen.dart';

void main() {
  group('CUJ: Auth - Sign In', () {
    testWidgets('login screen renders', (tester) async {
      await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: LoginScreen())));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });
  });
}
