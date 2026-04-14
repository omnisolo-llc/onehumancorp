import 'dart:async';
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/centrifuge_service.dart';

class SwarmObservabilityDashboard extends ConsumerStatefulWidget {
  const SwarmObservabilityDashboard({super.key});

  @override
  ConsumerState<SwarmObservabilityDashboard> createState() => _SwarmObservabilityDashboardState();
}

class _SwarmObservabilityDashboardState extends ConsumerState<SwarmObservabilityDashboard> {
  final List<dynamic> _events = [];
  final GlobalKey<AnimatedListState> _listKey = GlobalKey<AnimatedListState>();
  StreamSubscription<dynamic>? _subscription;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _initConnection();
    });
  }

  void _initConnection() async {
    final centrifugeService = ref.read(centrifugeServiceProvider);
    if (centrifugeService != null) {
      try {
        await centrifugeService.connect();
        _subscription = centrifugeService.subscribeRaw('mesh:coordination').listen((event) {
          if (mounted) {
            final wasEmpty = _events.isEmpty;
            _events.insert(0, event);
            _listKey.currentState?.insertItem(0);
            if (_events.length > 50) {
              final removedItem = _events.removeLast();
              _listKey.currentState?.removeItem(
                50,
                (context, animation) => _buildItem(removedItem, animation),
              );
            }
            if (wasEmpty) {
              setState(() {});
            }
          }
        });
      } catch (e) {
        debugPrint('Centrifuge connection error: $e');
      }
    }
  }

  @override
  void dispose() {
    _subscription?.cancel();
    super.dispose();
  }

  Widget _buildItem(dynamic event, Animation<double> animation) {
    return FadeTransition(
      opacity: animation,
      child: SlideTransition(
        position: animation.drive(Tween(begin: const Offset(-0.1, 0), end: Offset.zero)),
        child: ListTile(
          leading: const Icon(Icons.bolt, color: Colors.yellow),
          title: Text(
            event['action'] ?? 'Unknown Action',
            style: const TextStyle(color: Colors.white),
          ),
          subtitle: Text(
            'Agent: ${event['agent_id']} | Status: ${event['status']}',
            style: const TextStyle(color: Colors.white70),
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
        child: Container(
          padding: const EdgeInsets.all(20),
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(
              color: const Color.fromRGBO(255, 255, 255, 0.1),
            ),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text(
                'Swarm Intelligence Mesh',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 24,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 16),
              Expanded(
                child: AnimatedList(
                  key: _listKey,
                  initialItemCount: _events.length,
                  itemBuilder: (context, index, animation) {
                    return _buildItem(_events[index], animation);
                  },
                ),
              ),
              if (_events.isEmpty)
                const Padding(
                  padding: EdgeInsets.only(top: 8.0),
                  child: Text(
                    'Monitoring agent swarm...',
                    style: TextStyle(
                      fontFamily: 'Inter',
                      fontSize: 16,
                      color: Colors.white70,
                    ),
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }
}
