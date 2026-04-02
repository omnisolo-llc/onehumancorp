import 'dart:async';
import 'dart:math';
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

// Simulate Teammate Mesh messages from Redis/WebSockets
class MeshMessage {
  final String agentName;
  final String action;
  final DateTime timestamp;

  MeshMessage(this.agentName, this.action, this.timestamp);
}

final meshStreamProvider = StreamProvider.autoDispose<MeshMessage>((ref) {
  final controller = StreamController<MeshMessage>();
  final random = Random();
  final agents = ['Bolt-L7', 'Palette-L7', 'Architect-L8', 'Nexus-L6'];
  final actions = [
    'Synced Postgres vector memory',
    'Audited UI responsiveness',
    'Scaled Redis pub/sub queue',
    'Generated new visual tokens',
    'Analyzed market telemetry',
  ];

  Timer? timer;
  ref.onDispose(() {
    timer?.cancel();
    controller.close();
  });

  timer = Timer.periodic(const Duration(seconds: 2), (t) {
    if (!controller.isClosed) {
      controller.add(
        MeshMessage(
          agents[random.nextInt(agents.length)],
          actions[random.nextInt(actions.length)],
          DateTime.now(),
        ),
      );
    }
  });

  return controller.stream;
});

class SwarmObservabilityWidget extends ConsumerStatefulWidget {
  const SwarmObservabilityWidget({super.key});

  @override
  ConsumerState<SwarmObservabilityWidget> createState() => _SwarmObservabilityWidgetState();
}

class _SwarmObservabilityWidgetState extends ConsumerState<SwarmObservabilityWidget> {
  final List<MeshMessage> _messages = [];
  final ScrollController _scrollController = ScrollController();

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    ref.listen<AsyncValue<MeshMessage>>(meshStreamProvider, (previous, next) {
      if (next.hasValue && next.value != null) {
        setState(() {
          _messages.insert(0, next.value!);
          if (_messages.length > 50) {
            _messages.removeLast();
          }
        });
      }
    });

    return Semantics(
      label: 'Swarm Observability Dashboard',
      child: ClipRRect(
        borderRadius: BorderRadius.circular(24),
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
          child: Container(
            height: 350,
            decoration: BoxDecoration(
              color: Colors.white.withValues(alpha: 0.03),
              borderRadius: BorderRadius.circular(24),
              border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
            ),
            child: Padding(
              padding: const EdgeInsets.all(24.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Container(
                        padding: const EdgeInsets.all(8),
                        decoration: BoxDecoration(
                          color: colors.primary.withValues(alpha: 0.2),
                          shape: BoxShape.circle,
                        ),
                        child: Icon(Icons.wifi_tethering, color: colors.primary, size: 24),
                      ),
                      const SizedBox(width: 12),
                      const Text(
                        'Teammate Mesh Live Feed',
                        style: TextStyle(
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                          fontFamily: 'Outfit',
                          color: Colors.white,
                        ),
                      ),
                      const Spacer(),
                      _PulsingStatusIndicator(),
                    ],
                  ),
                  const SizedBox(height: 16),
                  Expanded(
                    child: _messages.isEmpty
                        ? Center(
                            child: Text(
                              'Listening for swarm activity...',
                              style: TextStyle(
                                color: Colors.white.withValues(alpha: 0.5),
                                fontFamily: 'Inter',
                              ),
                            ),
                          )
                        : ListView.builder(
                            controller: _scrollController,
                            itemCount: _messages.length,
                            itemBuilder: (context, index) {
                              final msg = _messages[index];
                              return _AnimatedMessageItem(
                                key: ValueKey(msg.timestamp.millisecondsSinceEpoch),
                                message: msg,
                              );
                            },
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

class _PulsingStatusIndicator extends StatefulWidget {
  @override
  State<_PulsingStatusIndicator> createState() => _PulsingStatusIndicatorState();
}

class _PulsingStatusIndicatorState extends State<_PulsingStatusIndicator> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _opacityAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 1),
    )..repeat(reverse: true);
    _opacityAnimation = Tween<double>(begin: 0.3, end: 1.0).animate(_controller);
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        FadeTransition(
          opacity: _opacityAnimation,
          child: Container(
            width: 10,
            height: 10,
            decoration: const BoxDecoration(
              color: Colors.greenAccent,
              shape: BoxShape.circle,
            ),
          ),
        ),
        const SizedBox(width: 8),
        const Text(
          'Live',
          style: TextStyle(
            color: Colors.greenAccent,
            fontWeight: FontWeight.bold,
            fontFamily: 'Inter',
          ),
        ),
      ],
    );
  }
}

class _AnimatedMessageItem extends StatefulWidget {
  final MeshMessage message;

  const _AnimatedMessageItem({super.key, required this.message});

  @override
  State<_AnimatedMessageItem> createState() => _AnimatedMessageItemState();
}

class _AnimatedMessageItemState extends State<_AnimatedMessageItem> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<Offset> _slideAnimation;
  late Animation<double> _fadeAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 400),
    );
    _slideAnimation = Tween<Offset>(
      begin: const Offset(0, 0.5),
      end: Offset.zero,
    ).animate(CurvedAnimation(parent: _controller, curve: Curves.easeOutCubic));
    _fadeAnimation = Tween<double>(begin: 0.0, end: 1.0).animate(_controller);
    _controller.forward();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final timeStr =
        '${widget.message.timestamp.hour.toString().padLeft(2, '0')}:${widget.message.timestamp.minute.toString().padLeft(2, '0')}:${widget.message.timestamp.second.toString().padLeft(2, '0')}';
    return SlideTransition(
      position: _slideAnimation,
      child: FadeTransition(
        opacity: _fadeAnimation,
        child: Container(
          margin: const EdgeInsets.only(bottom: 12),
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
          decoration: BoxDecoration(
            color: Colors.white.withValues(alpha: 0.05),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(color: Colors.white.withValues(alpha: 0.05)),
          ),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                timeStr,
                style: TextStyle(
                  color: Colors.white.withValues(alpha: 0.4),
                  fontSize: 12,
                  fontFamily: 'monospace',
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      widget.message.agentName,
                      style: const TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.bold,
                        fontFamily: 'Outfit',
                      ),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      widget.message.action,
                      style: TextStyle(
                        color: Colors.white.withValues(alpha: 0.8),
                        fontFamily: 'Inter',
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
