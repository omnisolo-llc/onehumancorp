import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'glass_card.dart';
import '../models/tier.dart';

class UpgradeBottomSheet extends ConsumerStatefulWidget {
  final String limitReason;

  const UpgradeBottomSheet({super.key, required this.limitReason});

  @override
  ConsumerState<UpgradeBottomSheet> createState() => _UpgradeBottomSheetState();
}

class _UpgradeBottomSheetState extends ConsumerState<UpgradeBottomSheet> {
  bool _isUpgrading = false;

  void _handleUpgrade() async {
    setState(() => _isUpgrading = true);
    // Simulate one-tap native payment processing
    await Future.delayed(const Duration(seconds: 2));
    if (mounted) {
      ref.read(tierProvider.notifier).upgradeToStarter();
      setState(() => _isUpgrading = false);
      Navigator.pop(context);
    }
  }

  @override
  Widget build(BuildContext context) {
    String headline = 'Limit Reached';
    String description = 'You have reached a limit on your current Free plan.';

    if (widget.limitReason == 'max_products') {
      headline = 'Your store is growing fast!';
      description = 'You have reached the limit of 10 products on the Free plan. Upgrade to Starter to add up to 100 products and get a custom domain.';
    } else if (widget.limitReason == 'storage') {
      headline = 'Your gallery is full.';
      description = 'Upgrade to Starter for 10x more space, or delete older photos.';
    }

    return Container(
      constraints: const BoxConstraints(maxWidth: 375),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surface,
        borderRadius: const BorderRadius.vertical(top: Radius.circular(24)),
      ),
      child: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Expanded(
                    child: Text(
                      headline,
                      style: const TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold),
                    ),
                  ),
                  IconButton(
                    icon: const Icon(Icons.close),
                    onPressed: () => Navigator.pop(context),
                  ),
                ],
              ),
              const SizedBox(height: 16),
              Text(
                description,
                style: const TextStyle(fontFamily: 'Inter', fontSize: 16),
              ),
              const SizedBox(height: 24),
              GlassCard(
                child: Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Column(
                    children: [
                      const Text(
                        'Starter Plan',
                        style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold),
                      ),
                      const SizedBox(height: 8),
                      const Text(
                        '\$9/mo',
                        style: TextStyle(fontFamily: 'Outfit', fontSize: 32, fontWeight: FontWeight.bold, color: Colors.greenAccent),
                      ),
                      const SizedBox(height: 16),
                      _isUpgrading
                          ? const Center(child: CircularProgressIndicator())
                          : FilledButton.icon(
                              onPressed: _handleUpgrade,
                              icon: const Icon(Icons.payment),
                              label: const Text('Upgrade with Apple / Google Pay'),
                              style: FilledButton.styleFrom(
                                padding: const EdgeInsets.symmetric(vertical: 16),
                                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                              ),
                            ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
