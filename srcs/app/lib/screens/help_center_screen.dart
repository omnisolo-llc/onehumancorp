import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../widgets/glass_card.dart';

class HelpCenterScreen extends StatelessWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'How can we help you today?',
              style: TextStyle(fontFamily: 'Outfit', fontSize: 28, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 24),
            Wrap(
              spacing: 16,
              runSpacing: 16,
              children: [
                _HelpCategoryCard(
                  title: 'Interactive Walkthroughs',
                  icon: Icons.directions_walk,
                  description: 'Step-by-step in-app tours for key flows.',
                  route: '/help', // To be updated when a dedicated screen exists
                ),
                _HelpCategoryCard(
                  title: 'Video Tutorials',
                  icon: Icons.play_circle_outline,
                  description: 'Short (< 90s) videos for top 10 tasks.',
                  route: '/video-tutorials',
                ),
                _HelpCategoryCard(
                  title: 'API Documentation',
                  icon: Icons.code,
                  description: 'Interactive API reference for advanced users.',
                  route: '/api-docs',
                ),
                _HelpCategoryCard(
                  title: 'Release Notes',
                  icon: Icons.new_releases,
                  description: 'What\'s new in the OHC platform.',
                  route: '/release-notes',
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class _HelpCategoryCard extends StatelessWidget {
  final String title;
  final IconData icon;
  final String description;
  final String? route;

  const _HelpCategoryCard({
    required this.title,
    required this.icon,
    required this.description,
    this.route,
  });

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 300),
      child: GlassCard(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Icon(icon, size: 32, color: Theme.of(context).colorScheme.primary),
            const SizedBox(height: 12),
            Text(
              title,
              style: const TextStyle(fontFamily: 'Inter', fontSize: 18, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 8),
            Text(
              description,
              style: const TextStyle(fontFamily: 'Inter', fontSize: 14),
            ),
            const SizedBox(height: 12),
            TextButton(
              onPressed: () {
                if (route != null) {
                  context.push(route!);
                }
              },
              child: const Text('Read Articles →'),
            ),
          ],
        ),
      ),
    );
  }
}
