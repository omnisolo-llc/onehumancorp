import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/widgets/video_tutorial_list.dart';
import 'package:go_router/go_router.dart';

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
              'How can we help you today?',
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 16),
            TextField(
              decoration: InputDecoration(
                hintText: 'Search for articles, guides...',
                prefixIcon: const Icon(Icons.search),
                border: OutlineInputBorder(borderRadius: BorderRadius.circular(12)),
                filled: true,
                fillColor: Theme.of(context).colorScheme.surfaceContainerHighest,
              ),
            ),
            const SizedBox(height: 32),
            const Text('Popular Topics', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
            const SizedBox(height: 16),
            Wrap(
              spacing: 16,
              runSpacing: 16,
              children: [
                _TopicCard(icon: Icons.rocket_launch, title: 'Getting Started', onTap: () {}),
                _TopicCard(icon: Icons.storefront, title: 'My Store', onTap: () {}),
                _TopicCard(icon: Icons.payment, title: 'Payments', onTap: () {}),
                _TopicCard(icon: Icons.smart_toy, title: 'AI Agents', onTap: () {}),
                _TopicCard(icon: Icons.campaign, title: 'Marketing', onTap: () {}),
                _TopicCard(icon: Icons.manage_accounts, title: 'Account & Billing', onTap: () {}),
              ],
            ),
            const SizedBox(height: 32),
            const Text('Video Tutorials', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
            const SizedBox(height: 16),
            const VideoTutorialList(),
            const SizedBox(height: 32),
            GlassCard(
              child: ListTile(
                leading: const Icon(Icons.support_agent, size: 40),
                title: const Text('Still need help?', style: TextStyle(fontWeight: FontWeight.bold)),
                subtitle: const Text('Chat with our AI Help Agent for instant answers.'),
                trailing: FilledButton(
                  onPressed: () => context.go('/help/chat'),
                  child: const Text('Ask anything'),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _TopicCard extends StatelessWidget {
  final IconData icon;
  final String title;
  final VoidCallback onTap;

  const _TopicCard({required this.icon, required this.title, required this.onTap});

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(12),
      child: GlassCard(
        child: Container(
          width: 140,
          padding: const EdgeInsets.all(16),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Icon(icon, size: 32, color: Theme.of(context).colorScheme.primary),
              const SizedBox(height: 8),
              Text(title, textAlign: TextAlign.center, style: const TextStyle(fontWeight: FontWeight.bold)),
            ],
          ),
        ),
      ),
    );
  }
}
