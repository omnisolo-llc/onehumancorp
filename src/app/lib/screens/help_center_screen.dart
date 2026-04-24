import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/widgets/tooltip_registry.dart';

class HelpCenterScreen extends StatelessWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => context.go('/dashboard'),
        ),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 800),
          child: ListView(
            padding: const EdgeInsets.all(24.0),
            children: [
              const Text(
                'How can we help you today?',
                style: TextStyle(fontFamily: 'Outfit', fontSize: 32, fontWeight: FontWeight.bold),
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 32),
              ContextualTooltip(
                tooltipKey: 'help_center_search',
                child: TextField(
                  decoration: InputDecoration(
                    hintText: 'Search for articles, guides, or features...',
                    prefixIcon: const Icon(Icons.search),
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(16),
                    ),
                    filled: true,
                    fillColor: Colors.white.withValues(alpha: 0.05),
                  ),
                ),
              ),
              const SizedBox(height: 32),
              const _HelpSection(
                title: 'Getting Started',
                icon: Icons.rocket_launch,
                articles: [
                  'How to launch your store',
                  'Setting up your custom domain',
                  'Adding your first product'
                ],
              ),
              const SizedBox(height: 16),
              const _HelpSection(
                title: 'My Store',
                icon: Icons.storefront,
                articles: [
                  'Managing your inventory',
                  'Customizing your storefront design',
                  'Handling refunds and returns'
                ],
              ),
              const SizedBox(height: 16),
              const _HelpSection(
                title: 'Payments',
                icon: Icons.payments,
                articles: [
                  'Connecting your bank account',
                  'Taking in-person payments (POS)',
                  'Understanding payout schedules'
                ],
              ),
              const SizedBox(height: 16),
              const _HelpSection(
                title: 'AI Agents',
                icon: Icons.smart_toy,
                articles: [
                  'What are AI Agents?',
                  'Configuring your Support Agent',
                  'How the Marketing Agent grows your audience'
                ],
              ),
              const SizedBox(height: 32),
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  ElevatedButton.icon(
                    onPressed: () => context.go('/help/videos'),
                    icon: const Icon(Icons.play_circle_fill),
                    label: const Text('Watch Video Tutorials'),
                  ),
                  const SizedBox(width: 16),
                  ElevatedButton.icon(
                    onPressed: () => context.go('/help/release_notes'),
                    icon: const Icon(Icons.new_releases),
                    label: const Text('What\'s New'),
                  ),
                  const SizedBox(width: 16),
                  ElevatedButton.icon(
                    onPressed: () => context.go('/help/api_docs'),
                    icon: const Icon(Icons.code),
                    label: const Text('API Reference'),
                  ),
                ],
              ),
              const SizedBox(height: 100),
            ],
          ),
        ),
      ),
    );
  }
}

class _HelpSection extends StatelessWidget {
  final String title;
  final IconData icon;
  final List<String> articles;

  const _HelpSection({
    required this.title,
    required this.icon,
    required this.articles,
  });

  @override
  Widget build(BuildContext context) {
    return GlassCard(
      child: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(icon, size: 28, color: Theme.of(context).colorScheme.primary),
                const SizedBox(width: 12),
                Text(
                  title,
                  style: const TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 22,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            ...articles.map((article) => Padding(
                  padding: const EdgeInsets.symmetric(vertical: 8.0),
                  child: InkWell(
                    onTap: () {},
                    child: Row(
                      children: [
                        const Icon(Icons.article, size: 18, color: Colors.grey),
                        const SizedBox(width: 8),
                        Expanded(
                          child: Text(
                            article,
                            style: const TextStyle(
                              fontFamily: 'Inter',
                              fontSize: 16,
                              color: Colors.white70,
                            ),
                          ),
                        ),
                        const Icon(Icons.chevron_right, color: Colors.grey),
                      ],
                    ),
                  ),
                )),
          ],
        ),
      ),
    );
  }
}
