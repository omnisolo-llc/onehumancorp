import 'dart:async';
import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/centrifuge_service.dart';

final _meshLogsProvider = StateProvider<List<CentrifugeMessage>>(
  (ref) => const [],
);

class SwarmMeshScreen extends ConsumerStatefulWidget {
  const SwarmMeshScreen({super.key});

  @override
  ConsumerState<SwarmMeshScreen> createState() => _SwarmMeshScreenState();
}

class _SwarmMeshScreenState extends ConsumerState<SwarmMeshScreen> {
  StreamSubscription<CentrifugeMessage>? _sub;
  CentrifugeService? _service;
  final _scrollCtrl = ScrollController();

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _connect());
  }

  Future<void> _connect() async {
    final svc = ref.read(centrifugeServiceProvider);
    if (svc == null) return;
    _service = svc;
    try {
      await svc.connect();
      // Subscribe to the shared tasks and locks channels for observability
      _sub = svc.subscribe('swarm:tasks:updates').listen(_onMessage);
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              'Swarm Mesh connection failed: $e',
              style: const TextStyle(fontFamily: 'Inter'),
            ),
          ),
        );
      }
    }
  }

  void _onMessage(CentrifugeMessage msg) {
    ref.read(_meshLogsProvider.notifier).update((msgs) => [...msgs, msg]);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (_scrollCtrl.hasClients) {
        _scrollCtrl.animateTo(
          _scrollCtrl.position.maxScrollExtent,
          duration: const Duration(milliseconds: 200),
          curve: Curves.easeOut,
        );
      }
    });
  }

  @override
  void dispose() {
    _sub?.cancel();
    _service?.disconnect();
    _scrollCtrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final logs = ref.watch(_meshLogsProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'Swarm Observability Mesh',
          style: TextStyle(fontFamily: 'Outfit'),
        ),
      ),
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(32.0),
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 800),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(12),
              child: BackdropFilter(
                filter: ImageFilter.compose(
                  outer: ColorFilter.matrix(<double>[
                    2.0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    2.0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    2.0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    1,
                    0,
                  ]), // saturate(200%) approximation via matrix
                  inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                ),
                child: Container(
                  decoration: BoxDecoration(
                    color: const Color.fromRGBO(255, 255, 255, 0.03),
                    borderRadius: BorderRadius.circular(12),
                    border: Border.all(
                      color: const Color.fromRGBO(255, 255, 255, 0.08),
                    ),
                  ),
                  child: Column(
                    children: [
                      Padding(
                        padding: const EdgeInsets.all(24.0),
                        child: Row(
                          children: [
                            const Icon(
                              Icons.hub,
                              color: Colors.white,
                              size: 28,
                            ),
                            const SizedBox(width: 12),
                            const Expanded(
                              child: Text(
                                'Realtime Teammate Mesh',
                                style: TextStyle(
                                  fontFamily: 'Outfit',
                                  color: Colors.white,
                                  fontSize: 24,
                                  fontWeight: FontWeight.bold,
                                ),
                                overflow: TextOverflow.ellipsis,
                              ),
                            ),
                            Container(
                              padding: const EdgeInsets.symmetric(
                                horizontal: 12,
                                vertical: 6,
                              ),
                              decoration: BoxDecoration(
                                color: Colors.green.withValues(alpha: 0.2),
                                borderRadius: BorderRadius.circular(20),
                                border: Border.all(
                                  color: Colors.green.withValues(alpha: 0.5),
                                ),
                              ),
                              child: const Row(
                                children: [
                                  Icon(
                                    Icons.circle,
                                    color: Colors.green,
                                    size: 12,
                                  ),
                                  SizedBox(width: 6),
                                  Text(
                                    'Connected',
                                    style: TextStyle(
                                      fontFamily: 'Inter',
                                      color: Colors.white,
                                      fontSize: 12,
                                      fontWeight: FontWeight.bold,
                                    ),
                                  ),
                                ],
                              ),
                            ),
                          ],
                        ),
                      ),
                      const Divider(
                        height: 1,
                        color: Color.fromRGBO(255, 255, 255, 0.08),
                      ),
                      Expanded(
                        child: logs.isEmpty
                            ? const Center(
                                child: Text(
                                  'Awaiting swarm activity...',
                                  style: TextStyle(
                                    fontFamily: 'Inter',
                                    color: Colors.white54,
                                    fontSize: 16,
                                  ),
                                ),
                              )
                            : ListView.builder(
                                controller: _scrollCtrl,
                                padding: const EdgeInsets.all(16),
                                itemCount: logs.length,
                                itemBuilder: (context, index) {
                                  final msg = logs[index];
                                  return _MeshLogItem(message: msg);
                                },
                              ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _MeshLogItem extends StatelessWidget {
  final CentrifugeMessage message;

  const _MeshLogItem({required this.message});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 12.0),
      child: Container(
        padding: const EdgeInsets.all(16),
        decoration: BoxDecoration(
          color: Colors.black.withValues(alpha: 0.2),
          borderRadius: BorderRadius.circular(8),
          border: Border.all(color: Colors.white.withValues(alpha: 0.05)),
        ),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Container(
              padding: const EdgeInsets.all(8),
              decoration: BoxDecoration(
                color: Theme.of(
                  context,
                ).colorScheme.primary.withValues(alpha: 0.2),
                shape: BoxShape.circle,
              ),
              child: Icon(
                Icons.memory,
                size: 20,
                color: Theme.of(context).colorScheme.primary,
              ),
            ),
            const SizedBox(width: 16),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Text(
                        message.authorName,
                        style: const TextStyle(
                          fontFamily: 'Inter',
                          color: Colors.white,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const Spacer(),
                      Text(
                        '${message.sentAt.hour.toString().padLeft(2, '0')}:${message.sentAt.minute.toString().padLeft(2, '0')}:${message.sentAt.second.toString().padLeft(2, '0')}',
                        style: const TextStyle(
                          fontFamily: 'Inter',
                          color: Colors.white54,
                          fontSize: 12,
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 8),
                  Text(
                    message.body,
                    style: const TextStyle(
                      fontFamily: 'Inter',
                      color: Colors.white70,
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
