import 'dart:ui';
import 'package:flutter/material.dart';

class OnboardingDashboardScreen extends StatelessWidget {
  const OnboardingDashboardScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Day One Setup Audit'),
      ),
      body: Center(
        child: ClipRect(
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            child: Container(
              color: const Color.fromRGBO(255, 255, 255, 0.05),
              padding: const EdgeInsets.all(20),
              child: const Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(
                    'Environment Provisioning',
                    style: TextStyle(fontFamily: 'Outfit', fontSize: 24),
                  ),
                  SizedBox(height: 10),
                  Text(
                    'Status: Active',
                    style: TextStyle(fontFamily: 'Inter', fontSize: 16),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
