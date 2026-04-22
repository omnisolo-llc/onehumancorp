import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/widgets/swarm_observability_widget.dart';
import 'package:ohc_app/widgets/hybrid_observability_widget.dart';
import 'package:ohc_app/widgets/hybrid_telemetry_widget.dart';
import 'package:ohc_app/widgets/sub_agent_queue_widget.dart';
import 'package:ohc_app/widgets/growth_referral_widget.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:ohc_app/widgets/glass_card.dart';

final dashboardProvider = FutureProvider.autoDispose<DashboardSnapshot>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) throw Exception('API not available');
  return api.getDashboard();
});

class DashboardScreen extends ConsumerWidget {
  const DashboardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final snapshot = ref.watch(dashboardProvider);
    return Scaffold(
      appBar: AppBar(
        title: const Text('Dashboard', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        leading: const Padding(
          padding: EdgeInsets.all(10.0),
          child: Icon(Icons.person),
        ),
      ),
      body: snapshot.when(
        loading:
            () => Center(
              child: CircularProgressIndicator(
                color: Theme.of(context).colorScheme.primary,
              ),
            ),
        error:
            (e, _) => Center(
              child: Text(
                'Error: $e',
                style: TextStyle(color: Theme.of(context).colorScheme.error, fontFamily: 'Inter'),
              ),
            ),
        data: (data) {
          final isStandalone =
              data.hybridHealth != null &&
              data.hybridHealth!.mode == 'standalone';

          return SingleChildScrollView(
            padding: const EdgeInsets.all(32.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Header section
                Row(
                  crossAxisAlignment: CrossAxisAlignment.end,
                  children: [
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          const Text(
                            'Business Overview',
                            style: TextStyle(
                              fontSize: 48,
                              fontWeight: FontWeight.bold,
                              fontFamily: 'Outfit',
                            ),
                          ),
                          const SizedBox(height: 8),
                          Text(
                            'Welcome back to ${data.organization.name}',
                            style: Theme.of(context).textTheme.titleLarge?.copyWith(
                              color: Theme.of(context).colorScheme.onSurfaceVariant,
                              fontFamily: 'Inter',
                            ),
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 48),
                const _SectionTitle('Hybrid Observability'),
                const SizedBox(height: 24),
                const Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Expanded(
                      flex: 2,
                      child: HybridTelemetryWidget(),
                    ),
                    SizedBox(width: 24),
                    Expanded(
                      flex: 3,
                      child: SwarmObservabilityWidget(),
                    ),
                  ],
                ),
                const SizedBox(height: 48),

                const _SectionTitle('Key Metrics'),
                const SizedBox(height: 24),
                Wrap(
                  spacing: 24,
                  runSpacing: 24,
                  children: [
                    _StatCard(
                      label: 'Total Cost',
                      value: '\$${data.costs.totalCostUSD.toStringAsFixed(2)}',
                      icon: Icons.attach_money,
                      color: Theme.of(context).colorScheme.primaryContainer,
                      iconColor: Theme.of(context).colorScheme.primary,
                    ),
                    _StatCard(
                      label: 'Tokens Used',
                      value: '${data.costs.totalTokens}',
                      icon: Icons.data_usage,
                      color: Theme.of(context).colorScheme.secondaryContainer,
                      iconColor: Theme.of(context).colorScheme.secondary,
                    ),
                    const GrowthReferralWidget(),
                  ],
                ),
                const SizedBox(height: 48),

                const _SectionTitle('Agent Orchestration'),
                const SizedBox(height: 24),
                Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Expanded(
                      flex: 3,
                      child: ClipRRect(
                        borderRadius: BorderRadius.circular(16),
                        child: const SizedBox(
                          height: 500, // Fixed height for task list in dashboard
                          child: TaskListScreen(),
                        ),
                      ),
                    ),
                    const SizedBox(width: 24),
                    Expanded(
                      flex: 2,
                      child: Column(
                        children: [
                          if (isStandalone) ...[
                            const HybridObservabilityWidget(),
                            const SizedBox(height: 24),
                          ],
                          SubAgentQueueWidget(statuses: data.statuses),
                        ],
                      ),
                    ),
                  ],
                ),

                const SizedBox(height: 48),
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const _SectionTitle('Active Agents'),
                    TextButton.icon(
                      onPressed: () {
                        // TODO: Implement navigation to detailed agents view
                      },
                      icon: const Icon(Icons.arrow_forward),
                      label: const Text('View All', style: TextStyle(fontFamily: 'Inter')),
                    ),
                  ],
                ),
                const SizedBox(height: 24),
                Wrap(
                  spacing: 24,
                  runSpacing: 24,
                  children:
                      data.agents.map((agent) {
                        return _AgentCard(
                          ref: ref,
                          role: agent.role,
                          count: 1, // Will update when count added to model
                        );
                      }).toList(),
                ),
              ],
            ),
          );
        },
      ),
    );
  }
}

class _AgentCard extends StatefulWidget {
  final WidgetRef ref;
  final String role;
  final int count;

  const _AgentCard({
    required this.ref,
    required this.role,
    required this.count,
  });

  @override
  State<_AgentCard> createState() => _AgentCardState();
}

class _AgentCardState extends State<_AgentCard> {
  bool _isScaling = false;
  // ignore: unused_field
  bool _isHovered = false;

