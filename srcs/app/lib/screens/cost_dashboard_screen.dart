import '../widgets/glass_card.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:intl/intl.dart';
import 'package:ohc_app/models/agent.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/services/api_service.dart';

/// Screen for financial analytics and token usage monitoring.
class CostDashboardScreen extends ConsumerStatefulWidget {
  const CostDashboardScreen({super.key});

  @override
  ConsumerState<CostDashboardScreen> createState() =>
      _CostDashboardScreenState();
}

class _CostDashboardScreenState extends ConsumerState<CostDashboardScreen> {
  late Future<DashboardSnapshot> _dashboardFuture;

  @override
  void initState() {
    super.initState();
    _refresh();
  }

  void _refresh() {
    setState(() {
      _dashboardFuture = ref.read(apiServiceProvider)!.getDashboard();
    });
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final currencyFormat = NumberFormat.currency(symbol: '\$');

    return Scaffold(
      appBar: AppBar(
        title: const Text('Cost & Token Usage'),
        actions: [
          FutureBuilder<DashboardSnapshot>(
            future: _dashboardFuture,
            builder: (context, snapshot) {
              final isRefreshing =
                  snapshot.connectionState == ConnectionState.waiting;
              return IconButton(
                onPressed: isRefreshing ? null : _refresh,
                icon:
                    isRefreshing
                        ? const SizedBox(
                          width: 20,
                          height: 20,
                          child: CircularProgressIndicator(strokeWidth: 2),
                        )
                        : const Icon(Icons.refresh),
                tooltip: 'Refresh costs',
              );
            },
          ),
        ],
      ),
      body: FutureBuilder<DashboardSnapshot>(
        future: _dashboardFuture,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return Center(
              child: CircularProgressIndicator(
                color: Theme.of(context).colorScheme.primary,
              ),
            );
          }

          if (snapshot.hasError) {
            return Center(child: Text('Error: ${snapshot.error}'));
          }

          final data = snapshot.data!;
          final costs = data.costs;

          return ListView(
            padding: const EdgeInsets.all(24),
            children: [
              // Summary Cards
              Row(
                children: [
                  Expanded(
                    child: _SummaryCard(
                      title: 'Total Spend',
                      value: currencyFormat.format(costs.totalCostUSD),
                      icon: Icons.account_balance_wallet,
                      color: colors.primary,
                    ),
                  ),
                  const SizedBox(width: 16),
                  Expanded(
                    child: _SummaryCard(
                      title: 'Total Tokens',
                      value: NumberFormat.compact().format(costs.totalTokens),
                      icon: Icons.generating_tokens,
                      color: colors.secondary,
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 32),

              // My Plan Section
              Text(
                'My Plan',
                style: Theme.of(
                  context,
                ).textTheme.titleLarge?.copyWith(fontWeight: FontWeight.bold),
              ),
              const SizedBox(height: 16),
              GlassCard(
                child: Padding(
                  padding: const EdgeInsets.all(20),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Text(
                            'Current Plan: ${data.organization.tier}',
                            style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 18),
                          ),
                          ElevatedButton(
                            onPressed: () {
                              context.go('/cost/pricing');
                            },
                            child: const Text('Upgrade'),
                          ),
                        ],
                      ),
                      const SizedBox(height: 16),
                      _ProgressBar(
                        label: 'AI Actions',
                        used: costs.actionUsed.toDouble(),
                        total: costs.actionQuota.toDouble(),
                        usedLabel: '${costs.actionUsed}',
                        totalLabel: '${costs.actionQuota} Actions',
                        color: colors.primary,
                      ),
                      const SizedBox(height: 16),
                      _ProgressBar(
                        label: 'Storage',
                        used: data.organization.storageUsed.toDouble(),
                        total: data.organization.storageQuota.toDouble(),
                        usedLabel: _formatBytes(data.organization.storageUsed),
                        totalLabel: _formatBytes(data.organization.storageQuota),
                        color: colors.secondary,
                      ),
                      const SizedBox(height: 16),
                      Row(
                        children: [
                           Icon(Icons.savings, color: colors.tertiary, size: 20),
                           const SizedBox(width: 8),
                           Text(
                             'Storage Compression Savings: ${currencyFormat.format(costs.storageSavings)}',
                             style: TextStyle(color: colors.tertiary, fontWeight: FontWeight.w500),
                           )
                        ]
                      ),
                      const SizedBox(height: 8),
                      Text(
                         'Estimated Next Bill: ${currencyFormat.format(costs.totalCostUSD)}',
                         style: const TextStyle(color: Colors.grey, fontSize: 12),
                      )
                    ],
                  ),
                ),
              ),
              const SizedBox(height: 32),

              // Usage per Agent Chart
              Text(
                'Usage per Agent',
                style: Theme.of(
                  context,
                ).textTheme.titleLarge?.copyWith(fontWeight: FontWeight.bold),
              ),
              const SizedBox(height: 16),
              GlassCard(
                child: Padding(
                  padding: const EdgeInsets.all(20),
                  child: Column(
                    children:
                        costs.agents.map((agentCost) {
                          final agent = data.agents.firstWhere(
                            (a) => a.id == agentCost.agentId,
                            orElse:
                                () => Agent(
                                  id: agentCost.agentId,
                                  name: 'Unknown Agent',
                                  role: '',
                                  status: '',
                                  organizationId: '',
                                  createdAt: DateTime.now(),
                                ),
                          );

                          final ratio =
                              costs.totalCostUSD > 0
                                  ? agentCost.costUSD / costs.totalCostUSD
                                  : 0.0;

                          return Semantics(
                            label:
                                'Usage for ${agent.name}: ${currencyFormat.format(agentCost.costUSD)}, ${NumberFormat.compact().format(agentCost.tokenUsed)} tokens',
                            child: Padding(
                              padding: const EdgeInsets.only(bottom: 16),
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  Row(
                                    mainAxisAlignment:
                                        MainAxisAlignment.spaceBetween,
                                    children: [
                                      Text(
                                        agent.name,
                                        style: const TextStyle(
                                          fontWeight: FontWeight.w500,
                                        ),
                                      ),
                                      Text(
                                        currencyFormat.format(
                                          agentCost.costUSD,
                                        ),
                                      ),
                                    ],
                                  ),
                                  const SizedBox(height: 8),
                                  Stack(
                                    children: [
                                      Container(
                                        height: 8,
                                        width: double.infinity,
                                        decoration: BoxDecoration(
                                          color: colors.surfaceContainerHighest,
                                          borderRadius: BorderRadius.circular(
                                            4,
                                          ),
                                        ),
                                      ),
                                      FractionallySizedBox(
                                        widthFactor: ratio.clamp(0.0, 1.0),
                                        child: Container(
                                          height: 8,
                                          decoration: BoxDecoration(
                                            color: colors.primary,
                                            borderRadius: BorderRadius.circular(
                                              4,
                                            ),
                                          ),
                                        ),
                                      ),
                                    ],
                                  ),
                                  const SizedBox(height: 4),
                                  Text(
                                    '${NumberFormat.compact().format(agentCost.tokenUsed)} tokens',
                                    style: TextStyle(
                                      fontSize: 10,
                                      color: colors.onSurfaceVariant,
                                    ),
                                  ),
                                ],
                              ),
                            ),
                          );
                        }).toList(),
                  ),
                ),
              ),

