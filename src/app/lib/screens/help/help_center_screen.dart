import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

class HelpCenterScreen extends StatelessWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit')),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 800),
          child: ListView(
            padding: const EdgeInsets.all(24),
            children: [
              const Text(
                'How can we help you today?',
                style: TextStyle(
                  fontSize: 28,
                  fontWeight: FontWeight.bold,
                  fontFamily: 'Outfit',
                ),
              ),
              const SizedBox(height: 16),
              const TextField(
                decoration: InputDecoration(
                  hintText: 'Search help articles...',
                  prefixIcon: Icon(Icons.search),
                  border: OutlineInputBorder(),
                  contentPadding: EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                ),
              ),
              const SizedBox(height: 32),
              const Text(
                'Browse by Topic',
                style: TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  fontFamily: 'Outfit',
                ),
              ),
              const SizedBox(height: 16),
              _buildTopicCard(
                context,
                icon: Icons.rocket_launch,
                title: 'Getting Started',
                description: 'Set up your account and launch your business.',
              ),
              const SizedBox(height: 12),
              _buildTopicCard(
                context,
                icon: Icons.storefront,
                title: 'My Store',
                description: 'Manage products, inventory, and your storefront.',
              ),
              const SizedBox(height: 12),
              _buildTopicCard(
                context,
                icon: Icons.payments,
                title: 'Payments',
                description: 'Accept payments and manage your finances.',
              ),
              const SizedBox(height: 12),
              _buildTopicCard(
                context,
                icon: Icons.smart_toy,
                title: 'AI Agents',
                description: 'Configure and train your AI assistants.',
              ),
              const SizedBox(height: 12),
              _buildTopicCard(
                context,
                icon: Icons.campaign,
                title: 'Marketing',
                description: 'Grow your audience and drive more sales.',
              ),
              const SizedBox(height: 12),
              _buildTopicCard(
                context,
                icon: Icons.receipt_long,
                title: 'Account & Billing',
                description: 'Manage your subscription and billing details.',
              ),
              const SizedBox(height: 32),
              const Text(
                'Video Tutorials',
                style: TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  fontFamily: 'Outfit',
                ),
              ),
              const SizedBox(height: 16),
              SizedBox(
                height: 160,
                child: ListView(
                  scrollDirection: Axis.horizontal,
                  children: [
                    _buildVideoThumbnail(context, 'Set up your first product', '1:20'),
                    const SizedBox(width: 16),
                    _buildVideoThumbnail(context, 'How to use AI Agents', '2:05'),
                    const SizedBox(width: 16),
                    _buildVideoThumbnail(context, 'Managing custom orders', '1:45'),
                  ],
                ),
              ),
              const SizedBox(height: 32),
              const Text(
                'Quick Links',
                style: TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  fontFamily: 'Outfit',
                ),
              ),
              const SizedBox(height: 16),
              ListTile(
                leading: const Icon(Icons.code),
                title: const Text('API Documentation', style: TextStyle(fontFamily: 'Inter')),
                trailing: const Icon(Icons.arrow_forward_ios, size: 16),
                onTap: () => context.go('/help/api'),
              ),
              const Divider(),
              ListTile(
                leading: const Icon(Icons.new_releases),
                title: const Text('Release Notes', style: TextStyle(fontFamily: 'Inter')),
                trailing: const Icon(Icons.arrow_forward_ios, size: 16),
                onTap: () => context.go('/help/changelog'),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildTopicCard(
    BuildContext context, {
    required IconData icon,
    required String title,
    required String description,
  }) {
    return Card(
      elevation: 2,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: InkWell(
        onTap: () {
          // Placeholder for topic navigation
        },
        borderRadius: BorderRadius.circular(12),
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Row(
            children: [
              Container(
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  color: Theme.of(context).colorScheme.primaryContainer,
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Icon(
                  icon,
                  color: Theme.of(context).colorScheme.onPrimaryContainer,
                  size: 28,
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      title,
                      style: const TextStyle(
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                        fontFamily: 'Outfit',
                      ),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      description,
                      style: const TextStyle(
                        fontSize: 14,
                        color: Colors.grey,
                        fontFamily: 'Inter',
                      ),
                    ),
                  ],
                ),
              ),
              const Icon(Icons.arrow_forward_ios, size: 16, color: Colors.grey),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildVideoThumbnail(BuildContext context, String title, String duration) {
    return Container(
      width: 240,
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surfaceContainerHighest,
        borderRadius: BorderRadius.circular(12),
      ),
      child: Stack(
        children: [
          const Center(
            child: Icon(Icons.play_circle_fill, size: 48, color: Colors.white70),
          ),
          Positioned(
            bottom: 8,
            left: 8,
            right: 8,
            child: Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              decoration: BoxDecoration(
                color: Colors.black54,
                borderRadius: BorderRadius.circular(4),
              ),
              child: Text(
                title,
                style: const TextStyle(color: Colors.white, fontSize: 12, fontFamily: 'Inter'),
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
              ),
            ),
          ),
          Positioned(
            top: 8,
            right: 8,
            child: Container(
              padding: const EdgeInsets.symmetric(horizontal: 4, vertical: 2),
              decoration: BoxDecoration(
                color: Colors.black54,
                borderRadius: BorderRadius.circular(4),
              ),
              child: Text(
                duration,
                style: const TextStyle(color: Colors.white, fontSize: 10, fontFamily: 'Inter'),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
