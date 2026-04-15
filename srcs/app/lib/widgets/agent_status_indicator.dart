import 'package:flutter/material.dart';

class AgentStatusIndicator extends StatefulWidget {
  final bool isActive;
  final double size;

  const AgentStatusIndicator({
    super.key,
    required this.isActive,
    this.size = 12.0,
  });

  @override
  State<AgentStatusIndicator> createState() => _AgentStatusIndicatorState();
}

class _AgentStatusIndicatorState extends State<AgentStatusIndicator> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _animation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1500),
    );
    _animation = Tween<double>(begin: 0.5, end: 1.0).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeInOut),
    );
    if (widget.isActive) {
      _controller.repeat(reverse: true);
    }
  }

  @override
  void didUpdateWidget(covariant AgentStatusIndicator oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.isActive != oldWidget.isActive) {
      if (widget.isActive) {
        _controller.repeat(reverse: true);
      } else {
        _controller.stop();
        _controller.value = 0.5;
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
    return AnimatedBuilder(
      animation: _animation,
      builder: (context, child) {
        return Container(
          width: widget.size,
          height: widget.size,
          decoration: BoxDecoration(
            shape: BoxShape.circle,
            color: widget.isActive ? Colors.greenAccent : Colors.grey,
            boxShadow: widget.isActive
                ? [
                    BoxShadow(
                      color: Colors.greenAccent.withOpacity(0.6 * _animation.value),
                      blurRadius: widget.size * _animation.value,
                      spreadRadius: widget.size * 0.5 * _animation.value,
                    )
                  ]
                : [],
          ),
        );
      },
    );
  }
}
