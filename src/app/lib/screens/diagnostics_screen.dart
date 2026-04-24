
import 'package:ohc_app/widgets/swarm_observability_widget.dart';
import 'package:ohc_app/widgets/swarm_velocity_widget.dart';
import 'package:ohc_app/widgets/hybrid_observability_widget.dart';
import 'package:ohc_app/widgets/hybrid_telemetry_widget.dart';
import 'package:ohc_app/widgets/sub_agent_queue_widget.dart';
  import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class DiagnosticsScreen extends ConsumerWidget {
  const DiagnosticsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final dashboardAsync = ref.watch(FutureProvider((ref) async {
      final api = ref.read(apiServiceProvider);
      if (api == null) return null;
      return api.getDashboard();
    }));

    return Scaffold(
      appBar: AppBar(
        title: const Text('Diagnostics Dashboard', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 800),
          child: SingleChildScrollView(
            padding: const EdgeInsets.all(24.0),
            child: GlassCard(
              child: Padding(
                padding: const EdgeInsets.all(32.0),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    const Text(
                      'Day One Setup Health Check',
                      style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold),
                      textAlign: TextAlign.center,
                    ),
                    const SizedBox(height: 24),
                    const _DiagnosticRow(title: 'Database (PostgreSQL / SQLite)', status: 'Connected', icon: Icons.storage),
                    const SizedBox(height: 16),
                    const _DiagnosticRow(title: 'Redis Cache', status: 'Connected', icon: Icons.memory),
                    const SizedBox(height: 16),
                    const _DiagnosticRow(title: 'AI Provider APIs', status: 'Connected', icon: Icons.psychology),
                    const SizedBox(height: 24),
                    const Divider(color: Colors.white24),
                    const SizedBox(height: 24),
                    const Text(
                      'Hybrid Health Status',
                      style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(height: 16),
                    dashboardAsync.when(
                      data: (snapshot) {
                        if (snapshot == null || snapshot.hybridHealth == null) {
                          return const Text('Hybrid health data unavailable', style: TextStyle(color: Colors.white70));
                        }
                        final health = snapshot.hybridHealth!;
                        return Column(
                          children: [
                            _HybridHealthRow(
                              label: 'Execution Mode',
                              value: health.mode.toUpperCase(),
                              icon: Icons.settings_input_component,
                            ),
                            _HybridHealthRow(
                              label: 'Cloud Connectivity',
                              value: health.cloudConnected ? 'CONNECTED' : 'DISCONNECTED',
                              icon: Icons.cloud_sync,
                              isError: !health.cloudConnected,
                            ),
                            _HybridHealthRow(
                              label: 'Mesh Status',
                              value: health.meshActive ? 'ACTIVE' : 'INACTIVE',
                              icon: Icons.hub,
                              isError: !health.meshActive,
                            ),
                            _HybridHealthRow(
                              label: 'Stuck Missions',
                              value: health.stuckMissions.toString(),
                              icon: Icons.bug_report,
                              isError: health.stuckMissions > 0,
                            ),
                            _HybridHealthRow(
    label: 'Sync Backlog',
    value: health.syncBacklog.toString(),
    icon: Icons.sync_problem,
  ),
  const SizedBox(height: 16),
  const SwarmObservabilityWidget(),
  const SizedBox(height: 16),
  const SwarmVelocityWidget(),
  const SizedBox(height: 16),
  const HybridObservabilityWidget(),
  const SizedBox(height: 16),
  const HybridTelemetryWidget(),
  const SizedBox(height: 16),
  SubAgentQueueWidget(statuses: snapshot.statuses),
                          ],
                        );
                      },
                      loading: () => const Center(child: CircularProgressIndicator()),
                      error: (err, stack) => SelectableText('Error loading health: $err', style: const TextStyle(color: Colors.redAccent)),
                    ),
                    const SizedBox(height: 32),
                    ElevatedButton(
                      onPressed: () {},
                      style: ElevatedButton.styleFrom(
                        padding: const EdgeInsets.symmetric(vertical: 16),
                      ),
                      child: const Text('Run Diagnostics', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
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

class _HybridHealthRow extends StatelessWidget {
  final String label;
  final String value;
  final IconData icon;
  final bool isError;

  const _HybridHealthRow({
    required this.label,
    required this.value,
    required this.icon,
    this.isError = false,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0),
      child: Row(
        children: [
          Icon(icon, size: 20, color: isError ? Colors.redAccent : Colors.white70),
          const SizedBox(width: 12),
          Text(label, style: const TextStyle(fontFamily: 'Inter', color: Colors.white70)),
          const Spacer(),
          Text(
            value,
            style: TextStyle(
              fontFamily: 'Inter',
              fontWeight: FontWeight.bold,
              color: isError ? Colors.redAccent : Theme.of(context).colorScheme.primary,
            ),
          ),
        ],
      ),
    );
  }
}

class _DiagnosticRow extends StatelessWidget {
  final String title;
  final String status;
  final IconData icon;

  const _DiagnosticRow({
    required this.title,
    required this.status,
    required this.icon,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surface.withValues(alpha: 0.1),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.white.withValues(alpha: 0.2)),
      ),
      child: Row(
        children: [
          Icon(icon, color: Theme.of(context).colorScheme.primary),
          const SizedBox(width: 16),
          Expanded(
            child: Text(title, style: const TextStyle(fontFamily: 'Inter', fontSize: 16, fontWeight: FontWeight.w500)),
          ),
          const Icon(Icons.check_circle, color: Colors.green),
          const SizedBox(width: 8),
          Text(status, style: const TextStyle(fontFamily: 'Inter', color: Colors.green)),
        ],
      ),
    );
  }
}
