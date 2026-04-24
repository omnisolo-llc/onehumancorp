import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/email_verification_screen.dart';

void main() {
  testWidgets('EmailVerificationScreen renders texts and triggers resend', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(
      home: EmailVerificationScreen(),
    ));

    expect(find.text('Verify Your Email'), findsOneWidget);
    expect(find.text('We have sent a verification link to your email address. Please click the link to verify your account.'), findsOneWidget);
    expect(find.text('I have verified my email ->'), findsOneWidget);

    // Check resend button text
    final resendFinder = find.text('Resend Verification Email');
    expect(resendFinder, findsOneWidget);

    // Tap resend
    await tester.tap(resendFinder);
    await tester.pumpAndSettle();

    // Expect snackbar and changed text
    expect(find.text('Verification email resent!'), findsOneWidget);
    expect(find.text('Email Resent'), findsOneWidget);
  });
}
