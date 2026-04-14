import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class DiagnosticsScreen extends ConsumerWidget {
  const DiagnosticsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'Diagnostics Dashboard',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
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
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 24,
                        fontWeight: FontWeight.bold,
                      ),
                      textAlign: TextAlign.center,
                    ),
                    const SizedBox(height: 24),
                    const _DiagnosticRow(
                      title: 'Database (PostgreSQL / SQLite)',
                      status: 'Connected',
                      icon: Icons.storage,
                    ),
                    const SizedBox(height: 16),
                    const _DiagnosticRow(
                      title: 'Redis Cache',
                      status: 'Connected',
                      icon: Icons.memory,
                    ),
                    const SizedBox(height: 16),
                    const _DiagnosticRow(
                      title: 'AI Provider APIs',
                      status: 'Connected',
                      icon: Icons.psychology,
                    ),
                    const SizedBox(height: 32),
                    ElevatedButton(
                      onPressed: () {},
                      style: ElevatedButton.styleFrom(
                        padding: const EdgeInsets.symmetric(vertical: 16),
                      ),
                      child: const Text(
                        'Run Diagnostics',
                        style: TextStyle(fontFamily: 'Inter', fontSize: 16),
                      ),
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
            child: Text(
              title,
              style: const TextStyle(
                fontFamily: 'Inter',
                fontSize: 16,
                fontWeight: FontWeight.w500,
              ),
            ),
          ),
          const Icon(Icons.check_circle, color: Colors.green),
          const SizedBox(width: 8),
          Text(
            status,
            style: const TextStyle(fontFamily: 'Inter', color: Colors.green),
          ),
        ],
      ),
    );
  }
}
