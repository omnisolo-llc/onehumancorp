import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../widgets/glass_card.dart';
import 'package:ohc_app/services/api_service.dart';

class PricingScreen extends ConsumerWidget {
  const PricingScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: AppBar(title: const Text('Pricing & Plans')),
      body: Center(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(24),
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 800),
            child: Column(
              children: [
                Text(
                  'Simple, transparent pricing',
                  style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                    fontFamily: 'Outfit',
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 32),
                Wrap(
                  spacing: 24,
                  runSpacing: 24,
                  alignment: WrapAlignment.center,
                  children: [
                    _PricingCard(
                      title: 'Free',
                      price: '\$0',
                      features: const [
                        '100 AI actions / month',
                        '500MB Storage',
                        'Basic support',
                      ],
                      buttonText: 'Current Plan',
                      isCurrent: true,
                      onPressed: null,
                    ),
                    _PricingCard(
                      title: 'Starter',
                      price: '\$29',
                      features: const [
                        '1,000 AI actions / month',
                        '5GB Storage',
                        'Priority support',
                      ],
                      buttonText: 'Upgrade',
                      isCurrent: false,
                      onPressed: () async {
                        await ref.read(apiServiceProvider)!.upgradePlan('Starter');
                        ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Plan upgraded to Starter')));
                        context.go('/');
                      },
                    ),
                    _PricingCard(
                      title: 'Pro',
                      price: '\$99',
                      features: const [
                        'Unlimited AI actions',
                        '50GB Storage',
                        '24/7 Phone support',
                      ],
                      buttonText: 'Upgrade',
                      isCurrent: false,
                      onPressed: () async {
                        await ref.read(apiServiceProvider)!.upgradePlan('Pro');
                        ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Plan upgraded to Pro')));
                        context.go('/');
                      },
                    ),
                    _PricingCard(
                      title: 'Business',
                      price: '\$299',
                      features: const [
                        'Unlimited AI actions',
                        'Custom Storage',
                        'Dedicated account manager',
                      ],
                      buttonText: 'Upgrade',
                      isCurrent: false,
                      onPressed: () async {
                        await ref.read(apiServiceProvider)!.upgradePlan('Business');
                        ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Plan upgraded to Business')));
                        context.go('/');
                      },
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

class _PricingCard extends StatelessWidget {
  final String title;
  final String price;
  final List<String> features;
  final String buttonText;
  final bool isCurrent;
  final VoidCallback? onPressed;

  const _PricingCard({
    required this.title,
    required this.price,
    required this.features,
    required this.buttonText,
    required this.isCurrent,
    this.onPressed,
  });

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 250,
      child: GlassCard(
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                title,
                style: const TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(height: 16),
              Text(
                price,
                style: const TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 36,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const Text('/month', style: TextStyle(color: Colors.grey)),
              const SizedBox(height: 24),
              ...features.map((f) => Padding(
                padding: const EdgeInsets.only(bottom: 8),
                child: Row(
                  children: [
                    const Icon(Icons.check, size: 16, color: Colors.green),
                    const SizedBox(width: 8),
                    Expanded(
                      child: Text(
                        f,
                        style: const TextStyle(fontFamily: 'Inter', fontSize: 14),
                      ),
                    ),
                  ],
                ),
              )),
              const SizedBox(height: 24),
              SizedBox(
                width: double.infinity,
                child: isCurrent
                    ? OutlinedButton(
                        onPressed: null,
                        child: Text(buttonText),
                      )
                    : FilledButton(
                        onPressed: onPressed,
                        child: Text(buttonText),
                      ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