              const SizedBox(height: 32),
              // Organization Hierarchy Preview
              Text(
                'Organization View',
                style: Theme.of(
                  context,
                ).textTheme.titleLarge?.copyWith(fontWeight: FontWeight.bold),
              ),
              const SizedBox(height: 16),
              GlassCard(
                child: Padding(
                  padding: const EdgeInsets.all(20),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        children: [
                          const Icon(Icons.business, size: 20),
                          const SizedBox(width: 8),
                          Text(
                            data.organization.name,
                            style: const TextStyle(fontWeight: FontWeight.bold),
                          ),
                          const Spacer(),
                          Text(data.organization.domain),
                        ],
                      ),
                      const Divider(height: 32),
                      ...data.organization.members
                          .take(3)
                          .map(
                            (m) => ListTile(
                              leading: Icon(
                                m.isHuman ? Icons.person : Icons.smart_toy,
                                size: 20,
                              ),
                              title: Text(m.name),
                              subtitle: Text(m.role),
                              dense: true,
                            ),
                          ),
                      if (data.organization.members.length > 3)
                        Center(
                          child: TextButton(
                            onPressed: () {},
                            child: const Text('View Full Org Tree'),
                          ),
                        ),
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

String _formatBytes(int bytes) {
  if (bytes < 1024) return '$bytes B';
  if (bytes < 1024 * 1024) return '${(bytes / 1024).toStringAsFixed(1)} KB';
  if (bytes < 1024 * 1024 * 1024) return '${(bytes / (1024 * 1024)).toStringAsFixed(1)} MB';
  return '${(bytes / (1024 * 1024 * 1024)).toStringAsFixed(1)} GB';
}

class _ProgressBar extends StatelessWidget {
  final String label;
  final double used;
  final double total;
  final String usedLabel;
  final String totalLabel;
  final Color color;

  const _ProgressBar({
    required this.label,
    required this.used,
    required this.total,
    required this.usedLabel,
    required this.totalLabel,
    required this.color,
  });

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final ratio = total > 0 ? (used / total).clamp(0.0, 1.0) : 0.0;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(label, style: const TextStyle(fontWeight: FontWeight.w500)),
            Text('$usedLabel / $totalLabel', style: TextStyle(color: colors.onSurfaceVariant, fontSize: 12)),
          ],
        ),
        const SizedBox(height: 8),
        LinearProgressIndicator(
          value: ratio,
          backgroundColor: colors.surfaceContainerHighest,
          color: color,
          minHeight: 8,
          borderRadius: BorderRadius.circular(4),
        ),
      ],
    );
  }
}

