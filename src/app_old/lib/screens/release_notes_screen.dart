import 'package:flutter/material.dart';

class ReleaseNotesScreen extends StatelessWidget {
  const ReleaseNotesScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("What's New", style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: const [
            Text('Version 2.4.0', style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold)),
            SizedBox(height: 16),
            Text('• New AI Help Center: Get instant answers from our AI.', style: TextStyle(fontSize: 16)),
            Text('• Bug fixes and performance improvements.', style: TextStyle(fontSize: 16)),
          ],
        ),
      ),
    );
  }
}
