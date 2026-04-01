import 'package:flutter/material.dart';
import 'dart:ui';
import 'package:flutter_riverpod/flutter_riverpod.dart';
// import 'package:flutter_svg/flutter_svg.dart'; // Temporarily disabled for Bazel build
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/services/api_service.dart';

final dashboardProvider = FutureProvider.autoDispose<DashboardSnapshot>((
  ref,
) async {
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
        title: const Text('Dashboard'),
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
                style: TextStyle(color: Theme.of(context).colorScheme.error),
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

  const _DashboardContent({super.key, required this.data, required this.ref});

  @override
  Widget build(BuildContext context) {
    // Collect all unique roles
    final Set<String> allRoles = {};
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
        _SectionTitle('Overview'),
        const SizedBox(height: 16),
        Wrap(
          spacing: 16,
          runSpacing: 16,
          children: [
            _ObservabilityWidget(dashboard: data),
            _StatCard(
              label: 'Active Agents',
              value: data.agents.where((a) => a.isRunning).length.toString(),
              icon: Icons.smart_toy,
              color: Theme.of(context).colorScheme.primary,
            ),
            _StatCard(
              label: 'Dashboard Updates',
              value: data.statuses.length.toString(),
              icon: Icons.pending_actions,
              color: Theme.of(context).colorScheme.secondary,
            ),
            _StatCard(
              label: 'Open Meetings',
              value: data.meetings.length.toString(),
              icon: Icons.video_call,
              color: Theme.of(context).colorScheme.tertiary,
            ),
            _StatCard(
              label: 'Total Org Members',
              value: data.organization.members.length.toString(),
              icon: Icons.people,
              color: Theme.of(context).colorScheme.primaryContainer,
              iconColor: Theme.of(context).colorScheme.onPrimaryContainer,
            ),
          ],
        ),
        const SizedBox(height: 32),
        _SectionTitle('Company Structure'),
        const SizedBox(height: 8),
        Text(
          'Manage your AI workforce. Scale roles up or down to match current organizational demands.',
          style: Theme.of(context).textTheme.bodyMedium?.copyWith(
            color: Theme.of(context).colorScheme.onSurfaceVariant,
          ),
        ),
        const SizedBox(height: 16),
        Wrap(
          spacing: 16,
          runSpacing: 16,
          children:
              roleList.map((role) {
                final count = data.agents.where((a) => a.role == role).length;
                return _RoleScaleCard(role: role, count: count, ref: ref);
              }).toList(),
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
            content: Text('Failed to scale ${widget.role}: $e'),
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
    final formattedRole = widget.role
        .replaceAll('_', ' ')
        .split(' ')
        .map((word) {
          if (word.isEmpty) return '';
          return word[0].toUpperCase() + word.substring(1).toLowerCase();
        })
        .join(' ');

    return Semantics(
      label: 'Scale $formattedRole role',
      child: SizedBox(
        width: 300,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(12),
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: ColorFilter.matrix(<double>[
                1.213,
                -0.213,
                -0.072,
                0,
                0,
                -0.213,
                1.213,
                -0.072,
                0,
                0,
                -0.213,
                -0.213,
                1.213,
                0,
                0,
                0,
                0,
                0,
                1,
                0,
              ]),
              inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            ),
            child: Container(
              decoration: BoxDecoration(
                color: colors.surfaceContainerHighest.withOpacity(0.4),
                borderRadius: BorderRadius.circular(12),
                border: Border.all(
                  color: colors.outlineVariant.withOpacity(0.5),
                ),
              ),
              child: Padding(
                padding: const EdgeInsets.symmetric(
                  vertical: 16,
                  horizontal: 20,
                ),
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
                              fontSize: 16,
                            ),
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                          ),
                          const SizedBox(height: 4),
                          Text(
                            '${widget.count} Agent${widget.count == 1 ? '' : 's'}',
                            style: TextStyle(
                              fontSize: 14,
                              color: colors.onSurfaceVariant,
                            ),
                          ),
                        ],
                      ),
                    ),
                    const SizedBox(width: 12),
                    Row(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        Semantics(
                          button: true,
                          label: 'Decrease $formattedRole count',
                          child: IconButton(
                            icon: const Icon(Icons.remove_circle_outline),
                            color: colors.primary,
                            onPressed:
                                widget.count > 0 && !_isScaling
                                    ? () => _scaleTo(widget.count - 1)
                                    : null,
                            tooltip: 'Fire Agent',
                          ),
                        ),
                        SizedBox(
                          width: 24,
                          child: Center(
                            child:
                                _isScaling
                                    ? SizedBox(
                                      width: 16,
                                      height: 16,
                                      child: CircularProgressIndicator(
                                        strokeWidth: 2,
                                        color: colors.primary,
                                      ),
                                    )
                                    : Text(
                                      '${widget.count}',
                                      style: const TextStyle(
                                        fontWeight: FontWeight.bold,
                                        fontSize: 16,
                                      ),
                                    ),
                          ),
                        ),
                        Semantics(
                          button: true,
                          label: 'Increase $formattedRole count',
                          child: IconButton(
                            icon: const Icon(Icons.add_circle_outline),
                            color: colors.primary,
                            onPressed:
                                !_isScaling
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
      ).textTheme.headlineSmall?.copyWith(fontWeight: FontWeight.bold),
    );
  }
}

class _StatCard extends StatelessWidget {
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
  Widget build(BuildContext context) {
    final effectiveIconColor = iconColor ?? color;
    return Semantics(
      label: '$label: $value',
      button: true,
      excludeSemantics: true,
      child: SizedBox(
        width: 180,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(12),
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: ColorFilter.matrix(<double>[
                1.213,
                -0.213,
                -0.072,
                0,
                0,
                -0.213,
                1.213,
                -0.072,
                0,
                0,
                -0.213,
                -0.213,
                1.213,
                0,
                0,
                0,
                0,
                0,
                1,
                0,
              ]),
              inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            ),
            child: Container(
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.surface.withOpacity(0.03),
                borderRadius: BorderRadius.circular(12),
                border: Border.all(
                  color: Theme.of(
                    context,
                  ).colorScheme.onSurface.withOpacity(0.08),
                ),
              ),
              child: Material(
                color: Colors.transparent,
                child: Semantics(
                  button: true,
                  label: '$label: $value action',
                  child: InkWell(
                    onTap: () {},
                    borderRadius: BorderRadius.circular(12),
                    splashColor: color.withOpacity(0.1),
                    highlightColor: color.withOpacity(0.05),
                    child: Padding(
                      padding: const EdgeInsets.all(20),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Icon(icon, color: effectiveIconColor, size: 28),
                          const SizedBox(height: 12),
                          Text(
                            value,
                            style: TextStyle(
                              fontSize: 32,
                              fontWeight: FontWeight.bold,
                              color: effectiveIconColor,
                            ),
                          ),
                          const SizedBox(height: 4),
                          Text(
                            label,
                            style: Theme.of(context).textTheme.bodySmall,
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

class _ObservabilityWidget extends StatelessWidget {
  final DashboardSnapshot dashboard;

  const _ObservabilityWidget({required this.dashboard});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final int missionCount =
        dashboard.agents.where((a) => a.isRunning).length; // Simulated
    final bool systemHealthy = dashboard.statuses.isNotEmpty;

    return Semantics(
      label: 'Observability Health Metrics Widget',
      child: SizedBox(
        width: 380,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(20),
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: ColorFilter.matrix(<double>[
                1.213,
                -0.213,
                -0.072,
                0,
                0,
                -0.213,
                1.213,
                -0.072,
                0,
                0,
                -0.213,
                -0.213,
                1.213,
                0,
                0,
                0,
                0,
                0,
                1,
                0,
              ]),
              inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            ),
            child: Container(
              decoration: BoxDecoration(
                color: colors.surface.withOpacity(0.15),
                borderRadius: BorderRadius.circular(20),
                border: Border.all(
                  color: colors.outlineVariant.withOpacity(0.4),
                  width: 1.5,
                ),
              ),
              child: Padding(
                padding: const EdgeInsets.all(24.0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        Icon(Icons.insights, color: colors.primary, size: 28),
                        const SizedBox(width: 12),
                        const Expanded(
                          child: Text(
                            'System Observability',
                            style: TextStyle(
                              fontFamily: 'Outfit',
                              fontSize: 20,
                              fontWeight: FontWeight.bold,
                            ),
                            overflow: TextOverflow.ellipsis,
                          ),
                        ),
                        Container(
                          padding: const EdgeInsets.symmetric(
                            horizontal: 10,
                            vertical: 4,
                          ),
                          decoration: BoxDecoration(
                            color:
                                systemHealthy
                                    ? Colors.green.withOpacity(0.2)
                                    : colors.error.withOpacity(0.2),
                            borderRadius: BorderRadius.circular(12),
                            border: Border.all(
                              color:
                                  systemHealthy
                                      ? Colors.green.withOpacity(0.5)
                                      : colors.error.withOpacity(0.5),
                            ),
                          ),
                          child: Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              Container(
                                width: 8,
                                height: 8,
                                decoration: BoxDecoration(
                                  color:
                                      systemHealthy
                                          ? Colors.green
                                          : colors.error,
                                  shape: BoxShape.circle,
                                ),
                              ),
                              const SizedBox(width: 6),
                              Text(
                                systemHealthy ? 'HEALTHY' : 'DEGRADED',
                                style: TextStyle(
                                  fontSize: 10,
                                  fontWeight: FontWeight.bold,
                                  color:
                                      systemHealthy
                                          ? Colors.green
                                          : colors.error,
                                  letterSpacing: 1.2,
                                ),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                    const SizedBox(height: 20),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Expanded(
                          child: _MetricsColumn(
                            label: 'ACTIVE MISSIONS',
                            value: '$missionCount',
                            icon: Icons.track_changes,
                          ),
                        ),
                        Expanded(
                          child: _MetricsColumn(
                            label: 'TOTAL EVENTS',
                            value: '${dashboard.statuses.length}',
                            icon: Icons.timeline,
                          ),
                        ),
                        const Expanded(
                          child: _MetricsColumn(
                            label: 'NODE LATENCY',
                            value: '14ms', // Placeholder for UX preview
                            icon: Icons.speed,
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
      ),
    );
  }
}

class _MetricsColumn extends StatelessWidget {
  final String label;
  final String value;
  final IconData icon;

  const _MetricsColumn({
    required this.label,
    required this.value,
    required this.icon,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Icon(
              icon,
              size: 14,
              color: Theme.of(context).colorScheme.onSurfaceVariant,
            ),
            const SizedBox(width: 4),
            Expanded(
              child: Text(
                label,
                style: TextStyle(
                  fontSize: 10,
                  color: Theme.of(context).colorScheme.onSurfaceVariant,
                  fontWeight: FontWeight.w600,
                  letterSpacing: 0.5,
                ),
                overflow: TextOverflow.ellipsis,
              ),
            ),
          ],
        ),
        const SizedBox(height: 6),
        Text(
          value,
          style: const TextStyle(
            fontFamily: 'Inter',
            fontSize: 24,
            fontWeight: FontWeight.bold,
          ),
        ),
      ],
    );
  }
}
