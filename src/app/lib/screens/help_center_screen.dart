import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

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
              style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
            ),
            const SizedBox(height: 24),
            Wrap(
              spacing: 16,
              runSpacing: 16,
              children: [
                _HelpCategoryCard(title: 'Getting Started', icon: Icons.rocket_launch, description: 'Learn the basics of setting up your store.'),
                _HelpCategoryCard(title: 'My Store', icon: Icons.store, description: 'Manage your products, inventory, and storefront.'),
                _HelpCategoryCard(title: 'Payments', icon: Icons.payment, description: 'Set up Stripe, process refunds, and view payouts.'),
                _HelpCategoryCard(title: 'AI Agents', icon: Icons.smart_toy, description: 'Understand how your AI team works for you.'),
                _HelpCategoryCard(title: 'Marketing', icon: Icons.campaign, description: 'Grow your audience and drive more sales.'),
                _HelpCategoryCard(title: 'Account & Billing', icon: Icons.person, description: 'Manage your subscription and organization settings.'),
              ],
            ),
            const SizedBox(height: 48),
            const Text(
              'Video Tutorials',
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
            ),
            const SizedBox(height: 16),
            Wrap(
              spacing: 16,
              runSpacing: 16,
              children: [
                _VideoPlaceholderCard(title: 'Set up your store in 5 minutes'),
                _VideoPlaceholderCard(title: 'Accepting your first payment'),
                _VideoPlaceholderCard(title: 'How AI agents handle customer support'),
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

  const _HelpCategoryCard({required this.title, required this.icon, required this.description});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 300,
      child: GlassCard(
        child: Padding(
          padding: const EdgeInsets.all(20.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Icon(icon, size: 36, color: Theme.of(context).colorScheme.primary),
              const SizedBox(height: 12),
              Text(title, style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
              const SizedBox(height: 8),
              Text(description, style: const TextStyle(fontSize: 14, fontFamily: 'Inter')),
            ],
          ),
        ),
      ),
    );
  }
}

class _VideoPlaceholderCard extends StatelessWidget {
  final String title;
  const _VideoPlaceholderCard({required this.title});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 250,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Container(
            height: 140,
            decoration: BoxDecoration(
              color: Colors.grey.withValues(alpha: 0.2),
              borderRadius: BorderRadius.circular(12),
            ),
            child: const Center(
              child: Icon(Icons.play_circle_fill, size: 48, color: Colors.white70),
            ),
          ),
          const SizedBox(height: 8),
          Text(title, style: const TextStyle(fontWeight: FontWeight.w600, fontFamily: 'Inter')),
          const SizedBox(height: 4),
          const Text('1 min read', style: TextStyle(fontSize: 12, color: Colors.grey)),
        ],
      ),
    );
  }
}
