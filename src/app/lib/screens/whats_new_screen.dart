import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/api_service.dart';

final changelogProvider = FutureProvider.autoDispose<List<Map<String, dynamic>>>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) return [];
  final res = await api.client.get(Uri.parse('${api.baseUrl}/api/changelog'));
  if (res.statusCode == 200) {
    final List<dynamic> data = jsonDecode(res.body);
    return data.cast<Map<String, dynamic>>();
  }
  return [];
});

class WhatsNewScreen extends ConsumerWidget {
  const WhatsNewScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final asyncData = ref.watch(changelogProvider);

    return Scaffold(
      appBar: AppBar(title: const Text("What's New", style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold))),
      body: asyncData.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (e, _) => Center(child: Text('Error: $e')),
        data: (logs) {
          if (logs.isEmpty) {
            return const Center(child: Text("No updates available."));
          }
          return ListView.builder(
            padding: const EdgeInsets.all(16),
            itemCount: logs.length,
            itemBuilder: (context, index) {
              final log = logs[index];
              return Padding(
                padding: const EdgeInsets.only(bottom: 16.0),
                child: GlassCard(
                  child: Padding(
                    padding: const EdgeInsets.all(16.0),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          log['version'] ?? 'Update',
                          style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          log['date'] ?? '',
                          style: TextStyle(color: Theme.of(context).colorScheme.onSurface.withOpacity(0.7)),
                        ),
                        const SizedBox(height: 12),
                        Text(log['description'] ?? '', style: const TextStyle(height: 1.5)),
                        if (log['features'] != null) ...[
                          const SizedBox(height: 12),
                          const Text('New Features:', style: TextStyle(fontWeight: FontWeight.bold)),
                          ...List<String>.from(log['features']).map((f) => Text('• $f', style: const TextStyle(height: 1.5))),
                        ]
                      ],
                    ),
                  ),
                ),
              );
            },
          );
        },
      ),
    );
  }
}
