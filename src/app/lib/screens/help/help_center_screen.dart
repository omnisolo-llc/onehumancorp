import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class HelpCenterScreen extends StatelessWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final categories = [
      {'id': 'getting_started', 'title': 'Getting Started', 'icon': Icons.rocket_launch},
      {'id': 'my_store', 'title': 'My Store', 'icon': Icons.storefront},
      {'id': 'payments', 'title': 'Payments', 'icon': Icons.payments},
      {'id': 'ai_agents', 'title': 'AI Agents', 'icon': Icons.smart_toy},
      {'id': 'marketing', 'title': 'Marketing', 'icon': Icons.campaign},
      {'id': 'account_billing', 'title': 'Account & Billing', 'icon': Icons.manage_accounts},
    ];

    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () {
          showDialog(
            context: context,
            builder: (context) => AlertDialog(
              title: const Text('AI Help Chat'),
              content: const Text('Ask me anything about OneHumanCorp!'),
              actions: [
                TextButton(
                  onPressed: () => Navigator.pop(context),
                  child: const Text('Close'),
                )
              ],
            ),
          );
        },
        icon: const Icon(Icons.support_agent),
        label: const Text('Ask anything'),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'How can we help you today?',
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
            ),
            const SizedBox(height: 16),
            TextField(
              decoration: InputDecoration(
                hintText: 'Search for articles...',
                prefixIcon: const Icon(Icons.search),
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(12),
                ),
                filled: true,
                fillColor: Theme.of(context).colorScheme.surfaceVariant,
              ),
            ),
            const SizedBox(height: 32),
            const Text(
              'Browse by Topic',
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
            ),
            const SizedBox(height: 16),
            ListView.separated(
              shrinkWrap: true,
              physics: const NeverScrollableScrollPhysics(),
              itemCount: categories.length,
              separatorBuilder: (context, index) => const SizedBox(height: 12),
              itemBuilder: (context, index) {
                final category = categories[index];
                return GlassCard(
                  child: ListTile(
                    leading: Icon(category['icon'] as IconData, color: Theme.of(context).colorScheme.primary),
                    title: Text(category['title'] as String, style: const TextStyle(fontWeight: FontWeight.w600)),
                    trailing: const Icon(Icons.chevron_right),
                    onTap: () => context.go('/help/article/${category['id']}'),
                  ),
                );
              },
            ),
            const SizedBox(height: 32),
            const Text(
              'Other Resources',
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
            ),
            const SizedBox(height: 16),
            GlassCard(
              child: Column(
                children: [
                  ListTile(
                    leading: const Icon(Icons.video_library),
                    title: const Text('Video Tutorials'),
                    trailing: const Icon(Icons.chevron_right),
                    onTap: () => context.go('/help/videos'),
                  ),
                  const Divider(),
                  ListTile(
                    leading: const Icon(Icons.new_releases),
                    title: const Text("What's New"),
                    trailing: const Icon(Icons.chevron_right),
                    onTap: () => context.go('/help/release-notes'),
                  ),
                  const Divider(),
                  ListTile(
                    leading: const Icon(Icons.api),
                    title: const Text('API Reference'),
                    trailing: const Icon(Icons.chevron_right),
                    onTap: () => context.go('/help/api-docs'),
                  ),
                  const Divider(),
                  ListTile(
                    leading: const Icon(Icons.tour),
                    title: const Text('Interactive Walkthrough: Setup Store'),
                    trailing: const Icon(Icons.chevron_right),
                    onTap: () => context.go('/help/walkthrough/setup_store'),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
