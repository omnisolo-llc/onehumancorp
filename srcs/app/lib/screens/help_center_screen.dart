import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class HelpCenterScreen extends StatelessWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center'),
        centerTitle: false,
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            GlassCard(
              child: Padding(
                padding: const EdgeInsets.all(20.0),
                child: Column(
                  children: [
                    Text(
                      'How can we help you today?',
                      style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                            fontWeight: FontWeight.bold,
                          ),
                      textAlign: TextAlign.center,
                    ),
                    const SizedBox(height: 16),
                    TextField(
                      decoration: InputDecoration(
                        hintText: 'Search for help...',
                        prefixIcon: const Icon(Icons.search),
                        border: OutlineInputBorder(
                          borderRadius: BorderRadius.circular(12),
                        ),
                        filled: true,
                        fillColor: Theme.of(context).colorScheme.surface,
                      ),
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 24),
            Text(
              'Browse Topics',
              style: Theme.of(context).textTheme.titleLarge?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
            ),
            const SizedBox(height: 16),
            _buildTopicGrid(context),
            const SizedBox(height: 24),
            Text(
              'Popular Articles',
              style: Theme.of(context).textTheme.titleLarge?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
            ),
            const SizedBox(height: 16),
            _buildPopularArticles(context),
            const SizedBox(height: 80), // Padding for the floating chat button
          ],
        ),
      ),
    );
  }

  Widget _buildTopicGrid(BuildContext context) {
    final topics = [
      {'icon': Icons.rocket_launch, 'title': 'Getting Started'},
      {'icon': Icons.storefront, 'title': 'My Store'},
      {'icon': Icons.payment, 'title': 'Payments'},
      {'icon': Icons.smart_toy, 'title': 'AI Agents'},
      {'icon': Icons.campaign, 'title': 'Marketing'},
      {'icon': Icons.settings, 'title': 'Account & Billing'},
    ];

    return GridView.builder(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: 2,
        crossAxisSpacing: 16,
        mainAxisSpacing: 16,
        childAspectRatio: 1.5,
      ),
      itemCount: topics.length,
      itemBuilder: (context, index) {
        final topic = topics[index];
        return GlassCard(
          child: InkWell(
            onTap: () {
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(content: Text('Opening ${topic['title']}...')),
              );
            },
            borderRadius: BorderRadius.circular(12),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Icon(
                  topic['icon'] as IconData,
                  size: 32,
                  color: Theme.of(context).colorScheme.primary,
                ),
                const SizedBox(height: 8),
                Text(
                  topic['title'] as String,
                  style: const TextStyle(fontWeight: FontWeight.w600),
                  textAlign: TextAlign.center,
                ),
              ],
            ),
          ),
        );
      },
    );
  }

  Widget _buildPopularArticles(BuildContext context) {
    final articles = [
      'How to set up your first product',
      'Understanding Stripe payouts',
      'What does the Customer Success Agent do?',
      'How to connect your custom domain',
    ];

    return GlassCard(
      child: ListView.separated(
        shrinkWrap: true,
        physics: const NeverScrollableScrollPhysics(),
        itemCount: articles.length,
        separatorBuilder: (context, index) => const Divider(height: 1),
        itemBuilder: (context, index) {
          return ListTile(
            title: Text(articles[index]),
            trailing: const Icon(Icons.chevron_right, size: 20),
            onTap: () {
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(content: Text('Opening article...')),
              );
            },
          );
        },
      ),
    );
  }
}
