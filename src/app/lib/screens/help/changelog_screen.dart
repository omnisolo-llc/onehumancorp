import 'package:flutter/material.dart';

class ChangelogScreen extends StatelessWidget {
  const ChangelogScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('What\'s New', style: TextStyle(fontFamily: 'Outfit')),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 800),
          child: ListView(
            padding: const EdgeInsets.all(24),
            children: [
              _buildReleaseNote(
                context,
                version: 'v1.2.0',
                date: 'April 2026',
                title: 'New AI Help Center',
                description: 'We\'ve added a brand new help center and an AI assistant that can answer your questions immediately from anywhere in the app!',
              ),
              const SizedBox(height: 24),
              _buildReleaseNote(
                context,
                version: 'v1.1.5',
                date: 'March 2026',
                title: 'Better Inventory Management',
                description: 'You can now bulk-update your product inventory and set low-stock alerts right from the dashboard.',
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildReleaseNote(BuildContext context, {required String version, required String date, required String title, required String description}) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.primaryContainer,
                borderRadius: BorderRadius.circular(16),
              ),
              child: Text(
                version,
                style: TextStyle(color: Theme.of(context).colorScheme.onPrimaryContainer, fontWeight: FontWeight.bold),
              ),
            ),
            const SizedBox(width: 12),
            Text(date, style: const TextStyle(color: Colors.grey, fontFamily: 'Inter')),
          ],
        ),
        const SizedBox(height: 12),
        Text(
          title,
          style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
        ),
        const SizedBox(height: 8),
        Text(
          description,
          style: const TextStyle(fontSize: 16, fontFamily: 'Inter'),
        ),
        const Divider(height: 32),
      ],
    );
  }
}
