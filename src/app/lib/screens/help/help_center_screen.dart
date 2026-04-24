import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
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
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Text(
              'How can we help you?',
              style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 24),
            TextField(
              decoration: InputDecoration(
                hintText: 'Search for articles, guides...',
                prefixIcon: const Icon(Icons.search),
                border: OutlineInputBorder(borderRadius: BorderRadius.circular(12)),
              ),
            ),
            const SizedBox(height: 32),
            _buildSectionTitle('Common Topics'),
            const SizedBox(height: 16),
            _buildTopicCard(context, 'Getting Started', Icons.rocket_launch, 'Learn the basics of setting up your store.'),
            const SizedBox(height: 16),
            _buildTopicCard(context, 'My Store', Icons.storefront, 'Manage products, inventory, and storefront design.'),
            const SizedBox(height: 16),
            _buildTopicCard(context, 'Payments', Icons.payment, 'Setup Stripe, handle deposits, and view payouts.'),
            const SizedBox(height: 16),
            _buildTopicCard(context, 'AI Agents', Icons.smart_toy, 'Configure your virtual employees and their skills.'),
            const SizedBox(height: 16),
            _buildTopicCard(context, 'Marketing', Icons.campaign, 'SEO, social media integration, and email campaigns.'),
            const SizedBox(height: 16),
            _buildTopicCard(context, 'Account & Billing', Icons.manage_accounts, 'Manage your OHC subscription and team members.'),

            const SizedBox(height: 32),
            _buildSectionTitle('Video Tutorials'),
            const SizedBox(height: 16),
            _buildTopicCard(context, 'Setting up your first product', Icons.play_circle_outline, 'A 90-second guide to adding items.'),
            const SizedBox(height: 16),
            _buildTopicCard(context, 'Connecting your bank', Icons.play_circle_outline, 'Learn how to receive payouts securely.'),

            const SizedBox(height: 32),
            _buildSectionTitle('Advanced'),
            const SizedBox(height: 16),
            _buildTopicCard(
              context,
              'Interactive API Documentation',
              Icons.code,
              'Access the Swagger/OpenAPI portal to interface with the Swarm directly.',
              onTap: () {
                // In a real app this would open the /api/docs webview or external browser
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(content: Text('Opening Interactive API Docs...')),
                );
              }
            ),
            const SizedBox(height: 16),
            _buildTopicCard(
              context,
              'Release Notes',
              Icons.new_releases,
              'See what\'s new in the latest update.',
              onTap: () => context.push('/changelog')
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildSectionTitle(String title) {
    return Text(
      title,
      style: const TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold),
    );
  }

  Widget _buildTopicCard(BuildContext context, String title, IconData icon, String description, {VoidCallback? onTap}) {
    return GlassCard(
      child: ListTile(
        leading: Icon(icon, color: Theme.of(context).primaryColor, size: 32),
        title: Text(title, style: const TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        subtitle: Text(description, style: const TextStyle(fontFamily: 'Inter', fontSize: 14)),
        trailing: const Icon(Icons.arrow_forward_ios, size: 16),
        onTap: onTap ?? () {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('Opening $title...')),
          );
        },
      ),
    );
  }
}
