import 'package:ohc_app/widgets/growth_referral_widget.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import 'package:ohc_app/widgets/glass_card.dart';
import 'dart:ui';
import 'package:flutter_riverpod/flutter_riverpod.dart';
// import 'package:flutter_svg/flutter_svg.dart'; // Temporarily disabled for Bazel build
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/widgets/swarm_observability_widget.dart';
import 'package:ohc_app/widgets/swarm_velocity_widget.dart';
import 'package:ohc_app/widgets/hybrid_observability_widget.dart';
import 'package:ohc_app/widgets/hybrid_telemetry_widget.dart';
import 'package:ohc_app/widgets/sub_agent_queue_widget.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:ohc_app/widgets/business_share_widget.dart';

final dashboardProvider = FutureProvider.autoDispose<DashboardSnapshot>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) throw Exception('API not available');
  return api.getDashboard();
});

class DashboardShimmer extends StatefulWidget {
  const DashboardShimmer({super.key});

  @override
  State<DashboardShimmer> createState() => _DashboardShimmerState();
}

class _DashboardShimmerState extends State<DashboardShimmer> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _opacity;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1500),
    )..repeat(reverse: true);
    _opacity = Tween<double>(begin: 0.3, end: 0.7).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeInOut),
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: _opacity,
      builder: (context, child) {
        return Opacity(
          opacity: _opacity.value,
          child: child,
        );
      },
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(24.0),
        child: Semantics(
          label: 'Dashboard Loading Skeleton',
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              _buildSkeletonRect(width: 150, height: 30),
              const SizedBox(height: 24),
              SizedBox(
                height: 120,
                child: ListView.separated(
                  scrollDirection: Axis.horizontal,
                  itemCount: 3,
                  separatorBuilder: (context, index) => const SizedBox(width: 16),
                  itemBuilder: (context, index) => _buildSkeletonCard(),
                ),
              ),
              const SizedBox(height: 32),
              _buildSkeletonRect(width: 200, height: 30),
              const SizedBox(height: 24),
              Wrap(
                spacing: 16,
                runSpacing: 16,
                children: List.generate(4, (index) => _buildSkeletonCard()),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildSkeletonRect({required double width, required double height}) {
    return Container(
      width: width,
      height: height,
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surfaceContainerHighest.withValues(alpha: 0.5),
        borderRadius: BorderRadius.circular(8),
      ),
    );
  }

  Widget _buildSkeletonCard() {
    return Container(
      width: 200,
      height: 120,
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surfaceContainerHighest.withValues(alpha: 0.5),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: Theme.of(context).colorScheme.outlineVariant.withValues(alpha: 0.2),
        ),
      ),
    );
  }
}

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
        loading: () => const DashboardShimmer(),
        error:
            (e, _) => Center(
              child: SelectableText(
                'Error: $e',
                style: TextStyle(color: Theme.of(context).colorScheme.error, fontFamily: 'Inter'),
              ),
            ),
        data: (data) => _DashboardContent(data: data, ref: ref),
      ),
    );
  }
}

class _DashboardContent extends StatelessWidget {
  final DashboardSnapshot data;
  final WidgetRef ref;

  const _DashboardContent({required this.data, required this.ref});

