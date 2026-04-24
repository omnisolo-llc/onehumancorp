import 'dart:ui';
import 'package:flutter/material.dart';

class HelpCenterScreen extends StatelessWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        backgroundColor: Colors.transparent,
      ),
      body: LayoutBuilder(
        builder: (context, constraints) {
          final isMobile = constraints.maxWidth < 768;
          return SingleChildScrollView(
            padding: const EdgeInsets.all(24.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                const Text(
                  'How can we help you grow today?',
                  style: TextStyle(fontFamily: 'Outfit', fontSize: 28, fontWeight: FontWeight.bold, color: Colors.white),
                ),
                const SizedBox(height: 24),
                TextField(
                  style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
                  decoration: InputDecoration(
                    hintText: 'Search for articles, guides, and tutorials...',
                    hintStyle: const TextStyle(color: Colors.white54),
                    prefixIcon: const Icon(Icons.search, color: Colors.white54),
                    filled: true,
                    fillColor: Colors.white.withValues(alpha: 0.05),
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(16),
                      borderSide: BorderSide(color: Colors.white.withValues(alpha: 0.1)),
                    ),
                  ),
                ),
                const SizedBox(height: 32),
                const Text(
                  'Explore Topics',
                  style: TextStyle(fontFamily: 'Outfit', fontSize: 22, fontWeight: FontWeight.bold, color: Colors.white),
                ),
                const SizedBox(height: 16),
                GridView.count(
                  crossAxisCount: isMobile ? 1 : 3,
                  shrinkWrap: true,
                  physics: const NeverScrollableScrollPhysics(),
                  mainAxisSpacing: 16,
                  crossAxisSpacing: 16,
                  childAspectRatio: isMobile ? 3 : 1.5,
                  children: const [
                    _TopicCard(title: 'Getting Started', icon: Icons.rocket_launch, description: 'Set up your store in minutes.'),
                    _TopicCard(title: 'My Store', icon: Icons.storefront, description: 'Manage products and inventory.'),
                    _TopicCard(title: 'Payments', icon: Icons.payments, description: 'Get paid and manage deposits.'),
                    _TopicCard(title: 'AI Agents', icon: Icons.smart_toy, description: 'Learn how AI can run your business.'),
                    _TopicCard(title: 'Marketing', icon: Icons.campaign, description: 'Reach more customers.'),
                    _TopicCard(title: 'Account & Billing', icon: Icons.person, description: 'Manage your OHC subscription.'),
                  ],
                ),
                const SizedBox(height: 48),
                const Text(
                  'Video Tutorials',
                  style: TextStyle(fontFamily: 'Outfit', fontSize: 22, fontWeight: FontWeight.bold, color: Colors.white),
                ),
                const SizedBox(height: 16),
                SizedBox(
                  height: 200,
                  child: ListView(
                    scrollDirection: Axis.horizontal,
                    children: const [
                      _VideoThumbnail(title: 'Accepting your first payment'),
                      _VideoThumbnail(title: 'Adding a new product'),
                      _VideoThumbnail(title: 'Customizing your storefront'),
                    ],
                  ),
                ),
                const SizedBox(height: 48),
                const Divider(),
                const SizedBox(height: 24),
                const Text(
                  'Additional Resources',
                  style: TextStyle(fontFamily: 'Outfit', fontSize: 22, fontWeight: FontWeight.bold, color: Colors.white),
                ),
                const SizedBox(height: 16),
                Wrap(
                  spacing: 16,
                  runSpacing: 16,
                  children: [
                    ActionChip(
                      label: const Text('Release Notes (What\'s New)'),
                      onPressed: () {},
                      backgroundColor: Colors.white.withValues(alpha: 0.1),
                    ),
                    ActionChip(
                      label: const Text('Advanced: API Documentation'),
                      onPressed: () {},
                      backgroundColor: Colors.white.withValues(alpha: 0.1),
                    ),
                  ],
                )
              ],
            ),
          );
        },
      ),
    );
  }
}

class _TopicCard extends StatelessWidget {
  final String title;
  final String description;
  final IconData icon;

  const _TopicCard({
    required this.title,
    required this.description,
    required this.icon,
  });

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.compose(
          outer: const ColorFilter.matrix(<double>[
            1.787, -0.715, -0.072, 0, 0,
            -0.213, 1.285, -0.072, 0, 0,
            -0.213, -0.715, 1.928, 0, 0,
            0, 0, 0, 1, 0,
          ]),
          inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        ),
        child: InkWell(
          onTap: () {},
          child: Container(
            padding: const EdgeInsets.all(20),
            decoration: BoxDecoration(
              color: Colors.white.withValues(alpha: 0.05),
              borderRadius: BorderRadius.circular(16),
              border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                Icon(icon, color: Colors.blueAccent, size: 32),
                const SizedBox(height: 12),
                Text(
                  title,
                  style: const TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold, color: Colors.white),
                ),
                const SizedBox(height: 4),
                Expanded(
                  child: Text(
                    description,
                    style: const TextStyle(fontFamily: 'Inter', fontSize: 14, color: Colors.white70),
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
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

class _VideoThumbnail extends StatelessWidget {
  final String title;

  const _VideoThumbnail({required this.title});

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 150,
      margin: const EdgeInsets.only(right: 16),
      decoration: BoxDecoration(
        color: Colors.black45,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
      ),
      child: Stack(
        alignment: Alignment.center,
        children: [
          const Icon(Icons.play_circle_fill, size: 48, color: Colors.white54),
          Positioned(
            bottom: 12,
            left: 12,
            right: 12,
            child: Text(
              title,
              style: const TextStyle(fontFamily: 'Inter', fontSize: 12, color: Colors.white),
              maxLines: 2,
              textAlign: TextAlign.center,
              overflow: TextOverflow.ellipsis,
            ),
          ),
        ],
      ),
    );
  }
}
