import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../models/help_article.dart';

class HelpService {
  final List<HelpArticle> _articles = [
    // Getting Started
    const HelpArticle(
      id: 'gs-1',
      title: 'How to set up your store',
      topic: 'Getting Started',
      content: 'Welcome to OHC! Setting up your store is easy. First, go to the "Setup Wizard" from the main menu. Follow the steps to choose your business type, add your name, and select what you sell. Your AI assistant will automatically generate a beautiful storefront for you.',
      tags: ['setup', 'store', 'begin', 'start'],
    ),
    const HelpArticle(
      id: 'gs-2',
      title: 'Connecting your domain',
      topic: 'Getting Started',
      content: 'You can connect a custom web address (like www.mybusiness.com) to your OHC store. Go to Settings > Domain, and click "Connect Domain". Follow the on-screen instructions to update your DNS settings. It may take up to 24 hours for the connection to work everywhere.',
      tags: ['domain', 'website', 'address', 'url'],
    ),

    // My Store
    const HelpArticle(
      id: 'ms-1',
      title: 'Adding products',
      topic: 'My Store',
      content: 'To add a product, go to your Dashboard and click the "Add Product" button. You can upload photos directly from your phone, set a price, and write a description. The AI can also help write a catchy description if you give it a few keywords!',
      tags: ['product', 'inventory', 'item', 'sell'],
    ),
    const HelpArticle(
      id: 'ms-2',
      title: 'Managing inventory',
      topic: 'My Store',
      content: 'Your Operations Agent automatically tracks how many items you have left. When someone buys an item, the count goes down. If you restock, just go to the product page and update the "Quantity" number.',
      tags: ['inventory', 'stock', 'quantity', 'manage'],
    ),

    // Payments
    const HelpArticle(
      id: 'pay-1',
      title: 'Accepting your first payment',
      topic: 'Payments',
      content: 'Before you can accept money, you need to connect a bank account. Go to Settings > Payments and click "Connect Bank". We use Stripe to securely process all payments. Once connected, customers can pay with credit cards, Apple Pay, or Google Pay.',
      tags: ['payment', 'money', 'bank', 'stripe', 'credit card'],
    ),
    const HelpArticle(
      id: 'pay-2',
      title: 'When do I get paid?',
      topic: 'Payments',
      content: 'After a customer pays, the money usually takes 2-3 business days to appear in your bank account. Your Finance Agent will automatically send you a notification when a payout is on its way.',
      tags: ['payout', 'money', 'bank', 'transfer', 'time'],
    ),

    // AI Agents
    const HelpArticle(
      id: 'ai-1',
      title: 'Activating your AI Support Agent',
      topic: 'AI Agents',
      content: 'Your Customer Success Agent can answer common questions from your customers while you sleep! Go to "Agents", find "Customer Success", and click "Activate". You can review all the messages it drafts before they are sent.',
      tags: ['ai', 'agent', 'support', 'customer service'],
    ),
    const HelpArticle(
      id: 'ai-2',
      title: 'How does the Marketing Agent work?',
      topic: 'AI Agents',
      content: 'The Marketing Agent helps you get noticed. It can write social media posts, suggest promotions, and even help design your website. Just go to "Agents" and chat with the Marketing Agent to tell it what you want to promote.',
      tags: ['ai', 'agent', 'marketing', 'promote', 'social media'],
    ),

    // Marketing
    const HelpArticle(
      id: 'mkt-1',
      title: 'Running a discount or sale',
      topic: 'Marketing',
      content: 'Want to offer 20% off? Just ask your Marketing Agent! Say "Create a 20% off sale for this weekend" in the chat, and it will set up the discount code and even draft an announcement post for your social media.',
      tags: ['sale', 'discount', 'promotion', 'coupon', 'marketing'],
    ),
    const HelpArticle(
      id: 'mkt-2',
      title: 'Understanding your website visitors',
      topic: 'Marketing',
      content: 'Your Business Advisory Agent will send you a simple report every week showing how many people visited your site, what they looked at, and what they bought. No complicated charts to learn!',
      tags: ['analytics', 'visitors', 'traffic', 'stats', 'report'],
    ),

    // Account & Billing
    const HelpArticle(
      id: 'acc-1',
      title: 'Changing your password',
      topic: 'Account & Billing',
      content: 'To change your password, go to Settings > Security and click "Change Password". You will need to enter your old password first. If you forgot your password, sign out and click "Forgot Password" on the login screen.',
      tags: ['password', 'security', 'login', 'account'],
    ),
    const HelpArticle(
      id: 'acc-2',
      title: 'Upgrading your OHC plan',
      topic: 'Account & Billing',
      content: 'Need more features? Go to Settings > Billing and click "Upgrade Plan". You can choose the tier that fits your growing business best. Your card on file will be charged automatically.',
      tags: ['plan', 'upgrade', 'billing', 'subscription', 'price'],
    ),
  ];

  List<HelpArticle> searchArticles(String query) {
    if (query.isEmpty) return _articles;

    final lowerQuery = query.toLowerCase();
    return _articles.where((article) {
      final matchesTitle = article.title.toLowerCase().contains(lowerQuery);
      final matchesContent = article.content.toLowerCase().contains(lowerQuery);
      final matchesTopic = article.topic.toLowerCase().contains(lowerQuery);
      final matchesTag = article.tags.any((tag) => tag.toLowerCase().contains(lowerQuery));

      return matchesTitle || matchesContent || matchesTopic || matchesTag;
    }).toList();
  }

  List<String> getTopics() {
    return _articles.map((a) => a.topic).toSet().toList();
  }

  List<HelpArticle> getArticlesByTopic(String topic) {
    return _articles.where((a) => a.topic == topic).toList();
  }

  HelpArticle? getArticleById(String id) {
    try {
      return _articles.firstWhere((a) => a.id == id);
    } catch (e) {
      return null;
    }
  }
}

final helpServiceProvider = Provider<HelpService>((ref) {
  return HelpService();
});
