import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:intl/intl.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class MyPlanScreen extends ConsumerStatefulWidget {
  const MyPlanScreen({super.key});

  @override
  ConsumerState<MyPlanScreen> createState() => _MyPlanScreenState();
}

class _MyPlanScreenState extends ConsumerState<MyPlanScreen> {
  final String _currentPlan = 'Free';
  final int _actionsUsed = 85;
  final int _actionsLimit = 100;
  final double _storageUsedMB = 450.0;
  final double _storageLimitMB = 500.0;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final currencyFormat = NumberFormat.currency(symbol: '\$');

    return Scaffold(
      appBar: AppBar(
        title: const Text('My Plan & Billing', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text('Current Plan Overview', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
            const SizedBox(height: 16),
            GlassCard(
              child: Padding(
                padding: const EdgeInsets.all(24),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text('Plan: $_currentPlan', style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
                        Text(currencyFormat.format(0.0) + ' / month', style: TextStyle(fontSize: 18, color: colors.primary, fontWeight: FontWeight.bold, fontFamily: 'Inter')),
                      ],
                    ),
                    const Divider(height: 32),
                    _buildProgressBar(context, 'AI Actions Used', _actionsUsed.toDouble(), _actionsLimit.toDouble(), '$_actionsUsed / $_actionsLimit actions'),
                    const SizedBox(height: 24),
                    _buildProgressBar(context, 'Storage Used', _storageUsedMB, _storageLimitMB, '${_storageUsedMB.toStringAsFixed(1)} MB / ${_storageLimitMB.toStringAsFixed(0)} MB'),
                    const SizedBox(height: 32),
                    Text('Estimated Next Bill: ${currencyFormat.format(0.0)}', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w500, fontFamily: 'Inter')),
                    const SizedBox(height: 16),
                    SizedBox(
                      width: double.infinity,
                      child: ElevatedButton(
                        onPressed: () {},
                        style: ElevatedButton.styleFrom(
                          padding: const EdgeInsets.symmetric(vertical: 16),
                        ),
                        child: const Text('Upgrade Plan', style: TextStyle(fontFamily: 'Inter', fontSize: 16, fontWeight: FontWeight.bold)),
                      ),
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 32),
            Text('Available Plans', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
            const SizedBox(height: 16),
            LayoutBuilder(
              builder: (context, constraints) {
                if (constraints.maxWidth > 800) {
                  return Row(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Expanded(child: _buildPlanCard(context, 'Free', 0, 100, '500MB', true)),
                      const SizedBox(width: 16),
                      Expanded(child: _buildPlanCard(context, 'Starter', 29, 1000, '5GB', false)),
                      const SizedBox(width: 16),
                      Expanded(child: _buildPlanCard(context, 'Pro', 99, null, '50GB', false)),
                    ],
                  );
                }
                return Column(
                  children: [
                    _buildPlanCard(context, 'Free', 0, 100, '500MB', true),
                    const SizedBox(height: 16),
                    _buildPlanCard(context, 'Starter', 29, 1000, '5GB', false),
                    const SizedBox(height: 16),
                    _buildPlanCard(context, 'Pro', 99, null, '50GB', false),
                  ],
                );
              },
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildProgressBar(BuildContext context, String label, double value, double limit, String textValue) {
    final colors = Theme.of(context).colorScheme;
    final ratio = limit > 0 ? (value / limit).clamp(0.0, 1.0) : 0.0;
    final isWarning = ratio > 0.8;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(label, style: const TextStyle(fontWeight: FontWeight.w500, fontFamily: 'Inter')),
            Text(textValue, style: const TextStyle(fontFamily: 'Inter')),
          ],
        ),
        const SizedBox(height: 8),
        Stack(
          children: [
            Container(
              height: 12,
              width: double.infinity,
              decoration: BoxDecoration(
                color: colors.surfaceContainerHighest,
                borderRadius: BorderRadius.circular(6),
              ),
            ),
            FractionallySizedBox(
              widthFactor: ratio,
              child: Container(
                height: 12,
                decoration: BoxDecoration(
                  color: isWarning ? colors.error : colors.primary,
                  borderRadius: BorderRadius.circular(6),
                ),
              ),
            ),
          ],
        ),
        if (isWarning)
           Padding(
             padding: const EdgeInsets.only(top: 8),
             child: Text('Approaching limit. Consider upgrading.', style: TextStyle(color: colors.error, fontSize: 12, fontFamily: 'Inter')),
           ),
      ],
    );
  }

  Widget _buildPlanCard(BuildContext context, String name, double price, int? aiLimit, String storage, bool isCurrent) {
    final colors = Theme.of(context).colorScheme;
    return GlassCard(
      child: Container(
        padding: const EdgeInsets.all(24),
        decoration: isCurrent ? BoxDecoration(
          border: Border.all(color: colors.primary, width: 2),
          borderRadius: BorderRadius.circular(16),
        ) : null,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(name, style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
            const SizedBox(height: 8),
            Text('\$${price.toStringAsFixed(0)} / month', style: TextStyle(fontSize: 20, color: colors.primary, fontWeight: FontWeight.w500, fontFamily: 'Inter')),
            const Divider(height: 32),
            _buildFeatureRow(context, aiLimit == null ? 'Unlimited AI Actions' : '$aiLimit AI Actions/month'),
            const SizedBox(height: 12),
            _buildFeatureRow(context, '$storage Storage'),
            const SizedBox(height: 12),
            _buildFeatureRow(context, 'Basic Support'),
            const SizedBox(height: 32),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: isCurrent ? null : () {},
                style: ElevatedButton.styleFrom(
                  backgroundColor: isCurrent ? colors.surfaceContainerHighest : colors.primary,
                  foregroundColor: isCurrent ? colors.onSurface : colors.onPrimary,
                  padding: const EdgeInsets.symmetric(vertical: 16),
                ),
                child: Text(isCurrent ? 'Current Plan' : 'Select $name', style: const TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildFeatureRow(BuildContext context, String text) {
    return Row(
      children: [
        Icon(Icons.check_circle, color: Theme.of(context).colorScheme.primary, size: 20),
        const SizedBox(width: 12),
        Expanded(child: Text(text, style: const TextStyle(fontFamily: 'Inter', fontSize: 15))),
      ],
    );
  }
}