  Future<void> _scaleTo(int newCount) async {
    if (_isScaling || newCount < 0) return;
    setState(() => _isScaling = true);
    try {
      final api = widget.ref.read(apiServiceProvider);
      if (api != null) {
        await api.scaleAgents(widget.role, newCount);
        widget.ref.invalidate(dashboardProvider);
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Failed to scale ${widget.role}: $e', style: const TextStyle(fontFamily: 'Inter')),
            backgroundColor: Theme.of(context).colorScheme.error,
          ),
        );
      }
    } finally {
      if (mounted) {
        setState(() => _isScaling = false);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final formattedRole = widget.role.replaceAll('_', ' ').split(' ').map((word) {
      if (word.isEmpty) return '';
      if (word.toUpperCase() == 'AI') return 'AI';
      return word[0].toUpperCase() + word.substring(1).toLowerCase();
    }).join(' ');

    return Semantics(
      label: 'Scale $formattedRole role',
      child: Tooltip(
        message: 'Manage $formattedRole Allocation',
        child: SizedBox(
          width: 320,
          child: GlassCard(
        child: Padding(
                      padding: const EdgeInsets.symmetric(vertical: 20, horizontal: 24),
                      child: Row(
                        children: [
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text(
                                  formattedRole,
                                  style: const TextStyle(
                                    fontWeight: FontWeight.bold,
                                    fontSize: 18,
                                    fontFamily: 'Outfit',
                                  ),
                                  maxLines: 1,
                                  overflow: TextOverflow.ellipsis,
                                ),
                                const SizedBox(height: 6),
                                Text(
                                  '${widget.count} Agent${widget.count == 1 ? '' : 's'}',
                                  style: TextStyle(
                                    fontSize: 15,
                                    color: colors.onSurfaceVariant,
                                    fontFamily: 'Inter',
                                  ),
                                ),
                              ],
                            ),
                          ),
                          const SizedBox(width: 16),
                          Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Semantics(
                            button: true,
                            label: 'Decrease $formattedRole count',
                            child: IconButton(
                              icon: const Icon(Icons.remove_circle_outline, size: 28),
                              color: colors.primary,
                              onPressed: widget.count > 0 && !_isScaling
                                  ? () => _scaleTo(widget.count - 1)
                                  : null,
                              tooltip: 'Fire Agent',
                            ),
                          ),
                          SizedBox(
                            width: 32,
                            child: Center(
                              child: _isScaling
                                  ? SizedBox(
                                      width: 20,
                                      height: 20,
                                      child: CircularProgressIndicator(
                                        strokeWidth: 2,
                                        color: colors.primary,
                                      ),
                                    )
                                  : Text(
                                      '${widget.count}',
                                      style: const TextStyle(
                                        fontWeight: FontWeight.bold,
                                        fontSize: 20,
                                        fontFamily: 'Inter',
                                      ),
                                    ),
                          ),
                          ),
                          Semantics(
                            button: true,
                            label: 'Increase $formattedRole count',
                            child: IconButton(
                              icon: const Icon(Icons.add_circle_outline, size: 28),
                              color: colors.primary,
                              onPressed: !_isScaling
                                  ? () => _scaleTo(widget.count + 1)
                                  : null,
                              tooltip: 'Hire Agent',
                            ),
                          ),
                            ],
                          ),
                        ],
                      ),
                    ),
      ),
        ),
      ),
    );
  }
}

class _SectionTitle extends StatelessWidget {
  final String text;
  const _SectionTitle(this.text);

  @override
  Widget build(BuildContext context) {
    return Text(
      text,
      style: Theme.of(
        context,
      ).textTheme.headlineSmall?.copyWith(fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
    );
  }
}

class _StatCard extends StatefulWidget {
  final String label;
  final String value;
  final IconData icon;
  final Color color;
  final Color? iconColor;

  const _StatCard({
    required this.label,
    required this.value,
    required this.icon,
    required this.color,
    this.iconColor,
  });

  @override
  State<_StatCard> createState() => _StatCardState();
}

class _StatCardState extends State<_StatCard> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<Offset> _slideAnimation;
  late Animation<double> _fadeAnimation;
  // ignore: unused_field
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
    final effectiveIconColor = widget.iconColor ?? widget.color;

    return Semantics(
      label: '${widget.label}: ${widget.value}',
      button: true,
      excludeSemantics: true,
      child: Tooltip(
        message: 'View ${widget.label}',
        child: SizedBox(
          width: 200,
          child: SlideTransition(
            position: _slideAnimation,
            child: FadeTransition(
              opacity: _fadeAnimation,
              child: GlassCard(
        child: Material(
                          color: Colors.transparent,
                          child: Semantics(
                            button: true,
                            label: '${widget.label}: ${widget.value} action',
                            child: InkWell(
                              onTap: () {},
                              borderRadius: BorderRadius.circular(16),
                              splashColor: widget.color.withValues(alpha: 0.1),
                              highlightColor: widget.color.withValues(alpha: 0.05),
                              child: Padding(
                                padding: const EdgeInsets.all(24),
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    Icon(widget.icon, color: effectiveIconColor, size: 32),
                                    const SizedBox(height: 16),
                                    Text(
                                      widget.value,
                                      style: TextStyle(
                                        fontSize: 36,
                                        fontWeight: FontWeight.bold,
                                        color: effectiveIconColor,
                                        fontFamily: 'Inter',
                                      ),
                                    ),
                                    const SizedBox(height: 6),
                                    Text(
                                      widget.label,
                                      style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                                        fontFamily: 'Inter',
                                        fontWeight: FontWeight.w500,
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
      ),
    );
  }
}
