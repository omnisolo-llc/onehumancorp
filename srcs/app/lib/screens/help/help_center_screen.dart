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
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          _HelpCategory(
            title: 'Getting Started',
            icon: Icons.rocket_launch,
            description: 'Learn the basics of setting up your business.',
            onTap: () => context.push('/help/article/getting-started'),
          ),
          _HelpCategory(
            title: 'My Store',
            icon: Icons.storefront,
            description: 'Managing products, inventory, and storefront.',
            onTap: () => context.push('/help/article/my-store'),
          ),
          _HelpCategory(
            title: 'Payments',
            icon: Icons.payment,
            description: 'Accepting payments and managing your balance.',
            onTap: () => context.push('/help/article/payments'),
          ),
          _HelpCategory(
            title: 'AI Agents',
            icon: Icons.smart_toy,
            description: 'How your AI teammates can help you run the business.',
            onTap: () => context.push('/help/article/ai-agents'),
          ),
          _HelpCategory(
            title: 'Marketing',
            icon: Icons.campaign,
            description: 'Promoting your business and reaching more customers.',
            onTap: () => context.push('/help/article/marketing'),
          ),
          _HelpCategory(
            title: 'Account & Billing',
            icon: Icons.account_circle,
            description: 'Managing your OHC subscription and settings.',
            onTap: () => context.push('/help/article/account-billing'),
          ),
          const SizedBox(height: 24),
          ListTile(
            leading: const Icon(Icons.video_library),
            title: const Text('Video Tutorials', style: TextStyle(fontFamily: 'Outfit')),
            subtitle: const Text('Quick guides to help you out', style: TextStyle(fontFamily: 'Inter')),
            trailing: const Icon(Icons.arrow_forward_ios, size: 16),
            onTap: () => context.push('/help/video-tutorials'),
          ),
          ListTile(
            leading: const Icon(Icons.code),
            title: const Text('API Documentation', style: TextStyle(fontFamily: 'Outfit')),
            subtitle: const Text('For advanced users', style: TextStyle(fontFamily: 'Inter')),
            trailing: const Icon(Icons.arrow_forward_ios, size: 16),
            onTap: () => context.push('/help/api-docs'),
          ),
          ListTile(
            leading: const Icon(Icons.new_releases),
            title: const Text('What\'s New', style: TextStyle(fontFamily: 'Outfit')),
            subtitle: const Text('Recent updates to OHC', style: TextStyle(fontFamily: 'Inter')),
            trailing: const Icon(Icons.arrow_forward_ios, size: 16),
            onTap: () => context.push('/help/changelog'),
          ),
        ],
      ),
    );
  }
}

class _HelpCategory extends StatelessWidget {
  final String title;
  final IconData icon;
  final String description;
  final VoidCallback onTap;

  const _HelpCategory({
    required this.title,
    required this.icon,
    required this.description,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 16),
      child: GlassCard(
        child: InkWell(
          onTap: onTap,
          borderRadius: BorderRadius.circular(16),
          child: Padding(
            padding: const EdgeInsets.all(20),
            child: Row(
              children: [
                Container(
                  padding: const EdgeInsets.all(12),
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.primary.withOpacity(0.1),
                    shape: BoxShape.circle,
                  ),
                  child: Icon(icon, color: Theme.of(context).colorScheme.primary),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        title,
                        style: const TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 18,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(height: 4),
                      Text(
                        description,
                        style: TextStyle(
                          fontFamily: 'Inter',
                          fontSize: 14,
                          color: Theme.of(context).colorScheme.onSurfaceVariant,
                        ),
                      ),
                    ],
                  ),
                ),
                const Icon(Icons.arrow_forward_ios, size: 16),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
