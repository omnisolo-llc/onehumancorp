import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class ReleaseNotesScreen extends StatelessWidget {
  const ReleaseNotesScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('What\'s New', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        centerTitle: true,
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24.0),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 800),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const Text(
                  'Latest Updates in One Human Corp',
                  style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
                  textAlign: TextAlign.center,
                ),
                const SizedBox(height: 32),
                _buildReleaseCard(
                  context,
                  version: 'Version 0.4.2',
                  date: 'April 2026',
                  title: 'A Smarter Help Center & In-App Assistant',
                  description: 'We\'ve completely revamped the way you get help. Now you can search for articles, watch video tutorials, and even chat with our AI Help Assistant directly from any screen in the app. No more leaving your store to find answers!',
                  features: [
                    'New searchable Help Center',
                    'Floating AI Help Assistant on every page',
                    'Interactive video tutorials',
                  ],
                ),
                const SizedBox(height: 24),
                _buildReleaseCard(
                  context,
                  version: 'Version 0.4.1',
                  date: 'March 2026',
                  title: 'Performance & Hybrid Sync Improvements',
                  description: 'We made things faster! Your store will now load quicker even on slow mobile networks. We also improved how the app syncs your data when you are offline.',
                  features: [
                    'Faster image loading with WebP auto-compression',
                    'Improved offline sync resilience',
                  ],
                ),
                const SizedBox(height: 32),
                Center(
                  child: TextButton.icon(
                    onPressed: () {},
                    icon: const Icon(Icons.history),
                    label: const Text('View Full History on Website'),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildReleaseCard(BuildContext context, {
    required String version,
    required String date,
    required String title,
    required String description,
    required List<String> features,
  }) {
    return GlassCard(
      padding: const EdgeInsets.all(24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                decoration: BoxDecoration(
                  color: Theme.of(context).colorScheme.primary.withOpacity(0.1),
                  borderRadius: BorderRadius.circular(20),
                ),
                child: Text(
                  version,
                  style: TextStyle(
                    color: Theme.of(context).colorScheme.primary,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
              Text(
                date,
                style: const TextStyle(color: Colors.grey, fontFamily: 'Inter'),
              ),
            ],
          ),
          const SizedBox(height: 16),
          Text(
            title,
            style: const TextStyle(fontSize: 22, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
          ),
          const SizedBox(height: 12),
          Text(
            description,
            style: const TextStyle(fontSize: 16, fontFamily: 'Inter', height: 1.5),
          ),
          const SizedBox(height: 16),
          const Text(
            'Highlights:',
            style: TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Inter'),
          ),
          const SizedBox(height: 8),
          ...features.map((feature) {
            return Padding(
              padding: const EdgeInsets.only(bottom: 6),
              child: Row(
                children: [
                  Icon(Icons.check_circle, size: 16, color: Theme.of(context).colorScheme.primary),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Text(feature, style: const TextStyle(fontFamily: 'Inter')),
                  ),
                ],
              ),
            );
          }),
          const SizedBox(height: 16),
          // Placeholder for screenshot
          Container(
            height: 200,
            width: double.infinity,
            decoration: BoxDecoration(
              color: Theme.of(context).colorScheme.surfaceContainerHighest.withOpacity(0.5),
              borderRadius: BorderRadius.circular(12),
            ),
            child: const Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Icon(Icons.image, size: 48, color: Colors.grey),
                  SizedBox(height: 8),
                  Text('Screenshot Placeholder', style: TextStyle(color: Colors.grey)),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
