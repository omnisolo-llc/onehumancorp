import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import '../models/tier.dart';
import '../widgets/upgrade_bottom_sheet.dart';

class ManageProductsScreen extends ConsumerWidget {
  const ManageProductsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tierState = ref.watch(tierProvider);
    final isLimitReached = tierState.productsCount >= tierState.maxProducts;

    return Scaffold(
      appBar: AppBar(
        title: const Text('Manage Products', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 800),
          child: ListView(
            padding: const EdgeInsets.all(24),
            children: [
              GlassCard(
                child: Padding(
                  padding: const EdgeInsets.all(24.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        'Your Products (${tierState.productsCount} / ${tierState.maxProducts})',
                        style: const TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold),
                      ),
                      const SizedBox(height: 16),
                      FilledButton.icon(
                        onPressed: () {
                          if (isLimitReached) {
                            showModalBottomSheet(
                              context: context,
                              isScrollControlled: true,
                              backgroundColor: Colors.transparent,
                              builder: (context) => const UpgradeBottomSheet(limitReason: 'max_products'),
                            );
                          } else {
                            ref.read(tierProvider.notifier).addProduct();
                          }
                        },
                        icon: const Icon(Icons.add),
                        label: const Text('Add Product'),
                      ),
                      const SizedBox(height: 24),
                      ListView.builder(
                        shrinkWrap: true,
                        physics: const NeverScrollableScrollPhysics(),
                        itemCount: tierState.productsCount,
                        itemBuilder: (context, index) {
                          return ListTile(
                            leading: const Icon(Icons.inventory_2),
                            title: Text('Product ${index + 1}'),
                            subtitle: const Text('\$10.00'),
                          );
                        },
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
