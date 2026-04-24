import 'package:flutter/material.dart';

class ReleaseNotesScreen extends StatelessWidget {
  const ReleaseNotesScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("What's New", style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          const Text('Version 1.2.0', style: TextStyle(fontSize: 22, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
          const SizedBox(height: 8),
          const Text('• Introducing the new AI Help Chat!\n• In-app video tutorials added.\n• Interactive walkthroughs for onboarding.', style: TextStyle(fontFamily: 'Inter', height: 1.5)),
          const Divider(height: 32),
          const Text('Version 1.1.0', style: TextStyle(fontSize: 22, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
          const SizedBox(height: 8),
          const Text('• Redesigned Dashboard.\n• Performance improvements for mobile devices.', style: TextStyle(fontFamily: 'Inter', height: 1.5)),
        ],
      ),
    );
  }
}
