import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class HelpCenterScreen extends ConsumerStatefulWidget {
  const HelpCenterScreen({super.key});

  @override
  ConsumerState<HelpCenterScreen> createState() => _HelpCenterScreenState();
}

class _HelpCenterScreenState extends ConsumerState<HelpCenterScreen> {
  final TextEditingController _searchController = TextEditingController();
  String _searchQuery = '';

  final List<Map<String, dynamic>> _topics = [
    {
      'icon': Icons.rocket_launch,
      'title': 'Getting Started',
      'articles': [
        'How to set up your business profile',
        'Inviting team members',
        'Choosing the right AI agents',
      ]
    },
    {
      'icon': Icons.storefront,
      'title': 'My Store',
      'articles': [
        'Adding physical products',
        'Managing inventory',
        'Customizing your storefront design',
      ]
    },
    {
      'icon': Icons.payment,
      'title': 'Payments',
      'articles': [
        'Connecting your bank account',
        'Understanding processing fees',
        'Handling refunds',
      ]
    },
    {
      'icon': Icons.smart_toy,
      'title': 'AI Agents',
      'articles': [
        'How does the Marketing Agent work?',
        'Tuning agent responses',
        'Agent handoffs to humans',
      ]
    },
    {
      'icon': Icons.campaign,
      'title': 'Marketing',
      'articles': [
        'SEO optimization tips',
        'Running social media campaigns',
        'Creating discount codes',
      ]
    },
    {
      'icon': Icons.manage_accounts,
      'title': 'Account & Billing',
      'articles': [
        'Upgrading your plan',
        'Viewing billing history',
        'Managing notifications',
      ]
    },
  ];

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.transparent,
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () {
          context.go('/chat?agent=help');
        },
        icon: const Icon(Icons.support_agent),
        label: const Text('Ask AI Help Agent'),
      ),
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            _buildSearchBox(),
            const SizedBox(height: 32),
            _buildTopicsGrid(),
            const SizedBox(height: 32),
            _buildVideoTutorialsSection(),
            const SizedBox(height: 32),
            _buildAdvancedSection(),
          ],
        ),
      ),
    );
  }

  Widget _buildSearchBox() {
    return GlassCard(
      child: TextField(
        controller: _searchController,
        onChanged: (value) => setState(() => _searchQuery = value),
        decoration: InputDecoration(
          hintText: 'Search for articles, guides, and more...',
          prefixIcon: const Icon(Icons.search),
          border: OutlineInputBorder(
            borderRadius: BorderRadius.circular(12),
            borderSide: BorderSide.none,
          ),
          filled: true,
          fillColor: Colors.white.withValues(alpha: 0.1),
        ),
      ),
    );
  }

  Widget _buildTopicsGrid() {
    // Filter logic would go here in a real app based on _searchQuery

    return LayoutBuilder(
      builder: (context, constraints) {
        int crossAxisCount = constraints.maxWidth > 800 ? 3 : (constraints.maxWidth > 500 ? 2 : 1);
        return GridView.builder(
          shrinkWrap: true,
          physics: const NeverScrollableScrollPhysics(),
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: crossAxisCount,
            crossAxisSpacing: 16,
            mainAxisSpacing: 16,
            childAspectRatio: 1.2,
          ),
          itemCount: _topics.length,
          itemBuilder: (context, index) {
            final topic = _topics[index];
            return _TopicCard(topic: topic);
          },
        );
      },
    );
  }

  Widget _buildVideoTutorialsSection() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'Video Tutorials',
          style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold),
        ),
        const SizedBox(height: 16),
        SizedBox(
          height: 180,
          child: ListView.separated(
            scrollDirection: Axis.horizontal,
            itemCount: 5, // Mock data
            separatorBuilder: (context, index) => const SizedBox(width: 16),
            itemBuilder: (context, index) {
              return _VideoThumbnail(index: index);
            },
          ),
        ),
      ],
    );
  }

  Widget _buildAdvancedSection() {
    return GlassCard(
      child: ListTile(
        leading: const Icon(Icons.api, size: 32),
        title: const Text('API Documentation (Advanced)', style: TextStyle(fontWeight: FontWeight.bold)),
        subtitle: const Text('Connect custom checkouts and integrate directly with OHC.'),
        trailing: const Icon(Icons.arrow_forward_ios, size: 16),
        onTap: () {
          // In a real app, this might open a web view or external link
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('Opening interactive API reference...')),
          );
        },
      ),
    );
  }
}

class _TopicCard extends StatelessWidget {
  final Map<String, dynamic> topic;

  const _TopicCard({required this.topic});

  @override
  Widget build(BuildContext context) {
    return GlassCard(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
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
                    style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 18),
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Expanded(
              child: ListView.builder(
                shrinkWrap: true,
                physics: const NeverScrollableScrollPhysics(),
                itemCount: (topic['articles'] as List).length,
                itemBuilder: (context, i) {
                  return Padding(
                    padding: const EdgeInsets.only(bottom: 4.0),
                    child: InkWell(
                      onTap: () {
                         ScaffoldMessenger.of(context).showSnackBar(
                           SnackBar(content: Text('Opening article: ${topic["articles"][i]}')),
                         );
                      },
                      child: Text(
                        '• ${topic["articles"][i]}',
                        style: const TextStyle(color: Colors.blueAccent, decoration: TextDecoration.underline),
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                  );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _VideoThumbnail extends StatelessWidget {
  final int index;

  const _VideoThumbnail({required this.index});

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 240,
      decoration: BoxDecoration(
        color: Colors.black26,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.white24),
      ),
      child: Stack(
        alignment: Alignment.center,
        children: [
          const Icon(Icons.play_circle_fill, size: 48, color: Colors.white54),
          Positioned(
            bottom: 8,
            left: 8,
            right: 8,
            child: Text(
              'Tutorial \${index + 1}',
              style: const TextStyle(fontWeight: FontWeight.bold, backgroundColor: Colors.black45),
            ),
          )
        ],
      ),
    );
  }
}
