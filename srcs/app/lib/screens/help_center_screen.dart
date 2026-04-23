import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/models/help_article.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/widgets/help_tooltip.dart';

final helpArticlesProvider = Provider<List<HelpArticle>>((ref) {
  return [
    const HelpArticle(
      id: '1',
      title: 'Set up your store',
      category: 'Getting Started',
      content: 'Setting up your store is easy. First, go to Settings and enter your business name and address. Then, start adding your products in the Dashboard.',
    ),
    const HelpArticle(
      id: '2',
      title: 'Accept your first payment',
      category: 'Payments',
      content: 'To accept payments, connect your bank account in the Billing section. OHC uses Stripe to securely process your customers\' credit cards.',
    ),
    const HelpArticle(
      id: '3',
      title: 'Activate your AI Support Agent',
      category: 'AI Agents',
      content: 'Go to the Agents page and click "Hire Agent" to get an AI Support Agent. It will automatically reply to customer questions for you.',
    ),
    const HelpArticle(
      id: '4',
      title: 'How to sell on Instagram',
      category: 'Marketing',
      content: 'Share your OHC storefront link in your Instagram bio. Your AI Marketing Agent can even draft posts for you!',
    ),
  ];
});

final helpSearchQueryProvider = StateProvider<String>((ref) => '');

class HelpCenterScreen extends ConsumerWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final articles = ref.watch(helpArticlesProvider);
    final searchQuery = ref.watch(helpSearchQueryProvider).toLowerCase();

    final filteredArticles = articles.where((article) {
      if (searchQuery.isEmpty) return true;
      return article.title.toLowerCase().contains(searchQuery) ||
             article.content.toLowerCase().contains(searchQuery);
    }).toList();

    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'Help Center',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
      ),
      body: SafeArea(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(16.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Text(
                'How can we help you?',
                style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                  fontFamily: 'Outfit',
                  fontWeight: FontWeight.bold,
                ),
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 24),
              TextField(
                onChanged: (value) => ref.read(helpSearchQueryProvider.notifier).state = value,
                decoration: InputDecoration(
                  hintText: 'Search for help...',
                  prefixIcon: const Icon(Icons.search),
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                  filled: true,
                  fillColor: Theme.of(context).colorScheme.surfaceContainerHighest,
                ),
              ),
              const SizedBox(height: 32),
              if (filteredArticles.isEmpty)
                const Center(
                  child: Padding(
                    padding: EdgeInsets.all(32.0),
                    child: Text('No articles found.', style: TextStyle(fontSize: 16)),
                  ),
                )
              else
                ...filteredArticles.map((article) => _ArticleCard(article: article)),
            ],
          ),
        ),
      ),
    );
  }
}

class _ArticleCard extends StatelessWidget {
  final HelpArticle article;

  const _ArticleCard({required this.article});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 16.0),
      child: GlassCard(
        child: Padding(
          padding: const EdgeInsets.all(20.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                children: [
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                    decoration: BoxDecoration(
                      color: Theme.of(context).colorScheme.primaryContainer,
                      borderRadius: BorderRadius.circular(4),
                    ),
                    child: Text(
                      article.category,
                      style: TextStyle(
                        fontSize: 12,
                        fontFamily: 'Inter',
                        fontWeight: FontWeight.bold,
                        color: Theme.of(context).colorScheme.onPrimaryContainer,
                      ),
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 12),
              HelpTooltip(
                tooltipKey: 'help_article_${article.id}', // Currently unregistered, but demonstrates usage
                child: Text(
                  article.title,
                  style: const TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
              const SizedBox(height: 8),
              Text(
                article.content,
                style: const TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 15,
                  height: 1.5,
                ),
              ),
              const SizedBox(height: 16),
              Align(
                alignment: Alignment.centerRight,
                child: TextButton(
                  onPressed: () {
                    // In a real app, this might open a full-page article view
                  },
                  child: const Text('Read full article →'),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
