import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'dart:ui';

class EmailVerificationScreen extends StatefulWidget {
  const EmailVerificationScreen({super.key});

  @override
  State<EmailVerificationScreen> createState() => _EmailVerificationScreenState();
}

class _EmailVerificationScreenState extends State<EmailVerificationScreen> {
  bool _resendSent = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF0D0D1A), Color(0xFF1A1A33)],
          ),
        ),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 400),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(24),
              child: BackdropFilter(
                filter: ImageFilter.compose(
                  outer: ColorFilter.matrix(const <double>[
                    1.787, -0.715, -0.072, 0, 0,
                    -0.213, 1.285, -0.072, 0, 0,
                    -0.213, -0.715, 1.928, 0, 0,
                    0, 0, 0, 1, 0,
                  ]),
                  inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                ),
                child: Container(
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.surface.withOpacity(0.6),
                    borderRadius: BorderRadius.circular(24),
                    border: Border.all(
                      color: Theme.of(context).colorScheme.outlineVariant.withOpacity(0.3),
                    ),
                  ),
                  child: Padding(
                    padding: const EdgeInsets.all(32),
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: [
                        const Icon(Icons.mark_email_unread_outlined, size: 64, color: Colors.blueAccent),
                        const SizedBox(height: 24),
                        const Text(
                          'Verify Your Email',
                          textAlign: TextAlign.center,
                          style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit', color: Colors.white),
                        ),
                        const SizedBox(height: 16),
                        const Text(
                          'We have sent a verification link to your email address. Please click the link to verify your account.',
                          textAlign: TextAlign.center,
                          style: TextStyle(fontFamily: 'Inter', color: Colors.white70),
                        ),
                        const SizedBox(height: 32),
                        ElevatedButton(
                          onPressed: () {
                            context.go('/business_setup');
                          },
                          child: const Text('I have verified my email ->', style: TextStyle(fontFamily: 'Inter')),
                        ),
                        const SizedBox(height: 16),
                        TextButton(
                          onPressed: _resendSent ? null : () {
                            setState(() { _resendSent = true; });
                            ScaffoldMessenger.of(context).showSnackBar(
                              const SnackBar(content: Text('Verification email resent!')),
                            );
                          },
                          child: Text(
                            _resendSent ? 'Email Resent' : 'Resend Verification Email',
                            style: const TextStyle(fontFamily: 'Inter', color: Colors.blueAccent),
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
