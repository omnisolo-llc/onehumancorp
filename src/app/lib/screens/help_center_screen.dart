import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class HelpCenterScreen extends StatelessWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontWeight: FontWeight.bold)),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'How can we help you run your business today?',
              style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 24),
            _buildCategorySection(context, 'Getting Started', [
              'Set up your store',
              'Connect your domain',
              'Add your first product',
            ]),
            _buildCategorySection(context, 'My Store', [
              'Manage inventory',
              'Process custom orders',
            ]),
            _buildCategorySection(context, 'Payments', [
              'Accept online payments',
              'Set up deposits',
            ]),
            _buildCategorySection(context, 'AI Agents', [
              'What can AI do for me?',
              'Configure the Support Agent',
            ]),
            _buildCategorySection(context, 'Marketing', [
              'Connect Instagram',
              'Send a newsletter',
            ]),
            _buildCategorySection(context, 'Account & Billing', [
              'Update your subscription',
            ]),
            const SizedBox(height: 80), // Space for FAB
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () {
          context.go('/chat');
        },
        icon: const Icon(Icons.chat_bubble_outline),
        label: const Text('Ask anything'),
      ),
    );
  }

  Widget _buildCategorySection(BuildContext context, String title, List<String> articles) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 24.0),
      child: GlassCard(
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                title,
                style: Theme.of(context).textTheme.titleLarge?.copyWith(
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(height: 12),
              ...articles.map((article) => ListTile(
                    contentPadding: EdgeInsets.zero,
                    title: Text(article),
                    trailing: const Icon(Icons.arrow_forward_ios, size: 16),
                    onTap: () {
                      // In a real implementation, this would navigate to the article
                      ScaffoldMessenger.of(context).showSnackBar(
                        SnackBar(content: Text('Opening: $article')),
                      );
                    },
                  )),
            ],
          ),
        ),
      ),
    );
  }
}
