import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class HelpCenterScreen extends ConsumerWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
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
              'Browse by Topic',
              style: TextStyle(
                fontSize: 24,
                fontWeight: FontWeight.bold,
                fontFamily: 'Outfit',
              ),
            ),
            const SizedBox(height: 16),
            Wrap(
              spacing: 16,
              runSpacing: 16,
              children: [
                _buildTopicCard(context, 'Getting Started', Icons.rocket_launch),
                _buildTopicCard(context, 'My Store', Icons.store),
                _buildTopicCard(context, 'Payments', Icons.payment),
              ],
            ),
            const SizedBox(height: 32),
            const Text(
              'Quick Links',
              style: TextStyle(
                fontSize: 24,
                fontWeight: FontWeight.bold,
                fontFamily: 'Outfit',
              ),
            ),
            const SizedBox(height: 16),
            GlassCard(
              child: Column(
                children: [
                  ListTile(
                    leading: const Icon(Icons.api),
                    title: const Text('API Documentation', style: TextStyle(fontFamily: 'Inter')),
                    trailing: const Icon(Icons.chevron_right),
                    onTap: () => context.push('/help-center/api'),
                  ),
                  const Divider(height: 1),
                  ListTile(
                    leading: const Icon(Icons.new_releases),
                    title: const Text('Release Notes', style: TextStyle(fontFamily: 'Inter')),
                    trailing: const Icon(Icons.chevron_right),
                    onTap: () => context.push('/help-center/release-notes'),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildTopicCard(BuildContext context, String title, IconData icon) {
    return SizedBox(
      width: 200,
      child: GlassCard(
        child: InkWell(
          onTap: () {},
          borderRadius: BorderRadius.circular(16),
          child: Padding(
            padding: const EdgeInsets.all(24.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Icon(icon, size: 32, color: Theme.of(context).colorScheme.primary),
                const SizedBox(height: 16),
                Text(
                  title,
                  style: const TextStyle(
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                    fontFamily: 'Outfit',
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
