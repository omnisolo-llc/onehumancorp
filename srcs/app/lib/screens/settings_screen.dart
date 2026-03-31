import 'package:flutter/material.dart';
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
                    leading: CircleAvatar(
                      child: Text(user.name.substring(0, 1).toUpperCase()),
                    ),
                    title: Text(user.name),
                    subtitle: Text(user.email),
                  ),
                  const Divider(),
                ],

                _SectionHeader(title: 'Communication'),
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
                  subtitle: const Text('App manages local backend lifecycle'),
                  value: settings.standaloneMode,
                  onChanged:
                      (value) => ref
                          .read(clientSettingsProvider.notifier)
                          .updateStandaloneMode(value),
                ),

                if (settings.standaloneMode) ...[
                  const Divider(),
                  _SectionHeader(title: 'Local Backend'),
                  _LocalBackendStatusCard(),
                ],

                const Divider(),
                _SectionHeader(title: 'Account'),
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
          (context) => AlertDialog(
            title: const Text('Edit Backend URL'),
            content: TextField(
              controller: controller,
              decoration: const InputDecoration(
                labelText: 'URL (e.g. http://localhost:8080)',
              ),
            ),
            actions: [
              TextButton(
                onPressed: () => Navigator.pop(context),
                child: const Text('Cancel'),
              ),
              TextButton(
                onPressed: () => Navigator.pop(context, controller.text),
                child: const Text('Save'),
              ),
            ],
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

  @override
  Widget build(BuildContext context) {
    final manager = ref.watch(localManagerServiceProvider);

    return FutureBuilder<bool>(
      future: manager.isServiceRunning(),
      builder: (context, snapshot) {
        final running = snapshot.data ?? false;
        return Card(
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
                              if (mounted)
                                setState(() => _isRunningDoctor = false);
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
        );
      },
    );
  }
}