String _formatBytes(int bytes) {
  if (bytes < 1024) return '$bytes B';
  if (bytes < 1024 * 1024) return '${(bytes / 1024).toStringAsFixed(1)} KB';
  if (bytes < 1024 * 1024 * 1024) return '${(bytes / (1024 * 1024)).toStringAsFixed(1)} MB';
  return '${(bytes / (1024 * 1024 * 1024)).toStringAsFixed(1)} GB';
}

class _ProgressBar extends StatelessWidget {
  final String label;
  final double used;
  final double total;
  final String usedLabel;
  final String totalLabel;
  final Color color;

  const _ProgressBar({
    required this.label,
    required this.used,
    required this.total,
    required this.usedLabel,
    required this.totalLabel,
    required this.color,
  });

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final ratio = total > 0 ? (used / total).clamp(0.0, 1.0) : 0.0;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(label, style: const TextStyle(fontWeight: FontWeight.w500)),
            Text('$usedLabel / $totalLabel', style: TextStyle(color: colors.onSurfaceVariant, fontSize: 12)),
          ],
        ),
        const SizedBox(height: 8),
        LinearProgressIndicator(
          value: ratio,
          backgroundColor: colors.surfaceContainerHighest,
          color: color,
          minHeight: 8,
          borderRadius: BorderRadius.circular(4),
        ),
      ],
    );
  }
}

class _SummaryCard extends StatelessWidget {
  final String title;
  final String value;
  final IconData icon;
  final Color color;

  const _SummaryCard({
    required this.title,
    required this.value,
    required this.icon,
    required this.color,
  });

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Semantics(
      label: '$title: $value',
      child: GlassCard(
        child: Padding(
          padding: const EdgeInsets.all(20),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Icon(icon, color: color, size: 24),
              const SizedBox(height: 12),
              Text(
                title,
                style: TextStyle(
                  fontSize: 12,
                  color: colors.onSurfaceVariant,
                  fontWeight: FontWeight.w500,
                ),
              ),
              const SizedBox(height: 4),
              Text(
                value,
                style: const TextStyle(
                  fontSize: 24,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
