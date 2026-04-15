import 'dart:ui';
import 'package:flutter/material.dart';

class AgentTaskProgressWidget extends StatefulWidget {
  final String taskName;
  final double progress;
  final bool isWorking;

  const AgentTaskProgressWidget({
    Key? key,
    required this.taskName,
    required this.progress,
    this.isWorking = false,
  }) : super(key: key);

  @override
  State<AgentTaskProgressWidget> createState() => _AgentTaskProgressWidgetState();
}

class _AgentTaskProgressWidgetState extends State<AgentTaskProgressWidget> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _glowAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 1),
    );
    _glowAnimation = Tween<double>(begin: 0.03, end: 0.15).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeInOut),
    );

    if (widget.isWorking) {
      _controller.repeat(reverse: true);
    }
  }

  @override
  void didUpdateWidget(covariant AgentTaskProgressWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.isWorking != oldWidget.isWorking) {
      if (widget.isWorking) {
        _controller.repeat(reverse: true);
      } else {
        _controller.stop();
        _controller.value = 0.0;
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
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: AnimatedBuilder(
          animation: _glowAnimation,
          builder: (context, child) {
            return Container(
              padding: const EdgeInsets.all(16.0),
              decoration: BoxDecoration(
                color: Color.fromRGBO(255, 255, 255, _glowAnimation.value),
                borderRadius: BorderRadius.circular(16),
                border: Border.all(color: const Color.fromRGBO(255, 255, 255, 0.1)),
              ),
              child: child,
            );
          },
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                widget.taskName,
                style: const TextStyle(
                  color: Colors.white,
                  fontFamily: 'Outfit',
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(height: 12),
              ClipRRect(
                borderRadius: BorderRadius.circular(8),
                child: LinearProgressIndicator(
                  value: widget.progress,
                  backgroundColor: const Color.fromRGBO(255, 255, 255, 0.1),
                  valueColor: const AlwaysStoppedAnimation<Color>(Colors.greenAccent),
                  minHeight: 8,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
