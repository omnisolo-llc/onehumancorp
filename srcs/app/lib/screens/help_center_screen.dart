import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'dart:ui';

class HelpCenterScreen extends StatefulWidget {
  const HelpCenterScreen({super.key});

  @override
  State<HelpCenterScreen> createState() => _HelpCenterScreenState();
}

class _HelpCenterScreenState extends State<HelpCenterScreen> {
  final TextEditingController _searchController = TextEditingController();
  String _searchQuery = '';

  final List<Map<String, dynamic>> _articles = [
    {
      'title': 'Getting Started: Launch your store in 5 minutes',
      'category': 'Getting Started',
      'icon': Icons.rocket_launch,
      'content': 'Welcome to OneHumanCorp! Here is how to quickly set up your first store without writing any code...',
    },
    {
      'title': 'How to accept payments with Stripe',
      'category': 'Payments',
      'icon': Icons.payment,
      'content': 'We partner with Stripe to make payments easy. Just go to Settings > Payments and click Connect...',
    },
    {
      'title': 'What are AI Agents and how do they work?',
      'category': 'AI Agents',
      'icon': Icons.smart_toy,
      'content': 'Think of AI agents as your digital employees. The Sales Agent talks to customers, while the Operations Agent manages your inventory...',
    },
    {
      'title': 'How to set up your domain name',
      'category': 'My Store',
      'icon': Icons.domain,
      'content': 'Want your own custom web address? Navigate to Store Settings and enter your purchased domain. Our AI handles the DNS configuration...',
    },
    {
      'title': 'Creating a social media marketing campaign',
      'category': 'Marketing',
      'icon': Icons.campaign,
      'content': 'Ask the Promoter Agent to create a campaign for you. It will generate Instagram posts, Facebook ads, and email newsletters automatically.',
    },
    {
      'title': 'Understanding your monthly bill',
      'category': 'Account & Billing',
      'icon': Icons.receipt,
      'content': 'Your monthly bill includes your subscription tier plus any extra AI computation time you used beyond the free limit. View your usage dashboard for details.',
    },
  ];

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final filteredArticles = _articles.where((article) {
      final titleMatch = article['title'].toLowerCase().contains(_searchQuery.toLowerCase());
      final categoryMatch = article['category'].toLowerCase().contains(_searchQuery.toLowerCase());
      return titleMatch || categoryMatch;
    }).toList();

    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      extendBodyBehindAppBar: true,
      body: SafeArea(
        child: LayoutBuilder(
          builder: (context, constraints) {
            // Determine grid columns based on available width
            int crossAxisCount = 1;
            if (constraints.maxWidth > 600) crossAxisCount = 2;
            if (constraints.maxWidth > 900) crossAxisCount = 3;

            return Padding(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'How can we help you today?',
                    style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                          fontFamily: 'Outfit',
                          fontWeight: FontWeight.bold,
                        ),
                  ),
                  const SizedBox(height: 16),
                  _buildSearchBar(),
                  const SizedBox(height: 24),
                  Expanded(
                    child: filteredArticles.isEmpty
                        ? const Center(
                            child: Text(
                              'No articles found.',
                              style: TextStyle(fontFamily: 'Outfit', fontSize: 16),
                            ),
                          )
                        : GridView.builder(
                            gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                              crossAxisCount: crossAxisCount,
                              crossAxisSpacing: 16,
                              mainAxisSpacing: 16,
                              childAspectRatio: 1.5,
                            ),
                            itemCount: filteredArticles.length,
                            itemBuilder: (context, index) {
                              final article = filteredArticles[index];
                              return _buildArticleCard(article);
                            },
                          ),
                  ),
                ],
              ),
            );
          },
        ),
      ),
    );
  }

  Widget _buildSearchBar() {
    return GlassCard(
      padding: EdgeInsets.zero,
      child: TextField(
        controller: _searchController,
        onChanged: (value) {
          setState(() {
            _searchQuery = value;
          });
        },
        decoration: InputDecoration(
          hintText: 'Search for articles, guides, or features...',
          hintStyle: const TextStyle(fontFamily: 'Outfit'),
          prefixIcon: const Icon(Icons.search),
          suffixIcon: _searchQuery.isNotEmpty
              ? IconButton(
                  icon: const Icon(Icons.clear),
                  onPressed: () {
                    _searchController.clear();
                    setState(() {
                      _searchQuery = '';
                    });
                  },
                )
              : null,
          border: InputBorder.none,
          contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 16),
        ),
        style: const TextStyle(fontFamily: 'Outfit'),
      ),
    );
  }

  Widget _buildArticleCard(Map<String, dynamic> article) {
    return GlassCard(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(article['icon'], color: Theme.of(context).colorScheme.primary),
              const SizedBox(width: 8),
              Expanded(
                child: Text(
                  article['category'],
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 12,
                    fontWeight: FontWeight.bold,
                    color: Theme.of(context).colorScheme.primary,
                  ),
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
          Text(
            article['title'],
            style: const TextStyle(
              fontFamily: 'Outfit',
              fontSize: 16,
              fontWeight: FontWeight.bold,
            ),
            maxLines: 2,
            overflow: TextOverflow.ellipsis,
          ),
          const SizedBox(height: 8),
          Expanded(
            child: Text(
              article['content'],
              style: const TextStyle(
                fontFamily: 'Outfit',
                fontSize: 14,
                color: Colors.grey,
              ),
              maxLines: 3,
              overflow: TextOverflow.ellipsis,
            ),
          ),
          const SizedBox(height: 8),
          TextButton(
            onPressed: () {
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(content: Text('Opening article: ${article['title']}')),
              );
            },
            style: TextButton.styleFrom(
              padding: EdgeInsets.zero,
              minimumSize: Size.zero,
              alignment: Alignment.centerLeft,
            ),
            child: const Text('Read article →', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
          ),
        ],
      ),
    );
  }
}
