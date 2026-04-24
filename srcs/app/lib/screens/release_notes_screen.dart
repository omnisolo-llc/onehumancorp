import 'package:flutter/material.dart';

class ReleaseNotesScreen extends StatelessWidget {
  const ReleaseNotesScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('What\'s New', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          _buildReleaseNote(
            context,
            'v1.2.0 - Help Center Added',
            'We have added a brand new help center, interactive walkthroughs, and a floating AI help chat to make getting started even easier.',
          ),
          const SizedBox(height: 16),
          _buildReleaseNote(
            context,
            'v1.1.0 - AutoDream Sync',
            'AutoDream vector embedding pipeline is now active for long-term memory.',
          ),
        ],
      ),
    );
  }

  Widget _buildReleaseNote(BuildContext context, String title, String description) {
    return Card(
      elevation: 2,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(title, style: const TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold)),
            const SizedBox(height: 8),
            Text(description, style: const TextStyle(fontFamily: 'Inter', fontSize: 14)),
          ],
        ),
      ),
    );
  }
}
