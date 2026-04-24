import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class HelpArticle {
  final String id;
  final String title;
  final String topic;
  final String content;

  const HelpArticle({
    required this.id,
    required this.title,
    required this.topic,
    required this.content,
  });
}

class HelpRepository {
  static const List<HelpArticle> articles = [
    HelpArticle(
      id: 'get_started',
      title: 'Getting Started with One Human Corp',
      topic: 'Getting Started',
      content: 'Welcome to One Human Corp! To get started, hire your first agent from the Dashboard. Once you have an agent, you can start delegating tasks. Create a virtual meeting room to brainstorm with your AI team.',
    ),
    HelpArticle(
      id: 'store_setup',
      title: 'Setting Up Your Storefront',
      topic: 'My Store',
      content: 'Navigate to the Dashboard and click on the "Business Setup Wizard". This will guide you through creating your storefront, adding products, and customizing your brand identity.',
    ),
    HelpArticle(
      id: 'accept_payments',
      title: 'Accepting Payments with Stripe',
      topic: 'Payments',
      content: 'Connect your Stripe account via the Integrations page. Once connected, your store will automatically be able to process credit cards, Apple Pay, and Google Pay securely.',
    ),
    HelpArticle(
      id: 'ai_agents',
      title: 'How to Manage AI Agents',
      topic: 'AI Agents',
      content: 'Go to the Agents page to see your current team. You can hire new agents for specific roles like Marketing or Finance. If an agent isn\'t performing well, you can review their tasks or fire them.',
    ),
    HelpArticle(
      id: 'marketing_seo',
      title: 'Improving Your SEO',
      topic: 'Marketing',
      content: 'Hire a Marketing Agent. They will automatically optimize your storefront for Google, suggest blog topics, and can even write promotional emails for your customers.',
    ),
    HelpArticle(
      id: 'billing_info',
      title: 'Understanding Your Billing',
      topic: 'Account & Billing',
      content: 'Your billing is based on the resources your agents use. Check the Cost & Usage dashboard to see a detailed breakdown of your daily and monthly expenses.',
    ),
  ];

  static List<String> get topics {
    return articles.map((a) => a.topic).toSet().toList();
  }

  static List<HelpArticle> search(String query) {
    if (query.isEmpty) return articles;
    final lowerQuery = query.toLowerCase();
    return articles.where((a) {
      return a.title.toLowerCase().contains(lowerQuery) ||
             a.topic.toLowerCase().contains(lowerQuery) ||
             a.content.toLowerCase().contains(lowerQuery);
    }).toList();
  }
}

class HelpCenterScreen extends StatefulWidget {
  const HelpCenterScreen({super.key});

  @override
  State<HelpCenterScreen> createState() => _HelpCenterScreenState();
}

class _HelpCenterScreenState extends State<HelpCenterScreen> {
  String _searchQuery = '';
  List<HelpArticle> _filteredArticles = HelpRepository.articles;

  void _onSearchChanged(String query) {
    setState(() {
      _searchQuery = query;
      _filteredArticles = HelpRepository.search(query);
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () {
            if (context.canPop()) {
              context.pop();
            } else {
              context.go('/dashboard');
            }
          },
        ),
      ),
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [
              Color.fromARGB(255, 30, 30, 40),
              Color.fromARGB(255, 15, 15, 20),
            ],
          ),
        ),
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              TextField(
                onChanged: _onSearchChanged,
                style: const TextStyle(color: Colors.white),
                decoration: InputDecoration(
                  hintText: 'Search for help topics...',
                  hintStyle: const TextStyle(color: Colors.white54),
                  prefixIcon: const Icon(Icons.search, color: Colors.white54),
                  filled: true,
                  fillColor: Colors.white.withOpacity(0.1),
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(12),
                    borderSide: BorderSide.none,
                  ),
                ),
              ),
              const SizedBox(height: 24),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                children: [
                  ElevatedButton.icon(
                    onPressed: () => context.push('/video_tutorials'),
                    icon: const Icon(Icons.play_circle_outline),
                    label: const Text('Videos'),
                    style: ElevatedButton.styleFrom(backgroundColor: Colors.white10, foregroundColor: Colors.white),
                  ),
                  ElevatedButton.icon(
                    onPressed: () => context.push('/api_docs'),
                    icon: const Icon(Icons.code),
                    label: const Text('API Docs'),
                    style: ElevatedButton.styleFrom(backgroundColor: Colors.white10, foregroundColor: Colors.white),
                  ),
                  ElevatedButton.icon(
                    onPressed: () => context.push('/release_notes'),
                    icon: const Icon(Icons.new_releases_outlined),
                    label: const Text('What\'s New'),
                    style: ElevatedButton.styleFrom(backgroundColor: Colors.white10, foregroundColor: Colors.white),
                  ),
                ],
              ),
              const SizedBox(height: 24),
              Expanded(
                child: _filteredArticles.isEmpty
                    ? const Center(
                        child: Text(
                          'No articles found.',
                          style: TextStyle(color: Colors.white54, fontSize: 16),
                        ),
                      )
                    : ListView.builder(
                        itemCount: _filteredArticles.length,
                        itemBuilder: (context, index) {
                          final article = _filteredArticles[index];
                          return Padding(
                            padding: const EdgeInsets.only(bottom: 16.0),
                            child: GlassCard(
                              child: Padding(
                                padding: const EdgeInsets.all(16.0),
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    Container(
                                      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                                      decoration: BoxDecoration(
                                        color: Colors.indigo.withOpacity(0.3),
                                        borderRadius: BorderRadius.circular(4),
                                      ),
                                      child: Text(
                                        article.topic,
                                        style: const TextStyle(
                                          color: Colors.indigoAccent,
                                          fontSize: 12,
                                          fontWeight: FontWeight.bold,
                                        ),
                                      ),
                                    ),
                                    const SizedBox(height: 8),
                                    Text(
                                      article.title,
                                      style: const TextStyle(
                                        color: Colors.white,
                                        fontSize: 18,
                                        fontWeight: FontWeight.bold,
                                      ),
                                    ),
                                    const SizedBox(height: 8),
                                    Text(
                                      article.content,
                                      style: const TextStyle(
                                        color: Colors.white70,
                                        fontSize: 14,
                                      ),
                                    ),
                                  ],
                                ),
                              ),
                            ),
                          );
                        },
                      ),
              ),
            ],
          ),
        ),
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () {
          context.push('/chat?agent=help_agent');
        },
        icon: const Icon(Icons.chat),
        label: const Text('Ask AI Agent'),
        backgroundColor: Colors.indigoAccent,
      ),
    );
  }
}