  @override
  Widget build(BuildContext context) {
    // Collect all unique roles
    final Set<String> allRoles = {};
    for (final profile in data.organization.roleProfiles) {
      if (profile.role.isNotEmpty && profile.role != 'CEO') {
        allRoles.add(profile.role);
      }
    }
    for (final member in data.organization.members) {
      if (member.role.isNotEmpty && !member.isHuman) {
        allRoles.add(member.role);
      }
    }
    for (final agent in data.agents) {
      if (agent.role.isNotEmpty) {
        allRoles.add(agent.role);
      }
    }

    final roleList = allRoles.toList()..sort();

    return ListView(
      padding: const EdgeInsets.all(24),
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            _SectionTitle('My Business'),
            OutlinedButton.icon(
              onPressed: () => context.go('/wizards/billing'),
              icon: const Icon(Icons.credit_card),
              label: const Text('Billing & Credits'),
            ),
          ],
        ),
        const SizedBox(height: 16),
        Wrap(
          spacing: 16,
          runSpacing: 16,
          children: [
            _StatCard(
              label: 'Active Agents',
              value: data.agents.where((a) => a.isRunning).length.toString(),
              icon: Icons.smart_toy,
              color: Theme.of(context).colorScheme.primary,
            ),
            _StatCard(
              label: 'Active Tasks',
              value: data.statuses.length.toString(),
              icon: Icons.pending_actions,
              color: Theme.of(context).colorScheme.secondary,
            ),
            _StatCard(
              label: 'Scheduled Calls',
              value: data.meetings.length.toString(),
              icon: Icons.video_call,
              color: Theme.of(context).colorScheme.tertiary,
            ),
            _StatCard(
              label: 'Team Members',
              value: data.organization.members.length.toString(),
              icon: Icons.people,
              color: Theme.of(context).colorScheme.primaryContainer,
              iconColor: Theme.of(context).colorScheme.onPrimaryContainer,
            ),
          ],
        ),
        // --- UPGRADE BANNER ---
        Container(
          margin: const EdgeInsets.only(bottom: 24),
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.primary.withValues(alpha: 0.1),
            borderRadius: BorderRadius.circular(16),
            border: Border.all(color: Theme.of(context).colorScheme.primary.withValues(alpha: 0.3)),
          ),
          child: Material(
            color: Colors.transparent,
            child: InkWell(
              borderRadius: BorderRadius.circular(16),
              onTap: () => context.go('/wizards/upgrade'),
              child: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 16),
                child: Row(
                  children: [
                    Icon(Icons.auto_awesome, color: Theme.of(context).colorScheme.primary),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text("What's new ✨", style: TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit', color: Theme.of(context).colorScheme.onSurface)),
                          const Text("OHC v2.4 is available. Upgrade now for 2x faster orchestration.", style: TextStyle(fontFamily: 'Inter', fontSize: 13)),
                        ],
                      ),
                    ),
                    FilledButton(
                      onPressed: () => context.go('/wizards/upgrade'),
                      child: const Text('Upgrade in 1 click'),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
        const SizedBox(height: 32),
        const SizedBox(height: 32),
        _SectionTitle('System Health'),
        const SizedBox(height: 16),
        _ObservabilityWidget(data: data),
        const SizedBox(height: 16),
        const SwarmObservabilityWidget(),
        const SizedBox(height: 16),
        const SwarmVelocityWidget(),
        const SizedBox(height: 16),
        const HybridObservabilityWidget(),
        const SizedBox(height: 16),
        Container(
          key: const ValueKey('hybrid_telemetry'),
          child: const HybridTelemetryWidget(),
        ),
        const SizedBox(height: 16),
        SizedBox(
          height: 350,
          child: GlassCard(
            padding: EdgeInsets.zero,
            child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Padding(
                      padding: const EdgeInsets.fromLTRB(24, 24, 24, 0),
                      child: Text(
                        'Proactive Task Stream',
                        style: TextStyle(
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                          color: Theme.of(context).colorScheme.onSurface,
                          fontFamily: 'Outfit',
                        ),
                      ),
                    ),
                    const Expanded(child: TaskListView()),
                  ],
                ),
          ),
        ),
        const SizedBox(height: 16),
        const GrowthReferralWidget(),
        const SizedBox(height: 16),
        BusinessShareWidget(data: data),
        const SizedBox(height: 16),
        SubAgentQueueWidget(statuses: data.statuses),
        const SizedBox(height: 32),
        _SectionTitle('Company Structure'),
        const SizedBox(height: 8),
        Text(
          'Manage your AI workforce. Scale roles up or down to match current organizational demands.',
          style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                color: Theme.of(context).colorScheme.onSurfaceVariant,
                fontFamily: 'Inter',
              ),
        ),
        const SizedBox(height: 16),
        Wrap(
          spacing: 16,
          runSpacing: 16,
          children: roleList.map((role) {
            final count = data.agents.where((a) => a.role == role).length;
            return _RoleScaleCard(
              role: role,
              count: count,
              ref: ref,
            );
          }).toList(),
        ),
      ],
    );
  }
}

class _ObservabilityWidget extends StatelessWidget {
  final DashboardSnapshot data;

  const _ObservabilityWidget({required this.data});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    final activeMissions = data.statuses.length; // Approximate from statuses
    final totalAgents = data.agents.length;
    final healthScore = totalAgents > 0 ? (data.agents.where((a) => a.isRunning).length / totalAgents * 100).round() : 100;

