import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/centrifuge_service.dart';
import 'package:ohc_app/services/powersync_service.dart';
import 'package:powersync/powersync.dart' hide Column, Row;
// import 'package:sqlite3/sqlite3.dart' as sqlite;

class SwarmMemoryScreen extends ConsumerStatefulWidget {
  const SwarmMemoryScreen({super.key});

  @override
  ConsumerState<SwarmMemoryScreen> createState() => _SwarmMemoryScreenState();
}

class _SwarmMemoryScreenState extends ConsumerState<SwarmMemoryScreen> {
  final GlobalKey<AnimatedListState> _listKey = GlobalKey<AnimatedListState>();
  final List<CentrifugeMessage> _liveMessages = [];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Swarm Memory Mesh', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: Row(
        children: [
          // Left Side: Realtime Teammate Mesh via Centrifuge
          Expanded(
            flex: 1,
            child: Padding(
              padding: const EdgeInsets.all(24.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('Live Mesh Activity', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
                  const SizedBox(height: 8),
                  const Text('Real-time websocket feed from the Teammate Mesh', style: TextStyle(fontFamily: 'Inter', color: Colors.grey)),
                  const SizedBox(height: 16),
                  Expanded(
                    child: _LiveMeshWidget(
                      listKey: _listKey,
                      liveMessages: _liveMessages,
                    ),
                  ),
                ],
              ),
            ),
          ),

          const VerticalDivider(width: 1),

          // Right Side: Offline-to-Cloud State Sync for Swarm Memories via PowerSync
          Expanded(
            flex: 1,
            child: Padding(
              padding: const EdgeInsets.all(24.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('Durable Swarm Memory', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
                  const SizedBox(height: 8),
                  const Text('Offline-to-Cloud State Sync (PowerSync)', style: TextStyle(fontFamily: 'Inter', color: Colors.grey)),
                  const SizedBox(height: 16),
                  const Expanded(
                    child: _DurableMemoryWidget(),
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

class _LiveMeshWidget extends ConsumerStatefulWidget {
  final GlobalKey<AnimatedListState> listKey;
  final List<CentrifugeMessage> liveMessages;

  const _LiveMeshWidget({required this.listKey, required this.liveMessages});

  @override
  ConsumerState<_LiveMeshWidget> createState() => _LiveMeshWidgetState();
}

class _LiveMeshWidgetState extends ConsumerState<_LiveMeshWidget> {
  @override
  Widget build(BuildContext context) {
    final centrifuge = ref.watch(centrifugeServiceProvider);

    if (centrifuge == null) {
      return const Center(child: Text('Connecting to Teammate Mesh...'));
    }

    return StreamBuilder<CentrifugeMessage>(
      stream: centrifuge.subscribe('mesh:tasks'), // listening to 'mesh:tasks'
      builder: (context, snapshot) {
        if (snapshot.hasData) {
          final msg = snapshot.data!;
          // We only insert if it's not already there
          if (!widget.liveMessages.any((m) => m.id == msg.id)) {
            widget.liveMessages.insert(0, msg);
            widget.listKey.currentState?.insertItem(0, duration: const Duration(milliseconds: 500));
          }
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
                  child: _AnimatedGlassMessageCard(message: msg),
                ),
              ),
            );
          },
        );
      },
    );
  }
}

class _AnimatedGlassMessageCard extends StatefulWidget {
  final CentrifugeMessage message;

  const _AnimatedGlassMessageCard({required this.message});

  @override
  State<_AnimatedGlassMessageCard> createState() => _AnimatedGlassMessageCardState();
}

class _AnimatedGlassMessageCardState extends State<_AnimatedGlassMessageCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOutCubic,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(16),
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: ColorFilter.matrix(const <double>[
                1.168, -0.153, -0.015, 0, 0,
                -0.046, 1.061, -0.015, 0, 0,
                -0.046, -0.152, 1.198, 0, 0,
                0, 0, 0, 1, 0,
              ]),
              inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            ),
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 300),
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: _isHovered
                    ? colors.surfaceContainerHighest.withValues(alpha: 0.3)
                    : colors.surfaceContainerHighest.withValues(alpha: 0.2),
                borderRadius: BorderRadius.circular(16),
                border: Border.all(
                  color: _isHovered
                      ? colors.outlineVariant.withValues(alpha: 0.8)
                      : colors.outlineVariant.withValues(alpha: 0.4),
                ),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withValues(alpha: 0.05),
                    blurRadius: 10,
                    offset: const Offset(0, 4),
                  ),
                ],
              ),
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
                          widget.message.authorName,
                          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, color: colors.primary, fontSize: 16),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          widget.message.body,
                          style: const TextStyle(fontFamily: 'Inter', fontSize: 14),
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

class _DurableMemoryWidget extends ConsumerWidget {
  const _DurableMemoryWidget();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final db = ref.watch(powersyncProvider).db;

    if (db == null) {
      return const Center(child: CircularProgressIndicator());
    }

    return StreamBuilder<List<dynamic>>(
      stream: db.watch('SELECT * FROM swarm_memory ORDER BY updated_at DESC LIMIT 50'),
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return const Center(child: CircularProgressIndicator());
        }

        if (snapshot.hasError) {
          return Center(child: Text('Error: ${snapshot.error}'));
        }

        final rows = snapshot.data ?? [];

        if (rows.isEmpty) {
          return const Center(child: Text('No memories found.', style: TextStyle(fontFamily: 'Inter')));
        }

        return ListView.builder(
          itemCount: rows.length,
          itemBuilder: (context, index) {
            final row = rows[index];
            final value = (row is Map) ? row['value'] as String? : (row as dynamic).read('value') as String?;
            final updatedAt = (row is Map) ? row['updated_at'] as String? : (row as dynamic).read('updated_at') as String?;

            return Padding(
              padding: const EdgeInsets.only(bottom: 12.0),
              child: _AnimatedMemoryCard(value: value ?? '', updatedAt: updatedAt ?? ''),
            );
          },
        );
      },
    );
  }
}

class _AnimatedMemoryCard extends StatefulWidget {
  final String value;
  final String updatedAt;

  const _AnimatedMemoryCard({required this.value, required this.updatedAt});

  @override
  State<_AnimatedMemoryCard> createState() => _AnimatedMemoryCardState();
}

class _AnimatedMemoryCardState extends State<_AnimatedMemoryCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOutCubic,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(16),
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: ColorFilter.matrix(const <double>[
                1.168, -0.153, -0.015, 0, 0,
                -0.046, 1.061, -0.015, 0, 0,
                -0.046, -0.152, 1.198, 0, 0,
                0, 0, 0, 1, 0,
              ]),
              inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            ),
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 300),
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: _isHovered
                    ? colors.secondaryContainer.withValues(alpha: 0.15)
                    : colors.secondaryContainer.withValues(alpha: 0.1),
                borderRadius: BorderRadius.circular(16),
                border: Border.all(
                  color: _isHovered
                      ? colors.outlineVariant.withValues(alpha: 0.6)
                      : colors.outlineVariant.withValues(alpha: 0.3),
                ),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    widget.value,
                    style: const TextStyle(fontFamily: 'Inter', fontSize: 14),
                  ),
                  const SizedBox(height: 8),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.end,
                    children: [
                      Icon(Icons.cloud_sync, size: 14, color: colors.secondary),
                      const SizedBox(width: 4),
                      Text(
                        widget.updatedAt,
                        style: TextStyle(fontFamily: 'Inter', fontSize: 12, color: colors.onSurfaceVariant),
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
  }
}
