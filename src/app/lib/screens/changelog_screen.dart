import 'package:flutter/material.dart';

class ChangelogScreen extends StatelessWidget {
  const ChangelogScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("What's New"),
      ),
      body: ListView(
        padding: const EdgeInsets.all(16.0),
        children: [
          _buildReleaseItem(
            'Version 2.1.0',
            'May 25, 2024',
            [
              'New AI Help Center: Get answers instantly from our AI assistant.',
              'Improved Product Page Layout.',
              'Fixed an issue with Stripe payments on mobile.',
            ],
          ),
          const SizedBox(height: 24),
          _buildReleaseItem(
            'Version 2.0.5',
            'May 10, 2024',
            [
              'Performance improvements and bug fixes.',
              'Added support for Arabic language.',
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildReleaseItem(String version, String date, List<String> notes) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(
              version,
              style: const TextStyle(
                fontSize: 20,
                fontWeight: FontWeight.bold,
              ),
            ),
            Text(
              date,
              style: const TextStyle(
                color: Colors.grey,
              ),
            ),
          ],
        ),
        const SizedBox(height: 12),
        ...notes.map((note) => Padding(
              padding: const EdgeInsets.only(bottom: 8.0),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('• ', style: TextStyle(fontSize: 16)),
                  Expanded(
                    child: Text(
                      note,
                      style: const TextStyle(fontSize: 16),
                    ),
                  ),
                ],
              ),
            )),
        const Divider(height: 32),
      ],
    );
  }
}
