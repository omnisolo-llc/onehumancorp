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
      appBar: AppBar(
        title: const Text(
          'Settings',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
      ),
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
                      backgroundColor: Theme.of(context).colorScheme.primaryContainer,
                      child: Text(
                        user.name.isNotEmpty ? user.name.substring(0, 1).toUpperCase() : '?',
                        style: TextStyle(
                          color: Theme.of(context).colorScheme.onPrimaryContainer,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ),
                    title: Text(
                      user.name,
                      style: const TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
                    ),
                    subtitle: Text(
                      user.email,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                  ),
                  const Divider(),
                ],

                const _SectionHeader(title: 'Communication'),
                Card(
                  elevation: 0,
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(16),
                    side: BorderSide(color: Theme.of(context).colorScheme.outlineVariant),
                  ),
                  child: ListTile(
                    contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                    leading: Icon(
                      settings.standaloneMode
                          ? Icons.laptop_windows
                          : Icons.cloud_queue,
                      color: Theme.of(context).colorScheme.primary,
                      size: 32,
                    ),
                    title: Text(
                      settings.standaloneMode
                          ? 'Desktop Standalone Mode'
                          : 'Remote Client Mode',
                      style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Inter'),
                    ),
                    subtitle: Padding(
                      padding: const EdgeInsets.only(top: 8.0),
                      child: Text(
                        settings.standaloneMode
                            ? 'This device manages a local backend and lightweight local services.'
                            : 'This app acts as a UI for a remote OHC server. Point Backend URL at a cloud or headless deployment.',
                        style: const TextStyle(fontFamily: 'Inter'),
                      ),
                    ),
                  ),
                ),
                const SizedBox(height: 16),
                ListTile(
                  leading: Icon(Icons.link, color: Theme.of(context).colorScheme.secondary),
                  title: const Text('Backend URL', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.w600)),
                  subtitle: Text(settings.backendUrl, style: const TextStyle(fontFamily: 'Inter')),
                  trailing: Tooltip(
                    message: 'Edit Backend URL',
                    child: IconButton(
                      icon: Icon(Icons.edit, color: Theme.of(context).colorScheme.primary),
                      onPressed:
                          () =>
                              _editBackendUrl(context, ref, settings.backendUrl),
                    ),
                  ),
                ),

                SwitchListTile(
                  secondary: Icon(Icons.computer, color: Theme.of(context).colorScheme.tertiary),
                  title: const Text('Standalone Mode', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.w600)),
                  subtitle: const Text(
                    'Run a local desktop backend. Disable this to use the app as a remote client.',
                    style: TextStyle(fontFamily: 'Inter'),
                  ),
                  value: settings.standaloneMode,
                  activeColor: Theme.of(context).colorScheme.primary,
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
                  leading: Icon(Icons.business, color: Theme.of(context).colorScheme.onSurfaceVariant),
                  title: const Text('Organization', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.w600)),
                  subtitle: Text(user?.organizationId ?? '—', style: const TextStyle(fontFamily: 'Inter')),
                ),
                ListTile(
                  leading: Icon(Icons.verified_user, color: Theme.of(context).colorScheme.onSurfaceVariant),
                  title: const Text('Role', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.w600)),
                  subtitle: Text(user?.role ?? '—', style: const TextStyle(fontFamily: 'Inter')),
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
                      fontWeight: FontWeight.bold,
                      fontFamily: 'Inter',
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
            title: const Text('Edit Backend URL', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
            content: TextField(
              controller: controller,
              decoration: InputDecoration(
                labelText: 'URL (e.g. http://localhost:8080)',
                border: OutlineInputBorder(borderRadius: BorderRadius.circular(12)),
              ),
            ),
            actions: [
              TextButton(
                onPressed: () => Navigator.pop(context),
                child: const Text('Cancel'),
              ),
              FilledButton(
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
      padding: const EdgeInsets.symmetric(vertical: 16.0, horizontal: 8.0),
      child: Text(
        title.toUpperCase(),
        style: Theme.of(context).textTheme.labelLarge?.copyWith(
          color: Theme.of(context).colorScheme.primary,
          fontWeight: FontWeight.bold,
          letterSpacing: 1.2,
          fontFamily: 'Outfit',
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
        return Semantics(
          label:
              'Local Backend Service Status: ${running ? "Running" : "Stopped"}',
          child: Card(
            elevation: 0,
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(16),
              side: BorderSide(color: Theme.of(context).colorScheme.outlineVariant),
            ),
            child: Padding(
              padding: const EdgeInsets.all(20.0),
              child: Column(
                children: [
                  Row(
                    children: [
                      Icon(
                        running ? Icons.check_circle : Icons.error,
                        size: 32,
                        color:
                            running
                                ? Theme.of(context).colorScheme.primary
                                : Theme.of(context).colorScheme.error,
                      ),
                      const SizedBox(width: 12),
                      Expanded(
                        child: Text(
                          running ? 'Service Running' : 'Service Stopped',
                          style: const TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                            fontFamily: 'Inter',
                          ),
                        ),
                      ),
                      ElevatedButton(
                        style: ElevatedButton.styleFrom(
                          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
                        ),
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
                                  width: 20,
                                  height: 20,
                                  child: CircularProgressIndicator(
                                    strokeWidth: 2,
                                  ),
                                )
                                : Text(
                                    running ? 'Stop' : 'Start',
                                    style: const TextStyle(fontWeight: FontWeight.bold),
                                  ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 16),
                  SizedBox(
                    width: double.infinity,
                    child: OutlinedButton.icon(
                      style: OutlinedButton.styleFrom(
                        padding: const EdgeInsets.symmetric(vertical: 16),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                      ),
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
                                              title: const Text('System Doctor', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                                              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
                                              content: SingleChildScrollView(
                                                child: Text(report, style: const TextStyle(fontFamily: 'monospace')),
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
                                width: 20,
                                height: 20,
                                child: CircularProgressIndicator(strokeWidth: 2),
                              )
                              : const Icon(Icons.medical_services),
                      label: const Text(
                        'Run Doctor Diagnostics',
                        style: TextStyle(fontWeight: FontWeight.bold),
                      ),
                    ),
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
