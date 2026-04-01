import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:ohc_app/models/dashboard.dart';

class ObservabilityWidget extends StatelessWidget {
  final DashboardSnapshot data;

  const ObservabilityWidget({super.key, required this.data});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    final int activeAgents = data.agents.where((a) => a.isRunning).length;
    final int totalUpdates = data.statuses.length;
    final int openMeetings = data.meetings.length;
    final int orgMembers = data.organization.members.length;

    // We calculate a pseudo "System Health" metric based on agent activity and errors.
    // For now, let's just make it a premium display.
    final String healthStatus = activeAgents > 0 ? "Healthy" : "Idle";
    final Color healthColor = activeAgents > 0
        ? Colors.greenAccent
        : Colors.orangeAccent;

    return Semantics(
      label:
          'Observability Widget. System Health: $healthStatus. Active Agents: $activeAgents. Dashboard Updates: $totalUpdates.',
      child: Container(
        width: double.infinity,
        margin: const EdgeInsets.only(bottom: 32),
        child: ClipRRect(
          borderRadius: BorderRadius.circular(16),
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
              padding: const EdgeInsets.all(24),
              decoration: BoxDecoration(
                color: colors.surfaceContainerHighest.withOpacity(0.3),
                borderRadius: BorderRadius.circular(16),
                border: Border.all(
                  color: colors.outlineVariant.withOpacity(0.4),
                ),
                gradient: LinearGradient(
                  begin: Alignment.topLeft,
                  end: Alignment.bottomRight,
                  colors: [
                    colors.primary.withOpacity(0.1),
                    colors.secondary.withOpacity(0.05),
                  ],
                ),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Icon(
                        Icons.monitor_heart,
                        color: colors.primary,
                        size: 28,
                      ),
                      const SizedBox(width: 12),
                      Text(
                        'System Observability',
                        style: Theme.of(context).textTheme.titleLarge?.copyWith(
                          fontWeight: FontWeight.bold,
                          fontFamily: 'Outfit',
                        ),
                      ),
                      const Spacer(),
                      Container(
                        padding: const EdgeInsets.symmetric(
                          horizontal: 12,
                          vertical: 6,
                        ),
                        decoration: BoxDecoration(
                          color: healthColor.withOpacity(0.2),
                          borderRadius: BorderRadius.circular(20),
                          border: Border.all(
                            color: healthColor.withOpacity(0.5),
                          ),
                        ),
                        child: Row(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            Container(
                              width: 8,
                              height: 8,
                              decoration: BoxDecoration(
                                color: healthColor,
                                shape: BoxShape.circle,
                              ),
                            ),
                            const SizedBox(width: 8),
                            Text(
                              healthStatus,
                              style: TextStyle(
                                color: healthColor,
                                fontWeight: FontWeight.bold,
                                fontSize: 12,
                              ),
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 24),
                  SizedBox(
                    width: double.infinity,
                    child: Wrap(
                      alignment: WrapAlignment.spaceAround,
                      spacing: 16,
                      runSpacing: 16,
                      children: [
                        _MetricItem(
                          label: 'Active Agents',
                          value: activeAgents.toString(),
                          icon: Icons.smart_toy,
                        ),
                        _MetricItem(
                          label: 'Dashboard Updates',
                          value: totalUpdates.toString(),
                          icon: Icons.pending_actions,
                        ),
                        _MetricItem(
                          label: 'Open Meetings',
                          value: openMeetings.toString(),
                          icon: Icons.video_call,
                        ),
                        _MetricItem(
                          label: 'Total Org Members',
                          value: orgMembers.toString(),
                          icon: Icons.people,
                        ),
                      ],
                    ),
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

class _MetricItem extends StatelessWidget {
  final String label;
  final String value;
  final IconData icon;

  const _MetricItem({
    required this.label,
    required this.value,
    required this.icon,
  });

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return Column(
      children: [
        Icon(icon, color: colors.onSurfaceVariant, size: 24),
        const SizedBox(height: 8),
        Text(
          value,
          style: TextStyle(
            fontSize: 28,
            fontWeight: FontWeight.bold,
            fontFamily: 'Outfit',
            color: colors.primary,
          ),
        ),
        const SizedBox(height: 4),
        Text(
          label,
          style: Theme.of(context).textTheme.bodySmall?.copyWith(
            color: colors.onSurfaceVariant,
            fontFamily: 'Inter',
          ),
        ),
      ],
    );
  }
}
