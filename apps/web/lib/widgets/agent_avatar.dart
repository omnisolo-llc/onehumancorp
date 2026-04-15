import 'dart:ui';
import 'package:flutter/material.dart';

class AgentAvatar extends StatefulWidget {
  final String agentName;
  final bool isOnline;
  final bool isWorking;

  const AgentAvatar({
    Key? key,
    required this.agentName,
    this.isOnline = true,
    this.isWorking = false,
  }) : super(key: key);

  @override
  State<AgentAvatar> createState() => _AgentAvatarState();
}

class _AgentAvatarState extends State<AgentAvatar> with SingleTickerProviderStateMixin {
  late AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 2),
    );
    if (widget.isWorking) {
      _controller.repeat(reverse: true);
    }
  }

  @override
  void didUpdateWidget(AgentAvatar oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.isWorking != oldWidget.isWorking) {
      if (widget.isWorking) {
        _controller.repeat(reverse: true);
      } else {
        _controller.stop();
        _controller.reset();
      }
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final Color statusColor = widget.isOnline
        ? (widget.isWorking ? Colors.blueAccent : Colors.green)
        : Colors.grey;

    final String initial = widget.agentName.isNotEmpty ? widget.agentName[0].toUpperCase() : '?';

    return Stack(
      alignment: Alignment.center,
      children: [
        if (widget.isWorking)
          AnimatedBuilder(
            animation: _controller,
            builder: (context, child) {
              return Container(
                width: 56 + (_controller.value * 8),
                height: 56 + (_controller.value * 8),
                decoration: BoxDecoration(
                  shape: BoxShape.circle,
                  color: statusColor.withOpacity(0.2 * (1 - _controller.value)),
                ),
              );
            },
          ),
        ClipOval(
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            child: Container(
              width: 48,
              height: 48,
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                color: const Color.fromRGBO(255, 255, 255, 0.05),
                border: Border.all(color: const Color.fromRGBO(255, 255, 255, 0.1)),
              ),
              child: Center(
                child: Text(
                  initial,
                  style: const TextStyle(
                    fontFamily: 'Outfit',
                    color: Colors.white,
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
            ),
          ),
        ),
        Positioned(
          bottom: 2,
          right: 2,
          child: Container(
            width: 12,
            height: 12,
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              color: statusColor,
              border: Border.all(color: Colors.black, width: 2),
            ),
          ),
        ),
      ],
    );
  }
}
