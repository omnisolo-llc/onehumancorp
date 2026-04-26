import 'package:flutter/material.dart';

class HelpCenterScreen extends StatelessWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          TextField(
            decoration: InputDecoration(
              hintText: 'Search for help...',
              prefixIcon: const Icon(Icons.search),
              border: OutlineInputBorder(
                borderRadius: BorderRadius.circular(24),
              ),
            ),
          ),
          const SizedBox(height: 24),
          _HelpCategory(icon: Icons.rocket_launch, title: 'Getting Started', description: 'Basics of setting up your store.'),
          _HelpCategory(icon: Icons.storefront, title: 'My Store', description: 'Manage your products and inventory.'),
          _HelpCategory(icon: Icons.payment, title: 'Payments', description: 'Handling transactions and payouts.'),
          _HelpCategory(icon: Icons.smart_toy, title: 'AI Agents', description: 'Configure your automated helpers.'),
          _HelpCategory(icon: Icons.campaign, title: 'Marketing', description: 'Grow your customer base.'),
          _HelpCategory(icon: Icons.account_circle, title: 'Account & Billing', description: 'Manage your subscription and settings.'),
        ],
      ),
    );
  }
}

class _HelpCategory extends StatelessWidget {
  final IconData icon;
  final String title;
  final String description;

  const _HelpCategory({required this.icon, required this.title, required this.description});

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.only(bottom: 16),
      elevation: 0,
      shape: RoundedRectangleBorder(
        side: BorderSide(color: Theme.of(context).colorScheme.outlineVariant),
        borderRadius: BorderRadius.circular(12),
      ),
      child: ListTile(
        leading: Icon(icon, color: Theme.of(context).colorScheme.primary),
        title: Text(title, style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        subtitle: Text(description, style: const TextStyle(fontFamily: 'Inter')),
        trailing: const Icon(Icons.chevron_right),
        onTap: () {},
      ),
    );
  }
}
