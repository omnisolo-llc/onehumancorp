import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class HelpCenterScreen extends StatefulWidget {
  const HelpCenterScreen({super.key});

  @override
  State<HelpCenterScreen> createState() => _HelpCenterScreenState();
}

class _HelpCenterScreenState extends State<HelpCenterScreen> {
  final TextEditingController _searchController = TextEditingController();
  List<Map<String, dynamic>> _filteredArticles = [];

  final List<Map<String, dynamic>> _categories = [
    {'icon': Icons.rocket_launch, 'title': 'Getting Started', 'articles': ['How to set up your store', 'Connecting your bank']},
    {'icon': Icons.storefront, 'title': 'My Store', 'articles': ['Adding products', 'Managing inventory']},
    {'icon': Icons.payment, 'title': 'Payments', 'articles': ['Refunding a customer', 'Setting up Apple Pay']},
    {'icon': Icons.smart_toy, 'title': 'AI Agents', 'articles': ['Teaching your agent new skills', 'Reviewing agent handoffs']},
    {'icon': Icons.campaign, 'title': 'Marketing', 'articles': ['Running an email campaign', 'Understanding SEO']},
    {'icon': Icons.account_balance_wallet, 'title': 'Account & Billing', 'articles': ['Updating your subscription', 'Viewing invoices']},
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: CustomScrollView(
        slivers: [
          SliverAppBar(
            expandedHeight: 200.0,
            floating: false,
            pinned: true,
            flexibleSpace: FlexibleSpaceBar(
              title: const Text('Help Center', style: TextStyle(fontWeight: FontWeight.bold)),
              background: Container(
                decoration: BoxDecoration(
                  gradient: LinearGradient(
                    begin: Alignment.topLeft,
                    end: Alignment.bottomRight,
                    colors: [
                      Theme.of(context).colorScheme.primary.withValues(alpha: 0.8),
                      Theme.of(context).colorScheme.secondary.withValues(alpha: 0.8),
                    ],
                  ),
                ),
                child: Center(
                  child: Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 32.0),
                    child: TextField(
                      controller: _searchController,
                      decoration: InputDecoration(
                        hintText: 'Search for help...',
                        prefixIcon: const Icon(Icons.search),
                        filled: true,
                        fillColor: Theme.of(context).colorScheme.surface,
                        border: OutlineInputBorder(
                          borderRadius: BorderRadius.circular(30),
                          borderSide: BorderSide.none,
                        ),
                      ),
                      onChanged: (value) {
                         // Basic search simulation
                         setState(() {
                           if (value.isEmpty) {
                             _filteredArticles = [];
                           } else {
                             _filteredArticles = [
                               {'title': 'Search result for "$value"'}
                             ];
                           }
                         });
                      },
                    ),
                  ),
                ),
              ),
            ),
          ),
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  if (_filteredArticles.isNotEmpty) ...[
                    const Text('Search Results', style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold)),
                    const SizedBox(height: 16),
                    ..._filteredArticles.map((article) => ListTile(
                      title: Text(article['title']),
                      trailing: const Icon(Icons.chevron_right),
                      onTap: () {},
                    )),
                    const Divider(),
                    const SizedBox(height: 16),
                  ],
                  const Text('Browse by Topic', style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold)),
                  const SizedBox(height: 16),
                  LayoutBuilder(
                    builder: (context, constraints) {
                      final crossAxisCount = constraints.maxWidth > 600 ? 3 : 2;
                      return GridView.builder(
                        shrinkWrap: true,
                        physics: const NeverScrollableScrollPhysics(),
                        gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                          crossAxisCount: crossAxisCount,
                          crossAxisSpacing: 16,
                          mainAxisSpacing: 16,
                          childAspectRatio: 1.2,
                        ),
                        itemCount: _categories.length,
                        itemBuilder: (context, index) {
                          final category = _categories[index];
                          return GlassCard(
                            child: InkWell(
                              onTap: () {
                                // Simulate navigation to category
                              },
                              child: Padding(
                                padding: const EdgeInsets.all(16.0),
                                child: Column(
                                  mainAxisAlignment: MainAxisAlignment.center,
                                  children: [
                                    Icon(category['icon'] as IconData, size: 40, color: Theme.of(context).colorScheme.primary),
                                    const SizedBox(height: 12),
                                    Text(
                                      category['title'] as String,
                                      textAlign: TextAlign.center,
                                      style: const TextStyle(fontWeight: FontWeight.bold),
                                    ),
                                  ],
                                ),
                              ),
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
        ],
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () {
            // Placeholder for AI Help Chat
        },
        icon: const Icon(Icons.chat),
        label: const Text('Ask AI Support'),
      ),
    );
  }
}
