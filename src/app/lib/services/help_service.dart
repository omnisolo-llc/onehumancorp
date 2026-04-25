import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/models/help_article.dart';

final helpServiceProvider = Provider<HelpService>((ref) => HelpService());

class HelpService {
  final List<HelpCategory> _categories = [
    HelpCategory(
      id: 'getting-started',
      title: 'Getting Started',
      icon: Icons.rocket_launch,
      articles: [
        HelpArticle(
          id: 'set-up-store',
          title: 'How to set up your store',
          content: 'Setting up your store is easy! Just navigate to the settings, add your business details, and configure your payment methods. Our AI will automatically generate a beautiful storefront for you based on the information provided.',
          categoryId: 'getting-started',
          keywords: ['setup', 'store', 'start', 'begin', 'onboarding'],
        ),
        HelpArticle(
          id: 'first-product',
          title: 'Adding your first product',
          content: 'To add a product, go to your dashboard and click "Add Product". Fill in the title, price, and upload an image. You can also specify variations like size or color. Our AI can even help write the product description for you!',
          categoryId: 'getting-started',
          keywords: ['product', 'add', 'item', 'inventory'],
        ),
      ],
    ),
    HelpCategory(
      id: 'payments',
      title: 'Payments & Billing',
      icon: Icons.payment,
      articles: [
        HelpArticle(
          id: 'accept-payments',
          title: 'How to accept payments',
          content: 'We use Stripe to process payments securely. To start accepting payments, link your bank account in the Settings > Payments section. Once verified, you can accept credit cards, Apple Pay, and Google Pay automatically.',
          categoryId: 'payments',
          keywords: ['payment', 'stripe', 'money', 'card', 'bank'],
        ),
        HelpArticle(
          id: 'refunds',
          title: 'Processing refunds',
          content: 'If a customer needs a refund, simply go to the order details page and click the "Refund" button. You can choose to refund the full amount or just a partial amount. Funds usually return to the customer in 3-5 days.',
          categoryId: 'payments',
          keywords: ['refund', 'return', 'money back'],
        ),
      ],
    ),
    HelpCategory(
      id: 'agents',
      title: 'AI Agents',
      icon: Icons.smart_toy,
      articles: [
        HelpArticle(
          id: 'support-agent',
          title: 'Configuring your Support Agent',
          content: 'Your Support Agent handles customer questions 24/7. To customize it, navigate to the Agents page. You can give it a name, adjust its tone (friendly, professional, etc.), and provide specific instructions on how to answer common questions.',
          categoryId: 'agents',
          keywords: ['ai', 'agent', 'support', 'bot', 'chat'],
        ),
        HelpArticle(
          id: 'marketing-agent',
          title: 'What does the Marketing Agent do?',
          content: 'The Marketing Agent helps you grow by suggesting social media posts, running ad campaigns, and optimizing your storefront for search engines (SEO). It works behind the scenes to bring you more customers.',
          categoryId: 'agents',
          keywords: ['marketing', 'seo', 'ads', 'growth', 'agent'],
        ),
      ],
    ),
  ];

  List<HelpCategory> getHelpCategories() {
    return _categories;
  }

  List<HelpArticle> search(String query) {
    if (query.isEmpty) return [];

    final lowerQuery = query.toLowerCase();
    final results = <HelpArticle>[];

    for (final category in _categories) {
      for (final article in category.articles) {
        if (article.title.toLowerCase().contains(lowerQuery) ||
            article.content.toLowerCase().contains(lowerQuery) ||
            article.keywords.any((k) => k.toLowerCase().contains(lowerQuery))) {
          results.add(article);
        }
      }
    }

    return results;
  }
}
