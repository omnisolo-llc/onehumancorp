import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:intl/intl.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/services/api_service.dart';
import '../widgets/glass_card.dart';

class MyPlanScreen extends ConsumerStatefulWidget {
  const MyPlanScreen({super.key});
  @override
  ConsumerState<MyPlanScreen> createState() => _MyPlanScreenState();
}

class _MyPlanScreenState extends ConsumerState<MyPlanScreen> {
  late Future<DashboardSnapshot> _dashboardFuture;

  @override
  void initState() {
    super.initState();
    _dashboardFuture = ref.read(apiServiceProvider)!.getDashboard();
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final currencyFormat = NumberFormat.currency(symbol: '\$');

    return Scaffold(
      appBar: AppBar(title: const Text('My Plan')),
      body: FutureBuilder<DashboardSnapshot>(
        future: _dashboardFuture,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return const Center(child: CircularProgressIndicator());
          }
          if (snapshot.hasError) {
            return Center(child: Text('Error: \${snapshot.error}'));
          }

          final data = snapshot.data!;
          final tier = data.organization.tier;
          final totalActions = data.costs.totalActions;
          final actionLimit = data.organization.actionLimit;
          final usedStorageMB = data.storage != null ? (data.storage!.usedBytes / 1000000).toStringAsFixed(1) : "0.0";
          final limitStorageGB = data.storage != null ? (data.storage!.limitBytes / 1000000000).toStringAsFixed(1) : "0.5";
          final estimatedBill = currencyFormat.format(data.costs.totalCostUSD);

          return ListView(
            padding: const EdgeInsets.all(24),
            children: [
              GlassCard(
                child: Padding(
                  padding: const EdgeInsets.all(20),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text('Current Plan', style: TextStyle(fontSize: 16, color: colors.onSurfaceVariant)),
                      Text(tier, style: const TextStyle(fontSize: 28, fontWeight: FontWeight.bold)),
                      const SizedBox(height: 16),
                      ElevatedButton(
                        onPressed: () => context.go('/pricing'),
                        child: const Text('Upgrade'),
                      ),
                    ],
                  ),
                ),
              ),
              const SizedBox(height: 24),
              GlassCard(
                child: Padding(
                  padding: const EdgeInsets.all(20),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text('Usage This Month', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
                      const SizedBox(height: 16),
                      Text('AI Actions: \$totalActions / \${actionLimit < 0 ? "Unlimited" : actionLimit}'),
                      const SizedBox(height: 8),
                      Text('Storage Used: \$usedStorageMB MB / \$limitStorageGB GB'),
                      const SizedBox(height: 8),
                      Text('Estimated Next Bill: \$estimatedBill'),
                    ],
                  ),
                ),
              ),
            ],
          );
        },
      ),
    );
  }
}
