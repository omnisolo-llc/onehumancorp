import '../widgets/glass_card.dart';
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:ohc_app/services/local_manager_service.dart';

class SettingsScreen extends ConsumerWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final user = ref.watch(authStateProvider).valueOrNull;
    final clientSettingsAsync = ref.watch(clientSettingsProvider);
    // Trigger lifecycle management
    ref.watch(standaloneManagerProvider);

    return Scaffold(
      appBar: AppBar(title: const Text('Settings')),
      body: clientSettingsAsync.when(
        loading:
            () => Center(
              child: CircularProgressIndicator(
                color: Theme.of(context).colorScheme.primary,
              ),
            ),
        error:
            (err, _) => Center(
              child: Text(
                'Error: $err',
                style: TextStyle(color: Theme.of(context).colorScheme.error),
              ),
            ),
        data:
            (settings) => ListView(
              padding: const EdgeInsets.all(24),
              children: [
                if (user != null) ...[
                  ListTile(
                    leading: Icon(
                      Icons.logout,
                      color: Theme.of(context).colorScheme.error,
                    ),
                    title: Text(
                      'Sign Out',
                      style: TextStyle(
                        color: Theme.of(context).colorScheme.error,
                      ),
                    ),
                    onTap: () => ref.read(authStateProvider.notifier).logout(),
                  ),
                ],
                if (user != null) ...[
                  ListTile(
                    leading: CircleAvatar(
                      child: Text(user.name.substring(0, 1).toUpperCase()),
                    ),
                    title: Text(user.name),
                    subtitle: Text(user.email),
                  ),
                  const Divider(),
                ],

                const _SectionHeader(title: 'Communication'),
                GlassCard(
                  child: ListTile(
                    leading: Icon(
                      settings.standaloneMode
                          ? Icons.laptop_windows
                          : Icons.cloud_queue,
                    ),
                    title: Text(
                      settings.standaloneMode
                          ? 'Desktop Standalone Mode'
                          : 'Remote Client Mode',
                    ),
                    subtitle: Text(
                      settings.standaloneMode
                          ? 'This device manages a local backend and lightweight local services.'
                          : 'This app acts as a UI for a remote OHC server. Point Backend URL at a cloud or headless deployment.',
                    ),
                  ),
                ),
                ListTile(
                  leading: const Icon(Icons.link),
                  title: const Text('Backend URL'),
                  subtitle: Text(settings.backendUrl),
                  trailing: IconButton(
                    icon: const Icon(Icons.edit),
                    tooltip: 'Edit Backend URL',
                    onPressed:
                        () =>
                            _editBackendUrl(context, ref, settings.backendUrl),
                  ),
                ),

                SwitchListTile(
                  secondary: const Icon(Icons.computer),
                  title: const Text('Standalone Mode'),
                  subtitle: const Text(
                    'Run a local desktop backend. Disable this to use the app as a remote client.',
                  ),
                  value: settings.standaloneMode,
                  onChanged:
                      (value) => ref
                          .read(clientSettingsProvider.notifier)
                          .updateStandaloneMode(value),
                ),

                if (settings.standaloneMode) ...[
                  const Divider(),
                  const _SectionHeader(title: 'Local Backend'),
                  const _LocalBackendStatusCard(),
                ],

                const Divider(),
                const _SectionHeader(title: 'Account'),
                ListTile(
                  leading: const Icon(Icons.business),
                  title: const Text('Organization'),
                  subtitle: Text(user?.organizationId ?? '—'),
                ),
                ListTile(
                  leading: const Icon(Icons.verified_user),
                  title: const Text('Role'),
                  subtitle: Text(user?.role ?? '—'),
                ),
                const Divider(),
                if (user != null)
                  ListTile(
                    leading: Icon(
                      Icons.logout,
                      color: Theme.of(context).colorScheme.error,
                    ),
                    title: Text(
                      'Sign Out',
                      style: TextStyle(
                        color: Theme.of(context).colorScheme.error,
                      ),
                    ),
                    onTap: () => ref.read(authStateProvider.notifier).logout(),
                  ),
              ],
            ),
      ),
    );
  }

  Future<void> _editBackendUrl(
    BuildContext context,
    WidgetRef ref,
    String current,
  ) async {
    final controller = TextEditingController(text: current);
    final result = await showDialog<String>(
      context: context,
      builder:
          (context) => Dialog(
            backgroundColor: Colors.transparent,
            child: ClipRRect(
              borderRadius: BorderRadius.circular(16),
              child: BackdropFilter(
                filter: ImageFilter.compose(
                  outer: ColorFilter.matrix(const <double>[
                    1.787, -0.715, -0.072, 0, 0,
                    -0.213, 1.285, -0.072, 0, 0,
                    -0.213, -0.715, 1.928, 0, 0,
                    0, 0, 0, 1, 0,
                  ]),
                  inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                ),
                child: Container(
                  padding: const EdgeInsets.all(24),
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.surface.withValues(alpha: 0.8),
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(
                      color: Theme.of(context).colorScheme.outlineVariant.withValues(alpha: 0.5),
                    ),
                  ),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        'Edit Backend URL',
                        style: TextStyle(
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                          fontFamily: 'Outfit',
                          color: Theme.of(context).colorScheme.onSurface,
                        ),
                      ),
                      const SizedBox(height: 16),
                      Text(
                        'Configure the URL for the OHC Cloud or local backend.',
                        style: TextStyle(
                          color: Theme.of(context).colorScheme.onSurfaceVariant,
                          fontFamily: 'Inter',
                        ),
                      ),
                      const SizedBox(height: 24),
                      TextField(
                        controller: controller,
                        decoration: InputDecoration(
                          labelText: 'URL (e.g. http://localhost:8080)',
                          prefixIcon: const Icon(Icons.link),
                          border: OutlineInputBorder(
                            borderRadius: BorderRadius.circular(12),
                          ),
                        ),
                      ),
                      const SizedBox(height: 32),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.end,
                        children: [
                          TextButton(
                            onPressed: () => Navigator.pop(context),
                            child: const Text('Cancel', style: TextStyle(fontFamily: 'Inter')),
                          ),
                          const SizedBox(width: 8),
                          FilledButton(
                            onPressed: () => Navigator.pop(context, controller.text),
                            style: FilledButton.styleFrom(
                              shape: RoundedRectangleBorder(
                                borderRadius: BorderRadius.circular(12),
                              ),
                            ),
                            child: const Text('Save', style: TextStyle(fontFamily: 'Inter')),
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
    if (result != null && result.isNotEmpty) {
      ref.read(clientSettingsProvider.notifier).updateBackendUrl(result);
    }
  }
}

class _SectionHeader extends StatelessWidget {
  final String title;
  const _SectionHeader({required this.title});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0),
      child: Text(
        title.toUpperCase(),
        style: Theme.of(context).textTheme.labelLarge?.copyWith(
          color: Theme.of(context).colorScheme.primary,
          fontWeight: FontWeight.bold,
        ),
      ),
    );
  }
}

class _LocalBackendStatusCard extends ConsumerStatefulWidget {
  const _LocalBackendStatusCard();

  @override
  ConsumerState<_LocalBackendStatusCard> createState() =>
      _LocalBackendStatusCardState();
}

class _LocalBackendStatusCardState
    extends ConsumerState<_LocalBackendStatusCard> {
  bool _isToggling = false;
  bool _isRunningDoctor = false;
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final manager = ref.watch(localManagerServiceProvider);

    return FutureBuilder<bool>(
      future: manager.isServiceRunning(),
      builder: (context, snapshot) {
        final running = snapshot.data ?? false;
        return Semantics(
          label:
              'Local Backend Service Status: ${running ? "Running" : "Stopped"}',
          child: GlassCard(
        child: Padding(
                      padding: const EdgeInsets.all(16.0),
                      child: Column(
                children: [
                  Row(
                    children: [
                      Icon(
                        running ? Icons.check_circle : Icons.error,
                        color:
                            running
                                ? Theme.of(context).colorScheme.primary
                                : Theme.of(context).colorScheme.error,
                      ),
                      const SizedBox(width: 8),
                      Text(running ? 'Service Running' : 'Service Stopped'),
                      const Spacer(),
                      ElevatedButton(
                        onPressed:
                            _isToggling
                                ? null
                                : () async {
                                  setState(() => _isToggling = true);
                                  try {
                                    if (running) {
                                      await manager.stopService();
                                    } else {
                                      await manager.startService();
                                    }
                                  } finally {
                                    if (mounted) {
                                      setState(() => _isToggling = false);
                                    }
                                  }
                                },
                        child:
                            _isToggling
                                ? const SizedBox(
                                  width: 16,
                                  height: 16,
                                  child: CircularProgressIndicator(
                                    strokeWidth: 2,
                                  ),
                                )
                                : Text(running ? 'Stop' : 'Start'),
                      ),
                    ],
                  ),
                  const SizedBox(height: 8),
                  OutlinedButton.icon(
                    onPressed:
                        _isRunningDoctor
                            ? null
                            : () async {
                              setState(() => _isRunningDoctor = true);
                              try {
                                final report = await manager.runDoctor();
                                if (context.mounted) {
                                  showDialog(
                                    context: context,
                                    builder:
                                        (context) => AlertDialog(
                                          title: const Text('System Doctor'),
                                          content: SingleChildScrollView(
                                            child: Text(report),
                                          ),
                                          actions: [
                                            TextButton(
                                              onPressed:
                                                  () => Navigator.pop(context),
                                              child: const Text('Close'),
                                            ),
                                          ],
                                        ),
                                  );
                                }
                              } finally {
                                if (mounted) {
                                  setState(() => _isRunningDoctor = false);
                                }
                              }
                            },
                    icon:
                        _isRunningDoctor
                            ? const SizedBox(
                              width: 16,
                              height: 16,
                              child: CircularProgressIndicator(strokeWidth: 2),
                            )
                            : const Icon(Icons.medical_services),
                    label: const Text('Run Doctor Diagnostics'),
                  ),
                ],
              ),
            ),
      ),
);
      },
    );
  }
}
