import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

class HelpCenterScreen extends StatefulWidget {
  const HelpCenterScreen({super.key});

  @override
  State<HelpCenterScreen> createState() => _HelpCenterScreenState();
}

class _HelpCenterScreenState extends State<HelpCenterScreen> {
  String _searchQuery = '';

  final List<Map<String, dynamic>> _allTopics = [
    {'title': 'Getting Started', 'icon': Icons.rocket_launch, 'desc': 'Learn the basics of setting up your business on OHC.'},
    {'title': 'My Store', 'icon': Icons.storefront, 'desc': 'Manage your products, services, and inventory.'},
    {'title': 'Payments', 'icon': Icons.payment, 'desc': 'Configure how you get paid and manage refunds.'},
    {'title': 'AI Agents', 'icon': Icons.smart_toy, 'desc': 'Learn how your AI assistants work for you.'},
    {'title': 'Marketing', 'icon': Icons.campaign, 'desc': 'Grow your customer base and launch campaigns.'},
    {'title': 'Account & Billing', 'icon': Icons.account_circle, 'desc': 'Manage your subscription and team members.'},
  ];

  @override
  Widget build(BuildContext context) {
    final filteredTopics = _searchQuery.isEmpty
        ? _allTopics
        : _allTopics.where((t) => (t['title'] as String).toLowerCase().contains(_searchQuery.toLowerCase()) ||
                                  (t['desc'] as String).toLowerCase().contains(_searchQuery.toLowerCase())).toList();

    return Scaffold(
      appBar: AppBar(title: const Text('Help Center')),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: TextField(
              decoration: InputDecoration(
                hintText: 'Search for help articles...',
                prefixIcon: const Icon(Icons.search),
                border: OutlineInputBorder(borderRadius: BorderRadius.circular(12)),
              ),
              onChanged: (val) {
                setState(() {
                  _searchQuery = val;
                });
              },
            ),
          ),
          Expanded(
            child: ListView.builder(
              padding: const EdgeInsets.symmetric(horizontal: 16.0),
              itemCount: filteredTopics.length,
              itemBuilder: (context, index) {
                final topic = filteredTopics[index];
                return _HelpTopicCard(
                  title: topic['title'] as String,
                  icon: topic['icon'] as IconData,
                  description: topic['desc'] as String,
                  onTap: () {
                      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Opening articles for ${topic['title']}')));
                  },
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}

class _HelpTopicCard extends StatelessWidget {
  final String title;
  final String description;
  final IconData icon;
  final VoidCallback onTap;

  const _HelpTopicCard({
    required this.title,
    required this.description,
    required this.icon,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      child: ListTile(
        leading: Icon(icon, color: Theme.of(context).colorScheme.primary),
        title: Text(title, style: const TextStyle(fontWeight: FontWeight.bold)),
        subtitle: Text(description),
        trailing: const Icon(Icons.chevron_right),
        onTap: onTap,
      ),
    );
  }
}
