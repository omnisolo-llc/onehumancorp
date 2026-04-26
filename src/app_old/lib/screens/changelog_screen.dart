import 'package:flutter/material.dart';

class ChangelogScreen extends StatelessWidget {
  const ChangelogScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Release Notes', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              "What's New",
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
            ),
            const SizedBox(height: 24),
            _buildReleaseBlock(
              version: 'v1.2.0',
              date: 'April 26, 2026',
              features: [
                'New AI Help Center: Access help articles directly from the dashboard.',
                'Interactive Walkthroughs: Guided tours for setting up your first store.',
                'Performance improvements on mobile devices.',
              ],
            ),
            const SizedBox(height: 24),
            _buildReleaseBlock(
              version: 'v1.1.0',
              date: 'April 10, 2026',
              features: [
                'Added support for Apple Pay and Google Pay.',
                'New Business Advisory Agent weekly reports.',
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildReleaseBlock({required String version, required String date, required List<String> features}) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Text(
              version,
              style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
            ),
            const SizedBox(width: 12),
            Text(
              date,
              style: const TextStyle(color: Colors.grey),
            ),
          ],
        ),
        const SizedBox(height: 8),
        ...features.map((feature) => Padding(
              padding: const EdgeInsets.only(bottom: 4.0),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('• ', style: TextStyle(fontSize: 16)),
                  Expanded(child: Text(feature)),
                ],
              ),
            )),
      ],
    );
  }
}
