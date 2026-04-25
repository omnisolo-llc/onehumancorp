import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class HelpCenterScreen extends ConsumerWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: Stack(
        children: [
          Container(
            decoration: const BoxDecoration(
              gradient: LinearGradient(
                colors: [Color(0xFFE0E7FF), Color(0xFFF3E8FF)],
                begin: Alignment.topLeft,
                end: Alignment.bottomRight,
              ),
            ),
          ),
          SafeArea(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  const Text(
                    'How can we help?',
                    style: TextStyle(fontFamily: 'Outfit', fontSize: 28, fontWeight: FontWeight.bold),
                    textAlign: TextAlign.center,
                  ),
                  const SizedBox(height: 24),
                  GlassCard(
                    child: TextField(
                      decoration: InputDecoration(
                        hintText: 'Search for articles...',
                        hintStyle: const TextStyle(fontFamily: 'Inter'),
                        prefixIcon: const Icon(Icons.search),
                        border: OutlineInputBorder(borderRadius: BorderRadius.circular(12), borderSide: BorderSide.none),
                        filled: true,
                        fillColor: Colors.white.withOpacity(0.5),
                      ),
                    ),
                  ),
                  const SizedBox(height: 32),
                  const Text('Topics', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold)),
                  const SizedBox(height: 16),
                  GridView.count(
                    crossAxisCount: MediaQuery.of(context).size.width > 600 ? 3 : 2,
                    crossAxisSpacing: 16,
                    mainAxisSpacing: 16,
                    shrinkWrap: true,
                    physics: const NeverScrollableScrollPhysics(),
                    children: const [
                      _TopicCard(icon: Icons.rocket_launch, title: 'Getting Started', description: 'Set up your store in minutes.'),
                      _TopicCard(icon: Icons.storefront, title: 'My Store', description: 'Manage products and orders.'),
                      _TopicCard(icon: Icons.payment, title: 'Payments', description: 'Connect Stripe and get paid.'),
                      _TopicCard(icon: Icons.smart_toy, title: 'AI Agents', description: 'Configure your AI team.'),
                      _TopicCard(icon: Icons.campaign, title: 'Marketing', description: 'Grow your business online.'),
                      _TopicCard(icon: Icons.account_circle, title: 'Account & Billing', description: 'Manage your subscription.'),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () {
            context.go('/chat'); // Navigates to the global chat for AI help
        },
        icon: const Icon(Icons.chat),
        label: const Text('Ask AI', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Theme.of(context).colorScheme.primaryContainer,
      ),
    );
  }
}

class _TopicCard extends StatelessWidget {
  final IconData icon;
  final String title;
  final String description;

  const _TopicCard({required this.icon, required this.title, required this.description});

  @override
  Widget build(BuildContext context) {
    return GlassCard(
      child: InkWell(
        onTap: () {},
        borderRadius: BorderRadius.circular(16),
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(icon, size: 32, color: Theme.of(context).colorScheme.primary),
              const SizedBox(height: 12),
              Text(title, style: const TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, fontSize: 16), textAlign: TextAlign.center),
              const SizedBox(height: 8),
              Text(description, style: TextStyle(fontFamily: 'Inter', fontSize: 12, color: Colors.black.withOpacity(0.6)), textAlign: TextAlign.center, maxLines: 2, overflow: TextOverflow.ellipsis),
            ],
          ),
        ),
      ),
    );
  }
}