    return Semantics(
      label: 'System Observability Panel',
      child: Tooltip(
        message: 'View System Health & Metrics',
        child: GlassCard(
            padding: EdgeInsets.zero,
            child: Material(
                color: Colors.transparent,
                child: InkWell(
                  onTap: () {
                    // Tap interaction for delight
                  },
                  borderRadius: BorderRadius.circular(24),
                  splashColor: colors.primary.withValues(alpha: 0.1),
                  highlightColor: colors.primary.withValues(alpha: 0.05),
                  child: Padding(
                    padding: const EdgeInsets.all(32),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Wrap(
                          crossAxisAlignment: WrapCrossAlignment.center,
                          children: [
                            Container(
                              padding: const EdgeInsets.all(10),
                              decoration: BoxDecoration(
                                color: colors.primary.withValues(alpha: 0.1),
                                borderRadius: BorderRadius.circular(12),
                              ),
                              child: Icon(Icons.monitor_heart, color: colors.primary, size: 28),
                            ),
                            const SizedBox(width: 16),
                            Text(
                              'AI System Status',
                              style: TextStyle(
                                fontSize: 24,
                                fontWeight: FontWeight.bold,
                                color: colors.onSurface,
                                fontFamily: 'Outfit',
                              ),
                            ),
                            const SizedBox(width: 16),
                            _StatusBadge(healthy: healthScore >= 80),
                          ],
                        ),
                        const SizedBox(height: 32),
                        Wrap(
                          spacing: 16.0,
                          runSpacing: 16.0,
                          alignment: WrapAlignment.spaceAround,
                          children: [
                            _Metric(label: 'Health Score', value: '$healthScore%', color: colors.primary, icon: Icons.health_and_safety),
                            _Metric(label: 'Current Tasks', value: '$activeMissions', color: colors.secondary, icon: Icons.rocket_launch),
                            _Metric(label: 'System Speed', value: 'Fast', color: colors.tertiary, icon: Icons.speed),
                            _Metric(label: 'Running Agents', value: '$totalAgents', color: colors.primaryContainer, icon: Icons.smart_toy, iconColor: colors.onPrimaryContainer),
                          ],
                        ),
                      ],
                    ),
                  ),
                ),
              ),
          ),
      ),
    );
  }
}

class _StatusBadge extends StatelessWidget {
  final bool healthy;
  const _StatusBadge({required this.healthy});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final color = healthy ? Colors.green : colors.error;
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      decoration: BoxDecoration(
        color: color.withValues(alpha: 0.2),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: color.withValues(alpha: 0.5)),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            width: 10,
            height: 10,
            decoration: BoxDecoration(
              color: color,
              shape: BoxShape.circle,
            ),
          ),
          const SizedBox(width: 8),
          Text(
            healthy ? 'System Nominal' : 'Degraded',
            style: TextStyle(
              fontSize: 14,
              fontWeight: FontWeight.bold,
              color: color,
              fontFamily: 'Inter',
            ),
          ),
        ],
      ),
    );
  }
}

class _Metric extends StatelessWidget {
  final String label;
  final String value;
  final Color color;
  final IconData icon;
  final Color? iconColor;

  const _Metric({required this.label, required this.value, required this.color, required this.icon, this.iconColor});

  @override
  Widget build(BuildContext context) {
    final effectiveIconColor = iconColor ?? color;
    return Column(
      children: [
        Icon(icon, color: effectiveIconColor, size: 24),
        const SizedBox(height: 8),
        Text(
          value,
          style: TextStyle(
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: color,
            fontFamily: 'Inter',
          ),
        ),
        const SizedBox(height: 4),
        Text(
          label,
          style: TextStyle(
            fontSize: 14,
            color: Theme.of(context).colorScheme.onSurfaceVariant,
            fontFamily: 'Inter',
            fontWeight: FontWeight.w500,
          ),
        ),
      ],
    );
  }
}

class _RoleScaleCard extends StatefulWidget {
  final String role;
  final int count;
  final WidgetRef ref;

  const _RoleScaleCard({
    required this.role,
    required this.count,
    required this.ref,
  });

  @override
  State<_RoleScaleCard> createState() => _RoleScaleCardState();
}

class _RoleScaleCardState extends State<_RoleScaleCard> {
  bool _isScaling = false;
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

    // Optional delay for staggered entrance animation if needed
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
    final colorScheme = Theme.of(context).colorScheme;

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
