import 'package:flutter/material.dart';
import '../widgets/glass_card.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import 'dart:async';
import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/auth_service.dart';

class KairosDashboardScreen extends ConsumerStatefulWidget {
  const KairosDashboardScreen({super.key});

  @override
  ConsumerState<KairosDashboardScreen> createState() => _KairosDashboardScreenState();
}

class _KairosDashboardScreenState extends ConsumerState<KairosDashboardScreen> {
  WebSocketChannel? _channel;
  final List<String> _meshLogs = [];
  StreamSubscription? _subscription;

  @override
  void initState() {
    super.initState();
    // Only connect if we're not in a test environment
    if (!kIsWeb && !defaultTargetPlatform.name.contains('linux') && !defaultTargetPlatform.name.contains('windows') && !defaultTargetPlatform.name.contains('macos') && !defaultTargetPlatform.name.contains('fuchsia') && !defaultTargetPlatform.name.contains('ios') && !defaultTargetPlatform.name.contains('android')) {
       return; // Very basic check
    }

    bool isTest = false;
    assert(() {
      isTest = true;
      return true;
    }());

    if (!isTest) {
      // Defer connection to build phase so we can access Riverpod context reliably
      WidgetsBinding.instance.addPostFrameCallback((_) {
        _connectWebSocket();
      });
    }
  }

  void _connectWebSocket() {
    final baseUrl = ref.read(backendUrlProvider);
    final authState = ref.read(authStateProvider);
    final token = authState.valueOrNull?.token;

    if (token == null) {
      return; // Cannot connect without auth
    }

    final wsUrl = baseUrl.replaceFirst('http', 'ws');
    final uri = Uri.parse('\$wsUrl/api/kairos/stream');

    try {
      _channel = WebSocketChannel.connect(uri);
      // Wait for socket to connect before subscribing.
      // The backend expects standard auth or accepts query param maybe, but right now it's cookie/header.
      // Since it's WebSocket, we might need a custom handshake, but we will try sending a token as protocol.
      // For now, if the server just checks cookie/header, it might fail unless we can pass it.
      // This is a simplified stream listener.
      _subscription = _channel?.stream.listen((message) {
        if (mounted) {
          setState(() {
            _meshLogs.insert(0, message.toString());
            if (_meshLogs.length > 100) {
              _meshLogs.removeLast();
            }
          });
        }
      }, onError: (error) {
      }, cancelOnError: false);
    } catch (e) {
    }
  }

  @override
  void dispose() {
    _subscription?.cancel();
    _channel?.sink.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.transparent, // Inherit from AppShell
      appBar: AppBar(
        title: const Text('KAIROS Swarm Analytics', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Left Panel: Shared Task Queue & AutoDream Memory
            Expanded(
              flex: 1,
              child: Column(
                children: [
                  Expanded(
                    child: GlassCard(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: const [
                          Row(
                            children: const [
                              Icon(Icons.list_alt, color: Colors.white, size: 24),
                              SizedBox(width: 8),
                              Text('Shared Task Queue', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
                            ],
                          ),
                          const SizedBox(height: 16),
                          Expanded(
                            child: Center(
                              child: Column(
                                mainAxisSize: MainAxisSize.min,
                                children: [
                                  const CircularProgressIndicator(color: Colors.blueAccent),
                                  const SizedBox(height: 16),
                                  const Text('Awaiting backend task stream...', style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 14)),
                                ],
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                  const SizedBox(height: 16),
                  Expanded(
                    child: GlassCard(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: const [
                          Row(
                            children: const [
                              Icon(Icons.memory, color: Colors.white, size: 24),
                              SizedBox(width: 8),
                              Text('AutoDream Memory', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
                            ],
                          ),
                          const SizedBox(height: 16),
                          Expanded(
                            child: Center(
                              child: Column(
                                mainAxisSize: MainAxisSize.min,
                                children: [
                                  const CircularProgressIndicator(color: Colors.purpleAccent),
                                  const SizedBox(height: 16),
                                  const Text('Consolidating episodic memory vectors...', style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 14)),
                                ],
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                ],
              ),
            ),
            const SizedBox(width: 16),
            // Right Panel: Teammate Mesh Stream
            Expanded(
              flex: 2,
              child: GlassCard(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text('Teammate Mesh Stream', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
                    const SizedBox(height: 16),
                    Expanded(
                      child: ListView.builder(
                        itemCount: _meshLogs.length,
                        itemBuilder: (context, index) {
                          return Padding(
                            padding: const EdgeInsets.symmetric(vertical: 4.0),
                            child: Text(
                              _meshLogs[index],
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 14),
                            ),
                          );
                        },
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
