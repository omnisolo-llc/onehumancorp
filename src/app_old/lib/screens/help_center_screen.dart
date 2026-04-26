import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../widgets/glass_card.dart';

class HelpCenterScreen extends StatelessWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Browse by Topic',
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
            ),
            const SizedBox(height: 16),
            GlassCard(
              child: ListTile(
                leading: const Icon(Icons.rocket_launch, color: Colors.blue),
                title: const Text('Getting Started'),
                subtitle: const Text('Learn the basics of OneHumanCorp.'),
                trailing: const Icon(Icons.arrow_forward_ios, size: 16),
                onTap: () {},
              ),
            ),
            const SizedBox(height: 8),
            GlassCard(
              child: ListTile(
                leading: const Icon(Icons.store, color: Colors.green),
                title: const Text('My Store'),
                subtitle: const Text('Manage products, inventory, and design.'),
                trailing: const Icon(Icons.arrow_forward_ios, size: 16),
                onTap: () {},
              ),
            ),
            const SizedBox(height: 8),
            GlassCard(
              child: ListTile(
                leading: const Icon(Icons.payment, color: Colors.purple),
                title: const Text('Payments'),
                subtitle: const Text('Accept payments, payouts, and billing.'),
                trailing: const Icon(Icons.arrow_forward_ios, size: 16),
                onTap: () {},
              ),
            ),
            const SizedBox(height: 32),
            const Text(
              'Quick Links',
              style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
            ),
            const SizedBox(height: 16),
            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: [
                ActionChip(
                  avatar: const Icon(Icons.code, size: 16),
                  label: const Text('API Documentation'),
                  onPressed: () {
                     context.go('/help/api-docs');
                  },
                ),
                ActionChip(
                  avatar: const Icon(Icons.new_releases, size: 16),
                  label: const Text('Release Notes'),
                  onPressed: () {
                     context.go('/help/changelog');
                  },
                ),
              ],
            )
          ],
        ),
      ),
    );
  }
}
