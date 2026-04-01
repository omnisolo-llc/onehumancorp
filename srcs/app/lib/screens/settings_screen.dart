import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';

class SettingsScreen extends ConsumerWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final user = ref.watch(authStateProvider).valueOrNull;
    final clientSettingsAsync = ref.watch(clientSettingsProvider);

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
