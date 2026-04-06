import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:intl/intl.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/models/agent.dart';

final _dashboardFutureProvider = FutureProvider.autoDispose<DashboardSnapshot>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) throw Exception('API not available');
  return api.getDashboard();
});

class CostDashboardScreen extends ConsumerWidget {
  const CostDashboardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final currencyFormat = NumberFormat.currency(symbol: '\$');
    final colors = Theme.of(context).colorScheme;
    final dashboardAsyncValue = ref.watch(_dashboardFutureProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Cost & Telemetry', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        actions: [
          IconButton(
            onPressed: () {
              ref.invalidate(_dashboardFutureProvider);
            },
            icon: dashboardAsyncValue.isLoading
                 ? const SizedBox(width: 16, height: 16, child: CircularProgressIndicator(strokeWidth: 2))
                 : const Icon(Icons.refresh),
            tooltip: 'Refresh costs',
          ),
        ],
      ),
      body: dashboardAsyncValue.when(
        loading: () => Center(
          child: CircularProgressIndicator(
            color: Theme.of(context).colorScheme.primary,
          ),
        ),
        error: (error, stack) => Center(child: Text('Error: $error', style: const TextStyle(fontFamily: 'Inter'))),
        data: (data) {
          final costs = data.costs;

          return ListView(
            padding: const EdgeInsets.all(24),
            children: [
              // Summary Cards
              Row(
                children: [
                  Expanded(
                    child: _AnimatedSummaryCard(
                      title: 'Total Spend',
                      value: currencyFormat.format(costs.totalCostUSD),
                      icon: Icons.account_balance_wallet,
                      color: colors.primary,
                    ),
                  ),
                  const SizedBox(width: 16),
                  Expanded(
                    child: _AnimatedSummaryCard(
                      title: 'Total Tokens',
                      value: NumberFormat.compact().format(costs.totalTokens),
                      icon: Icons.generating_tokens,
                      color: colors.secondary,
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 32),

              // Usage per Agent Chart
              Text(
                'Usage per Agent',
                style: Theme.of(
                  context,
                ).textTheme.titleLarge?.copyWith(fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
              ),
              const SizedBox(height: 16),
              _GlassPanel(
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
                                          fontWeight: FontWeight.bold,
                                          fontFamily: 'Outfit',
                                        ),
                                      ),
                                      Text(
                                        currencyFormat.format(
                                          agentCost.costUSD,
                                        ),
                                        style: const TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.w500),
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
                                          color: colors.surfaceContainerHighest.withValues(alpha: 0.3),
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
                                      fontSize: 12,
                                      color: colors.onSurfaceVariant,
                                      fontFamily: 'Inter',
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
                ).textTheme.titleLarge?.copyWith(fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
              ),
              const SizedBox(height: 16),
              _GlassPanel(
                child: Padding(
                  padding: const EdgeInsets.all(20),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        children: [
                          Icon(Icons.business, size: 24, color: colors.primary),
                          const SizedBox(width: 12),
                          Text(
                            data.organization.name,
                            style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 18, fontFamily: 'Outfit'),
                          ),
                          const Spacer(),
                          Container(
                            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                            decoration: BoxDecoration(
                               color: colors.primary.withValues(alpha: 0.1),
                               borderRadius: BorderRadius.circular(16),
                            ),
                            child: Text(data.organization.domain, style: TextStyle(color: colors.primary, fontFamily: 'Inter', fontSize: 12, fontWeight: FontWeight.w600)),
                          ),
                        ],
                      ),
                      const Divider(height: 32),
                      ...data.organization.members
                          .take(3)
                          .map(
                            (m) => Padding(
                              padding: const EdgeInsets.only(bottom: 12),
                              child: Row(
                                children: [
                                  Container(
                                     padding: const EdgeInsets.all(10),
                                     decoration: BoxDecoration(
                                        color: colors.surfaceContainerHighest.withValues(alpha: 0.5),
                                        shape: BoxShape.circle,
                                     ),
                                     child: Icon(
                                        m.isHuman ? Icons.person : Icons.smart_toy,
                                        size: 20,
                                        color: colors.onSurface,
                                     ),
                                  ),
                                  const SizedBox(width: 16),
                                  Expanded(
                                     child: Column(
                                        crossAxisAlignment: CrossAxisAlignment.start,
                                        children: [
                                           Text(m.name, style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit', fontSize: 16)),
                                           const SizedBox(height: 2),
                                           Text(m.role, style: TextStyle(color: colors.onSurfaceVariant, fontFamily: 'Inter', fontSize: 14)),
                                        ],
                                     ),
                                  ),
                                ],
                              ),
                            ),
                          ),
                      if (data.organization.members.length > 3)
                        Center(
                          child: TextButton(
                            onPressed: () {},
                            child: const Text('View Full Org Tree', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.w600)),
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

class _AnimatedSummaryCard extends StatefulWidget {
  final String title;
  final String value;
  final IconData icon;
  final Color color;

  const _AnimatedSummaryCard({
    required this.title,
    required this.value,
    required this.icon,
    required this.color,
  });

  @override
  State<_AnimatedSummaryCard> createState() => _AnimatedSummaryCardState();
}

class _AnimatedSummaryCardState extends State<_AnimatedSummaryCard> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<Offset> _slideAnimation;
  late Animation<double> _fadeAnimation;
  bool _isHovered = false;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 600),
    );
    _slideAnimation = Tween<Offset>(
      begin: const Offset(0, 0.2),
      end: Offset.zero,
    ).animate(CurvedAnimation(parent: _controller, curve: Curves.easeOutQuart));
    _fadeAnimation = Tween<double>(begin: 0.0, end: 1.0)
        .animate(CurvedAnimation(parent: _controller, curve: Curves.easeOut));

    Future.delayed(const Duration(milliseconds: 100), () {
      if (mounted) {
        _controller.forward();
      }
    });
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Semantics(
      label: '${widget.title}: ${widget.value}',
      child: SlideTransition(
        position: _slideAnimation,
        child: FadeTransition(
          opacity: _fadeAnimation,
          child: MouseRegion(
            onEnter: (_) => setState(() => _isHovered = true),
            onExit: (_) => setState(() => _isHovered = false),
            child: AnimatedScale(
              scale: _isHovered ? 1.02 : 1.0,
              duration: const Duration(milliseconds: 200),
              curve: Curves.easeOutCubic,
              child: ClipRRect(
                borderRadius: BorderRadius.circular(16),
                child: BackdropFilter(
                  filter: ImageFilter.compose(
                    outer: ColorFilter.matrix(const <double>[
                      1.168, -0.153, -0.015, 0, 0,
                      -0.046, 1.061, -0.015, 0, 0,
                      -0.046, -0.152, 1.198, 0, 0,
                      0, 0, 0, 1, 0,
                    ]),
                    inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                  ),
                  child: AnimatedContainer(
                    duration: const Duration(milliseconds: 300),
                    decoration: BoxDecoration(
                      color: _isHovered
                          ? const Color.fromRGBO(255, 255, 255, 0.08)
                          : const Color.fromRGBO(255, 255, 255, 0.03),
                      borderRadius: BorderRadius.circular(16),
                      border: Border.all(
                        color: _isHovered
                            ? Colors.white.withValues(alpha: 0.3)
                            : Colors.white.withValues(alpha: 0.1),
                      ),
                    ),
                    child: Padding(
                      padding: const EdgeInsets.all(24),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Icon(widget.icon, color: widget.color, size: 32),
                          const SizedBox(height: 16),
                          Text(
                            widget.title,
                            style: TextStyle(
                              fontSize: 14,
                              color: colors.onSurfaceVariant,
                              fontWeight: FontWeight.w500,
                              fontFamily: 'Inter',
                            ),
                          ),
                          const SizedBox(height: 6),
                          Text(
                            widget.value,
                            style: const TextStyle(
                              fontSize: 32,
                              fontWeight: FontWeight.bold,
                              fontFamily: 'Inter',
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _GlassPanel extends StatelessWidget {
  final Widget child;

  const _GlassPanel({required this.child});

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.compose(
          outer: ColorFilter.matrix(const <double>[
            1.168, -0.153, -0.015, 0, 0,
            -0.046, 1.061, -0.015, 0, 0,
            -0.046, -0.152, 1.198, 0, 0,
            0, 0, 0, 1, 0,
          ]),
          inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        ),
        child: Container(
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            borderRadius: BorderRadius.circular(16),
            border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
          ),
          child: child,
        ),
      ),
    );
  }
}
