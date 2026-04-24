import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class HelpCenterScreen extends StatefulWidget {
  const HelpCenterScreen({super.key});

  @override
  State<HelpCenterScreen> createState() => _HelpCenterScreenState();
}

class _HelpCenterScreenState extends State<HelpCenterScreen> {
  final TextEditingController _searchController = TextEditingController();

  final List<Map<String, dynamic>> _topics = [
    {
      'title': 'Getting Started',
      'icon': Icons.rocket_launch,
      'articles': ['How to set up your store', 'Connecting your domain', 'Choosing a template'],
    },
    {
      'title': 'My Store',
      'icon': Icons.storefront,
      'articles': ['Adding products', 'Managing inventory', 'Setting up variants'],
    },
    {
      'title': 'Payments',
      'icon': Icons.payment,
      'articles': ['Accepting credit cards', 'Setting up Stripe', 'Processing refunds'],
    },
    {
      'title': 'AI Agents',
      'icon': Icons.smart_toy,
      'articles': ['What is an AI agent?', 'Customizing your support agent', 'Automating emails'],
    },
    {
      'title': 'Marketing',
      'icon': Icons.campaign,
      'articles': ['Creating a discount code', 'Running a sale', 'SEO basics'],
    },
    {
      'title': 'Account & Billing',
      'icon': Icons.manage_accounts,
      'articles': ['Changing your plan', 'Updating billing info', 'Managing users'],
    },
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        centerTitle: true,
      ),
      body: SingleChildScrollView(
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 800),
            child: Padding(
              padding: const EdgeInsets.all(24.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  const Text(
                    'How can we help you today?',
                    style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
                    textAlign: TextAlign.center,
                  ),
                  const SizedBox(height: 24),
                  TextField(
                    controller: _searchController,
                    decoration: InputDecoration(
                      hintText: 'Search for articles, guides, and more...',
                      prefixIcon: const Icon(Icons.search),
                      border: OutlineInputBorder(
                        borderRadius: BorderRadius.circular(16),
                      ),
                      filled: true,
                      fillColor: Theme.of(context).colorScheme.surface,
                    ),
                  ),
                  const SizedBox(height: 32),
                  const Text(
                    'Video Tutorials',
                    style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
                  ),
                  const SizedBox(height: 16),
                  SizedBox(
                    height: 180,
                    child: ListView.builder(
                      scrollDirection: Axis.horizontal,
                      itemCount: 4,
                      itemBuilder: (context, index) {
                        return Container(
                          width: 280,
                          margin: const EdgeInsets.only(right: 16),
                          child: GlassCard(
                            padding: const EdgeInsets.all(16),
                            child: Column(
                              mainAxisAlignment: MainAxisAlignment.center,
                              children: [
                                const Icon(Icons.play_circle_fill, size: 48, color: Colors.blueAccent),
                                const SizedBox(height: 16),
                                Text(
                                  'Tutorial ${index + 1}: Basics',
                                  style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Inter'),
                                  textAlign: TextAlign.center,
                                ),
                              ],
                            ),
                          ),
                        );
                      },
                    ),
                  ),
                  const SizedBox(height: 32),
                  const Text(
                    'Browse Topics',
                    style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
                  ),
                  const SizedBox(height: 16),
                  LayoutBuilder(
                    builder: (context, constraints) {
                      int crossAxisCount = constraints.maxWidth > 600 ? 2 : 1;
                      return GridView.builder(
                        shrinkWrap: true,
                        physics: const NeverScrollableScrollPhysics(),
                        gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                          crossAxisCount: crossAxisCount,
                          crossAxisSpacing: 16,
                          mainAxisSpacing: 16,
                          childAspectRatio: 1.5,
                        ),
                        itemCount: _topics.length,
                        itemBuilder: (context, index) {
                          final topic = _topics[index];
                          return GlassCard(
                            padding: const EdgeInsets.all(16),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Row(
                                  children: [
                                    Icon(topic['icon'], color: Theme.of(context).colorScheme.primary),
                                    const SizedBox(width: 8),
                                    Expanded(
                                      child: Text(
                                        topic['title'],
                                        style: const TextStyle(
                                          fontWeight: FontWeight.bold,
                                          fontSize: 18,
                                          fontFamily: 'Outfit',
                                        ),
                                      ),
                                    ),
                                  ],
                                ),
                                const SizedBox(height: 12),
                                ...((topic['articles'] as List<String>).map((article) {
                                  return Padding(
                                    padding: const EdgeInsets.only(bottom: 8.0),
                                    child: InkWell(
                                      onTap: () {},
                                      child: Text(
                                        article,
                                        style: TextStyle(
                                          color: Theme.of(context).colorScheme.primary,
                                          fontFamily: 'Inter',
                                          decoration: TextDecoration.underline,
                                        ),
                                      ),
                                    ),
                                  );
                                })),
                              ],
                            ),
                          );
                        },
                      );
                    },
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
