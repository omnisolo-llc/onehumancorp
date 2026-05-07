import 'package:flutter/material.dart';
import '../main.dart'; // For GlassContainer
import 'business_setup_wizard_screen.dart';

class LoginScreen extends StatefulWidget {
  const LoginScreen({super.key});

  @override
  State<LoginScreen> createState() => _LoginScreenState();
}

class _LoginScreenState extends State<LoginScreen> {
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();
  bool _showVerification = false;

  void _login() {
    Navigator.of(context).pushReplacement(
      MaterialPageRoute(builder: (context) => const BusinessSetupWizardScreen()),
    );
  }

  void _ssoLogin(String provider) {
    setState(() {
      _showVerification = true;
    });
  }

  void _resendVerification() {
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Verification email resent!')),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 400),
          child: SingleChildScrollView(
            child: Padding(
              padding: const EdgeInsets.all(20),
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  const Text(
                    'One Human Corp',
                    style: TextStyle(fontFamily: 'Outfit', fontSize: 32, fontWeight: FontWeight.bold, color: Colors.white),
                    textAlign: TextAlign.center,
                  ),
                  const SizedBox(height: 30),
                  GlassContainer(
                    child: Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: TextField(
                        controller: _emailController,
                        style: const TextStyle(color: Colors.white),
                        decoration: const InputDecoration(labelText: 'Email', labelStyle: TextStyle(color: Colors.white70), border: InputBorder.none),
                      ),
                    ),
                  ),
                  const SizedBox(height: 15),
                  GlassContainer(
                    child: Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: TextField(
                        controller: _passwordController,
                        obscureText: true,
                        style: const TextStyle(color: Colors.white),
                        decoration: const InputDecoration(labelText: 'Password', labelStyle: TextStyle(color: Colors.white70), border: InputBorder.none),
                      ),
                    ),
                  ),
                  if (_showVerification) ...[
                    const SizedBox(height: 15),
                    const Text('Please check your email to verify your account.', style: TextStyle(color: Colors.greenAccent), textAlign: TextAlign.center),
                    TextButton(
                      onPressed: _resendVerification,
                      child: const Text('Resend Verification Email', style: TextStyle(color: Colors.blueAccent)),
                    ),
                    ElevatedButton(
                      onPressed: _login,
                      style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFF6B4EFF), padding: const EdgeInsets.symmetric(vertical: 20)),
                      child: const Text('Continue to Setup', style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold)),
                    ),
                  ] else ...[
                    const SizedBox(height: 30),
                    ElevatedButton(
                      onPressed: _login,
                      style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFF6B4EFF), padding: const EdgeInsets.symmetric(vertical: 20)),
                      child: const Text('Sign In', style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold)),
                    ),
                    const SizedBox(height: 15),
                    const Text('OR', style: TextStyle(color: Colors.white54, fontWeight: FontWeight.bold), textAlign: TextAlign.center),
                    const SizedBox(height: 15),
                    ElevatedButton(
                      onPressed: () => _ssoLogin('Google'),
                      style: ElevatedButton.styleFrom(backgroundColor: Colors.white10, padding: const EdgeInsets.symmetric(vertical: 20)),
                      child: const Text('Use Google or Apple', style: TextStyle(color: Colors.white)),
                    ),
                  ],
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
