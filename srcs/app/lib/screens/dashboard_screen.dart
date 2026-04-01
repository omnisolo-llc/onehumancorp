import 'package:flutter/material.dart';
import 'dart:ui';
import 'package:flutter_riverpod/flutter_riverpod.dart';
// import 'package:flutter_svg/flutter_svg.dart'; // Temporarily disabled for Bazel build
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/widgets/observability_widget.dart';
import 'package:ohc_app/services/api_service.dart';

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
        _SectionTitle('System Observability'),
        const SizedBox(height: 16),
        ObservabilityWidget(isConnected: data.agents.where((a) => a.isRunning).isNotEmpty || data.agents.isEmpty, activeMissions: data.statuses.length, latencyMs: 12, cpuUsage: data.agents.where((a) => a.isRunning).length / (data.agents.isNotEmpty ? data.agents.length : 1), memoryUsage: 0.45),
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
      return word[0].toUpperCase() + word.substring(1).toLowerCase();
    }).join(' ');

    return Semantics(
      label: 'Scale $formattedRole role',
      child: Tooltip(
        message: 'Manage $formattedRole Allocation',
        child: SizedBox(
          width: 320,
          child: ClipRRect(
            borderRadius: BorderRadius.circular(16),
            child: BackdropFilter(
              filter: ImageFilter.compose(
                outer: ColorFilter.matrix(<double>[
                  1.168,
                  -0.153,
                  -0.015,
                  0,
                  0,
                  -0.046,
                  1.061,
                  -0.015,
                  0,
                  0,
                  -0.046,
                  -0.152,
                  1.198,
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
                  color: colors.surfaceContainerHighest.withValues(alpha: 0.4),
                  borderRadius: BorderRadius.circular(16),
                  border: Border.all(color: colors.outlineVariant.withValues(alpha: 0.5)),
                ),
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
      child: Tooltip(
        message: 'View $label',
        child: SizedBox(
          width: 200,
          child: ClipRRect(
            borderRadius: BorderRadius.circular(16),
            child: BackdropFilter(
              filter: ImageFilter.compose(
                outer: ColorFilter.matrix(<double>[
                  1.168,
                  -0.153,
                  -0.015,
                  0,
                  0,
                  -0.046,
                  1.061,
                  -0.015,
                  0,
                  0,
                  -0.046,
                  -0.152,
                  1.198,
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
                  color: Theme.of(context).colorScheme.surface.withValues(alpha: 0.03),
                  borderRadius: BorderRadius.circular(16),
                  border: Border.all(
                    color: Theme.of(
                      context,
                    ).colorScheme.onSurface.withValues(alpha: 0.08),
                  ),
                ),
                child: Material(
                  color: Colors.transparent,
                  child: Semantics(
                    button: true,
                    label: '$label: $value action',
                    child: InkWell(
                      onTap: () {},
                      borderRadius: BorderRadius.circular(16),
                      splashColor: color.withValues(alpha: 0.1),
                      highlightColor: color.withValues(alpha: 0.05),
                      child: Padding(
                        padding: const EdgeInsets.all(24),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Icon(icon, color: effectiveIconColor, size: 32),
                            const SizedBox(height: 16),
                            Text(
                              value,
                              style: TextStyle(
                                fontSize: 36,
                                fontWeight: FontWeight.bold,
                                color: effectiveIconColor,
                                fontFamily: 'Inter',
                              ),
                            ),
                            const SizedBox(height: 6),
                            Text(
                              label,
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
