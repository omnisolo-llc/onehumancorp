import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class HelpCenterScreen extends StatelessWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0D0D1A),
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'How can we help you?',
              style: TextStyle(fontFamily: 'Outfit', fontSize: 32, fontWeight: FontWeight.bold, color: Colors.white),
            ),
            const SizedBox(height: 24),
            TextField(
              decoration: InputDecoration(
                hintText: 'Search for articles, tutorials...',
                hintStyle: const TextStyle(color: Colors.white54),
                prefixIcon: const Icon(Icons.search, color: Colors.white54),
                filled: true,
                fillColor: Colors.white.withAlpha(13),
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(12),
                  borderSide: BorderSide.none,
                ),
              ),
              style: const TextStyle(color: Colors.white),
            ),
            const SizedBox(height: 32),
            const Text(
              'Video Tutorials',
              style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
            ),
            const SizedBox(height: 16),
            Wrap(
              spacing: 16,
              runSpacing: 16,
              children: [
                _VideoTutorialCard(title: 'Set up your store in 5 minutes', duration: '1:20'),
                _VideoTutorialCard(title: 'Accept your first payment', duration: '0:55'),
                _VideoTutorialCard(title: 'Activate your AI Support Agent', duration: '1:10'),
              ],
            ),
            const SizedBox(height: 32),
            const Text(
              'Topics',
              style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
            ),
            const SizedBox(height: 16),
            Wrap(
              spacing: 16,
              runSpacing: 16,
              children: [
                _TopicCard(icon: Icons.rocket_launch, title: 'Getting Started', articleCount: 12),
                _TopicCard(icon: Icons.storefront, title: 'My Store', articleCount: 8),
                _TopicCard(icon: Icons.payment, title: 'Payments', articleCount: 15),
                _TopicCard(icon: Icons.smart_toy, title: 'AI Agents', articleCount: 20),
                _TopicCard(icon: Icons.campaign, title: 'Marketing', articleCount: 7),
                _TopicCard(icon: Icons.account_circle, title: 'Account & Billing', articleCount: 5),
              ],
            ),
            const SizedBox(height: 32),
            const Text(
              'Advanced',
              style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
            ),
            const SizedBox(height: 16),
            GlassCard(
              child: ListTile(
                leading: const Icon(Icons.api, color: Colors.blueAccent),
                title: const Text('API Documentation', style: TextStyle(fontFamily: 'Inter', color: Colors.white, fontWeight: FontWeight.bold)),
                subtitle: const Text('For developers and custom integrations', style: TextStyle(fontFamily: 'Inter', color: Colors.white70)),
                trailing: const Icon(Icons.arrow_forward_ios, color: Colors.white54, size: 16),
                onTap: () {
                  // Navigate to API Docs or open URL
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _VideoTutorialCard extends StatelessWidget {
  final String title;
  final String duration;

  const _VideoTutorialCard({required this.title, required this.duration});

  @override
  Widget build(BuildContext context) {
    return Semantics(
      label: 'Video tutorial: $title',
      button: true,
      child: SizedBox(
        width: 250,
        child: GlassCard(
          padding: EdgeInsets.zero,
          child: InkWell(
            onTap: () {},
            borderRadius: BorderRadius.circular(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Container(
                  height: 140,
                  width: double.infinity,
                  decoration: BoxDecoration(
                    color: Colors.black26,
                    borderRadius: const BorderRadius.vertical(top: Radius.circular(16)),
                  ),
                  child: const Center(
                    child: Icon(Icons.play_circle_fill, size: 48, color: Colors.white),
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        title,
                        style: const TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white),
                        maxLines: 2,
                        overflow: TextOverflow.ellipsis,
                      ),
                      const SizedBox(height: 8),
                      Text(
                        duration,
                        style: const TextStyle(fontFamily: 'Inter', color: Colors.white54, fontSize: 12),
                      ),
                    ],
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

class _TopicCard extends StatelessWidget {
  final IconData icon;
  final String title;
  final int articleCount;

  const _TopicCard({required this.icon, required this.title, required this.articleCount});

  @override
  Widget build(BuildContext context) {
    return Semantics(
      label: 'Topic: $title with $articleCount articles',
      button: true,
      child: SizedBox(
        width: 200,
        child: GlassCard(
          child: InkWell(
            onTap: () {},
            borderRadius: BorderRadius.circular(16),
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Icon(icon, size: 32, color: Colors.blueAccent),
                  const SizedBox(height: 16),
                  Text(
                    title,
                    style: const TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white, fontSize: 16),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    '$articleCount articles',
                    style: const TextStyle(fontFamily: 'Inter', color: Colors.white54, fontSize: 12),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
