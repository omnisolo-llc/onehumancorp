import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/widgets/swarm_observability_widget.dart';
import 'package:ohc_app/services/api_service.dart';

final kairosStreamProvider = StreamProvider.autoDispose<dynamic>((ref) {
  final api = ref.watch(apiServiceProvider);
  if (api == null) return const Stream.empty();

  // Using web_socket_channel to connect to the backend stream
  String wsUrl = api.baseUrl.replaceFirst('https', 'wss').replaceFirst('http', 'ws');
  final uri = Uri.parse('$wsUrl/api/kairos/stream');
  final channel = WebSocketChannel.connect(uri);

  ref.onDispose(() => channel.sink.close());

  return channel.stream.map((event) => jsonDecode(event as String));
});

class KairosDashboardScreen extends ConsumerWidget {
  const KairosDashboardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final textTheme = Theme.of(context).textTheme;
    final streamAsync = ref.watch(kairosStreamProvider);

    return Scaffold(
      appBar: AppBar(
        title: Text(
          'KAIROS Swarm Analytics',
          style: textTheme.titleLarge?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Expanded(
              child: GlassCard(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text('Shared Task Queue', style: textTheme.titleMedium?.copyWith(fontFamily: 'Inter')),
                    const SizedBox(height: 16),
                    Expanded(
                      child: streamAsync.when(
                        data: (data) {
                          final tasks = data['tasks'] as List<dynamic>? ?? [];
                          if (tasks.isEmpty) {
                            return const Text('No pending tasks.', style: TextStyle(fontFamily: 'Inter'));
                          }
                          return ListView.builder(
                            itemCount: tasks.length,
                            itemBuilder: (context, index) {
                              return ListTile(
                                leading: const Icon(Icons.task_alt, color: Colors.white70),
                                title: Text(tasks[index].toString(), style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
                              );
                            },
                          );
                        },
                        loading: () => const Center(child: CircularProgressIndicator()),
                        error: (err, stack) => Text('Error connecting to stream: $err', style: const TextStyle(color: Colors.red)),
                      ),
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(width: 16),
            const Expanded(
              flex: 2,
              child: SwarmObservabilityWidget(),
            ),
            const SizedBox(width: 16),
            Expanded(
              child: GlassCard(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text('AutoDream Memory', style: textTheme.titleMedium?.copyWith(fontFamily: 'Inter')),
                    const SizedBox(height: 16),
                    Expanded(
                      child: streamAsync.when(
                        data: (data) {
                          final memories = data['memories'] as List<dynamic>? ?? [];
                          if (memories.isEmpty) {
                            return const Text('Memory consolidation idle.', style: TextStyle(fontFamily: 'Inter'));
                          }
                          return ListView.builder(
                            itemCount: memories.length,
                            itemBuilder: (context, index) {
                              return ListTile(
                                leading: const Icon(Icons.memory, color: Colors.white70),
                                title: Text('Vector: ${memories[index]}', style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
                              );
                            },
                          );
                        },
                        loading: () => const Center(child: CircularProgressIndicator()),
                        error: (err, stack) => Text('Error connecting to stream: $err', style: const TextStyle(color: Colors.red)),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
