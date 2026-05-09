import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'business_setup_wizard_screen.dart';

class LandingScreen extends StatelessWidget {
  const LandingScreen({super.key});

  Future<void> _trackDownload(String os) async {
    try {
      const baseUrl = String.fromEnvironment('API_URL', defaultValue: 'https://api.onehumancorp.com');
      await http.post(
        Uri.parse('$baseUrl/api/growth/downloads'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({'os': os, 'version': '1.0.0'}),
      );
    } catch (e) {
      debugPrint('Error tracking download: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Text(
              'Welcome to One Human Corp',
              style: TextStyle(fontSize: 32, fontWeight: FontWeight.bold, color: Colors.white),
            ),
            const SizedBox(height: 40),
            ElevatedButton(
              key: const Key('downloadMacBtn'),
              onPressed: () => _trackDownload('Mac'),
              style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFF6B4EFF)),
              child: const Text('Download for Mac', style: TextStyle(color: Colors.white)),
            ),
            const SizedBox(height: 10),
            ElevatedButton(
              key: const Key('downloadWindowsBtn'),
              onPressed: () => _trackDownload('Windows'),
              style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFF6B4EFF)),
              child: const Text('Download for Windows', style: TextStyle(color: Colors.white)),
            ),
            const SizedBox(height: 10),
            ElevatedButton(
              key: const Key('downloadLinuxBtn'),
              onPressed: () => _trackDownload('Linux'),
              style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFF6B4EFF)),
              child: const Text('Download for Linux', style: TextStyle(color: Colors.white)),
            ),
            const SizedBox(height: 30),
            TextButton(
              key: const Key('continueSetupBtn'),
              onPressed: () {
                Navigator.pushReplacement(
                  context,
                  MaterialPageRoute(builder: (context) => const ProviderScope(child: BusinessSetupWizardScreen())),
                );
              },
              child: const Text('Continue to Setup', style: TextStyle(color: Colors.white70)),
            ),
          ],
        ),
      ),
    );
  }
}
