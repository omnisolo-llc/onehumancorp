import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/widgets/pulse_animation.dart';
import 'package:ohc_app/models/shared_task.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/centrifuge_service.dart';

final sharedTasksProvider = FutureProvider.autoDispose<List<SharedTask>>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) return [];
  final rawTasks = await api.listSharedTasks();
  return rawTasks.map((t) => SharedTask.fromJson(t)).toList();
});

class KairosDashboardScreen extends ConsumerStatefulWidget {
  const KairosDashboardScreen({super.key});

  @override
  ConsumerState<KairosDashboardScreen> createState() => _KairosDashboardScreenState();
}

class _KairosDashboardScreenState extends ConsumerState<KairosDashboardScreen> {
  final GlobalKey<AnimatedListState> _listKey = GlobalKey<AnimatedListState>();
  final List<CentrifugeMessage> _liveMessages = [];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.transparent,
      appBar: AppBar(
        title: const Text('KAIROS Orchestration Dashboard', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        backgroundColor: Colors.transparent,
      ),
      body: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Expanded(
            flex: 2,
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('Shared Tasks', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
                  const SizedBox(height: 16),
                  Expanded(child: const _SharedTaskList()),
                ],
              ),
            ),
          ),
          Expanded(
            flex: 1,
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('Live Mesh Event Log', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
                  const SizedBox(height: 16),
                  Expanded(
                    child: _MeshEventLog(listKey: _listKey, liveMessages: _liveMessages),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _SharedTaskList extends ConsumerWidget {
  const _SharedTaskList();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tasksAsync = ref.watch(sharedTasksProvider);

    return tasksAsync.when(
      data: (tasks) => ListView.builder(
        itemCount: tasks.length,
        itemBuilder: (context, index) {
          final task = tasks[index];
          return Padding(
            padding: const EdgeInsets.only(bottom: 16.0),
            child: _TaskCard(task: task),
          );
        },
      ),
      loading: () => const Center(child: CircularProgressIndicator()),
      error: (err, stack) => Center(child: Text('Error: $err', style: TextStyle(color: Theme.of(context).colorScheme.error, fontFamily: 'Inter'))),
    );
  }
}

class _TaskCard extends StatelessWidget {
  final SharedTask task;

  const _TaskCard({required this.task});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final isActive = task.status.toUpperCase() == 'IN_PROGRESS';
    final isCompleted = task.status.toUpperCase() == 'COMPLETED';

    final card = GlassCard(
      color: colors.surfaceContainerHighest.withValues(alpha: 0.2),
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(
                  child: Text(
                    task.title,
                    style: const TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
                if (isActive)
                  PulseAnimation(
                    child: Container(
                      width: 12,
                      height: 12,
                      decoration: const BoxDecoration(
                        color: Colors.blueAccent,
                        shape: BoxShape.circle,
                      ),
                    ),
                  )
                else if (isCompleted)
                  const Icon(Icons.check_circle, color: Colors.greenAccent, size: 20)
              ],
            ),
            const SizedBox(height: 8),
            Text(
              'Status: ${task.status}',
              style: TextStyle(
                fontFamily: 'Inter',
                color: _getStatusColor(task.status),
              ),
            ),
            if (task.agentId != null) ...[
              const SizedBox(height: 4),
              Text(
                'Agent: ${task.agentId}',
                style: const TextStyle(fontFamily: 'Inter', color: Colors.grey),
              ),
            ],
            if (task.dependencies != null && task.dependencies!.isNotEmpty) ...[
              const SizedBox(height: 8),
              Text(
                'Dependencies: ${task.dependencies!.join(', ')}',
                style: const TextStyle(fontFamily: 'Inter', color: Colors.grey, fontSize: 12),
              ),
            ],
          ],
        ),
      ),
    );

    if (isActive) {
      return PulseAnimation(child: card);
    }

    return card;
  }

  Color _getStatusColor(String status) {
    switch (status.toUpperCase()) {
      case 'COMPLETED':
        return Colors.greenAccent;
      case 'IN_PROGRESS':
        return Colors.blueAccent;
      case 'FAILED':
        return Colors.redAccent;
      case 'REVIEW':
        return Colors.orangeAccent;
      case 'ASSIGNED':
        return Colors.purpleAccent;
      default:
        return Colors.grey;
    }
  }
}

class _MeshEventLog extends ConsumerStatefulWidget {
  final GlobalKey<AnimatedListState> listKey;
  final List<CentrifugeMessage> liveMessages;

  const _MeshEventLog({required this.listKey, required this.liveMessages});

  @override
  ConsumerState<_MeshEventLog> createState() => _MeshEventLogState();
}

class _MeshEventLogState extends ConsumerState<_MeshEventLog> {
  StreamSubscription<CentrifugeMessage>? _subscription;

  @override
  void initState() {
    super.initState();
    _subscribeToMesh();
  }

  void _subscribeToMesh() {
    final centrifuge = ref.read(centrifugeServiceProvider);
    if (centrifuge != null) {
      _subscription = centrifuge.subscribe('mesh:tasks').listen((msg) {
        if (!widget.liveMessages.any((m) => m.id == msg.id)) {
          widget.liveMessages.insert(0, msg);
          widget.listKey.currentState?.insertItem(0, duration: const Duration(milliseconds: 500));
        }
      });
    }
  }

  @override
  void dispose() {
    _subscription?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final centrifuge = ref.watch(centrifugeServiceProvider);

    if (centrifuge == null) {
      return const Center(child: Text('Connecting to Teammate Mesh...'));
    }

    // Ensure subscription exists if centrifuge becomes available after init
    if (_subscription == null) {
      _subscribeToMesh();
    }

    return AnimatedList(
      key: widget.listKey,
      initialItemCount: widget.liveMessages.length,
      itemBuilder: (context, index, animation) {
        final msg = widget.liveMessages[index];
        return SlideTransition(
          position: animation.drive(Tween(begin: const Offset(-1, 0), end: Offset.zero).chain(CurveTween(curve: Curves.easeOutQuart))),
          child: FadeTransition(
            opacity: animation,
            child: Padding(
              padding: const EdgeInsets.only(bottom: 12.0),
              child: _GlassMessageCard(message: msg),
            ),
          ),
        );
      },
    );
  }
}

class _GlassMessageCard extends StatelessWidget {
  final CentrifugeMessage message;

  const _GlassMessageCard({required this.message});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return GlassCard(
      padding: const EdgeInsets.all(16),
      color: colors.surfaceContainerHighest.withValues(alpha: 0.2),
      child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Container(
                width: 40,
                height: 40,
                decoration: BoxDecoration(
                  color: colors.primary.withValues(alpha: 0.2),
                  shape: BoxShape.circle,
                ),
                child: Icon(Icons.memory, color: colors.primary),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      message.authorName,
                      style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, color: colors.primary, fontSize: 16),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      message.body,
                      style: const TextStyle(fontFamily: 'Inter', fontSize: 14),
                    ),
                  ],
                ),
              ),
            ],
          ),
    );
  }
}
