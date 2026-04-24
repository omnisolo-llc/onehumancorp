import 'package:flutter/material.dart';

class ChangelogScreen extends StatelessWidget {
  const ChangelogScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("What's New", style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Release Notes',
              style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
            ),
            const SizedBox(height: 8),
            const Text(
              'See the latest features, improvements, and bug fixes added to your business platform.',
              style: TextStyle(fontSize: 16, fontFamily: 'Inter', color: Colors.grey),
            ),
            const SizedBox(height: 32),
            _ChangelogItem(
              version: 'v1.4.0',
              date: 'October 24, 2026',
              title: 'AI Help Center & Tooltips',
              description: 'We completely overhauled the Help Center to make it easier to find answers. You can now chat directly with our AI Help Agent from any page. We also added helpful tooltips across the dashboard to explain system metrics in plain English.',
            ),
          ],
        ),
      ),
    );
  }
}

class _ChangelogItem extends StatelessWidget {
  final String version;
  final String date;
  final String title;
  final String description;

  const _ChangelogItem({required this.version, required this.date, required this.title, required this.description});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 32.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
                decoration: BoxDecoration(color: Theme.of(context).colorScheme.primaryContainer, borderRadius: BorderRadius.circular(16)),
                child: Text(version, style: TextStyle(color: Theme.of(context).colorScheme.onPrimaryContainer, fontWeight: FontWeight.bold, fontFamily: 'Inter')),
              ),
              const SizedBox(width: 12),
              Text(date, style: const TextStyle(color: Colors.grey, fontFamily: 'Inter', fontSize: 14)),
            ],
          ),
          const SizedBox(height: 12),
          Text(title, style: const TextStyle(fontSize: 20, fontWeight: FontWeight.w600, fontFamily: 'Outfit')),
          const SizedBox(height: 8),
          Text(description, style: const TextStyle(fontSize: 16, height: 1.5, fontFamily: 'Inter')),
          const SizedBox(height: 16),
          const Divider(),
        ],
      ),
    );
  }
}
