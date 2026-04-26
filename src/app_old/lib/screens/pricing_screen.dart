import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../widgets/glass_card.dart';

class PricingScreen extends StatelessWidget {
  const PricingScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Pricing & Billing')),
      body: ListView(
        padding: const EdgeInsets.all(24),
        children: const [
          _PricingCard(tier: 'Free', price: '\$0/mo', description: 'Up to 100 AI Actions, 500MB storage.'),
          SizedBox(height: 16),
          _PricingCard(tier: 'Starter', price: '\$29/mo', description: 'Up to 1000 AI Actions, 5GB storage.'),
          SizedBox(height: 16),
          _PricingCard(tier: 'Pro', price: '\$99/mo', description: 'Unlimited AI Actions, 50GB storage.'),
          SizedBox(height: 16),
          _PricingCard(tier: 'Business', price: 'Custom', description: 'Enterprise features and support.'),
        ],
      ),
    );
  }
}

class _PricingCard extends StatelessWidget {
  final String tier;
  final String price;
  final String description;

  const _PricingCard({required this.tier, required this.price, required this.description});

  @override
  Widget build(BuildContext context) {
    return GlassCard(
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(tier, style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold)),
            const SizedBox(height: 8),
            Text(price, style: const TextStyle(fontSize: 20)),
            const SizedBox(height: 8),
            Text(description),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: () {
                if (context.canPop()) {
                  context.pop();
                } else {
                  context.go('/my-plan');
                }
              },
              child: const Text('Select Plan'),
            ),
          ],
        ),
      ),
    );
  }
}
