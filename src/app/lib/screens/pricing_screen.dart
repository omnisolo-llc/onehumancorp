import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../widgets/glass_card.dart';

class PricingScreen extends ConsumerWidget {
  const PricingScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final colors = Theme.of(context).colorScheme;
    return Scaffold(
      appBar: AppBar(
        title: const Text('Plans & Pricing', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: Center(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(24),
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 1200),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                const Text(
                  'Choose your plan',
                  style: TextStyle(fontSize: 32, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
                  textAlign: TextAlign.center,
                ),
                const SizedBox(height: 16),
                Text(
                  'Simple, transparent pricing for businesses of all sizes.',
                  style: TextStyle(fontSize: 18, color: colors.onSurfaceVariant),
                  textAlign: TextAlign.center,
                ),
                const SizedBox(height: 48),
                Wrap(
                  alignment: WrapAlignment.center,
                  spacing: 24,
                  runSpacing: 24,
                  children: [
                    _PricingTierCard(
                      name: 'Free',
                      price: '\$0',
                      description: 'For individuals just getting started.',
                      features: const [
                        '100 AI actions / month',
                        '500MB Storage',
                        'Standard Support',
                      ],
                      buttonText: 'Current Plan',
                      isCurrent: true,
                    ),
                    _PricingTierCard(
                      name: 'Starter',
                      price: '\$29',
                      description: 'For growing small businesses.',
                      features: const [
                        '1,000 AI actions / month',
                        '5GB Storage',
                        'Priority Email Support',
                        'Custom Domain',
                      ],
                      buttonText: 'Upgrade to Starter',
                      isPopular: true,
                    ),
                    _PricingTierCard(
                      name: 'Pro',
                      price: '\$99',
                      description: 'For established businesses scaling up.',
                      features: const [
                        'Unlimited AI actions',
                        '50GB Storage',
                        '24/7 Phone Support',
                        'Advanced Analytics',
                      ],
                      buttonText: 'Upgrade to Pro',
                    ),
                    _PricingTierCard(
                      name: 'Business',
                      price: '\$299',
                      description: 'For high-volume operations.',
                      features: const [
                        'Dedicated AI Agent',
                        'Unlimited Storage',
                        'Dedicated Success Manager',
                        'Custom Integrations',
                      ],
                      buttonText: 'Contact Sales',
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class _PricingTierCard extends StatelessWidget {
  final String name;
  final String price;
  final String description;
  final List<String> features;
  final String buttonText;
  final bool isPopular;
  final bool isCurrent;

  const _PricingTierCard({
    required this.name,
    required this.price,
    required this.description,
    required this.features,
    required this.buttonText,
    this.isPopular = false,
    this.isCurrent = false,
  });

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 280),
      child: GlassCard(
        child: Container(
          decoration: BoxDecoration(
            border: isPopular ? Border.all(color: colors.primary, width: 2) : null,
            borderRadius: BorderRadius.circular(16),
          ),
          padding: const EdgeInsets.all(24),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              if (isPopular)
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
                  margin: const EdgeInsets.only(bottom: 16),
                  decoration: BoxDecoration(
                    color: colors.primaryContainer,
                    borderRadius: BorderRadius.circular(20),
                  ),
                  child: Text(
                    'Most Popular',
                    style: TextStyle(
                      color: colors.onPrimaryContainer,
                      fontSize: 12,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
              Text(
                name,
                style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
              ),
              const SizedBox(height: 8),
              Wrap(
                crossAxisAlignment: WrapCrossAlignment.end,
                children: [
                  Text(
                    price,
                    style: const TextStyle(fontSize: 48, fontWeight: FontWeight.w900),
                  ),
                  const Padding(
                    padding: EdgeInsets.only(bottom: 8.0, left: 4.0),
                    child: Text('/month', style: TextStyle(fontSize: 16)),
                  ),
                ],
              ),
              const SizedBox(height: 16),
              Text(
                description,
                style: TextStyle(color: colors.onSurfaceVariant),
              ),
              const SizedBox(height: 24),
              SizedBox(
                width: double.infinity,
                child: isCurrent
                    ? OutlinedButton(
                        onPressed: null,
                        child: Text(buttonText),
                      )
                    : FilledButton(
                        onPressed: () {},
                        child: Text(buttonText),
                      ),
              ),
              const SizedBox(height: 24),
              const Divider(),
              const SizedBox(height: 16),
              ...features.map((feature) => Padding(
                    padding: const EdgeInsets.only(bottom: 12),
                    child: Row(
                      children: [
                        Icon(Icons.check_circle, color: colors.primary, size: 20),
                        const SizedBox(width: 12),
                        Expanded(child: Text(feature)),
                      ],
                    ),
                  )),
            ],
          ),
        ),
      ),
    );
  }
}
