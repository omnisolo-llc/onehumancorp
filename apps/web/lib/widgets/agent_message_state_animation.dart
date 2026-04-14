import 'dart:ui';
import 'package:flutter/material.dart';

enum MessageState { idle, thinking, sending, delivered }

class AgentMessageStateAnimation extends StatefulWidget {
  final MessageState state;
  final Widget child;

  const AgentMessageStateAnimation({
    Key? key,
    required this.state,
    required this.child,
  }) : super(key: key);

  @override
  State<AgentMessageStateAnimation> createState() =>
      _AgentMessageStateAnimationState();
}

class _AgentMessageStateAnimationState
    extends State<AgentMessageStateAnimation>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _scaleAnimation;
  late Animation<double> _opacityAnimation;
  late Animation<Offset> _slideAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 600),
    );

    _scaleAnimation = TweenSequence<double>([
      TweenSequenceItem(tween: Tween(begin: 1.0, end: 1.05), weight: 50),
      TweenSequenceItem(tween: Tween(begin: 1.05, end: 1.0), weight: 50),
    ]).animate(CurvedAnimation(parent: _controller, curve: Curves.easeInOut));

    _opacityAnimation = Tween<double>(begin: 0.5, end: 1.0).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeInOut),
    );

    _slideAnimation = Tween<Offset>(
      begin: const Offset(0, 0.1),
      end: Offset.zero,
    ).animate(CurvedAnimation(parent: _controller, curve: Curves.easeOutCubic));

    _updateAnimationState();
  }

  @override
  void didUpdateWidget(AgentMessageStateAnimation oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.state != widget.state) {
      _updateAnimationState();
    }
  }

  void _updateAnimationState() {
    _controller.stop();
    switch (widget.state) {
      case MessageState.idle:
        _controller.value = 1.0;
        break;
      case MessageState.thinking:
        _controller.repeat(reverse: true);
        break;
      case MessageState.sending:
        _controller.reset();
        _controller.forward();
        break;
      case MessageState.delivered:
        _controller.value = 1.0;
        break;
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
      animation: _controller,
      builder: (context, child) {
        // Evaluate dynamic values for static tree
        final bool isThinking = widget.state == MessageState.thinking;
        final bool isSending = widget.state == MessageState.sending;

        final double currentScale = isThinking ? _scaleAnimation.value : 1.0;
        final double currentOpacity = isThinking || isSending ? _opacityAnimation.value : 1.0;
        final Offset currentSlide = isSending ? _slideAnimation.value : Offset.zero;

        return Transform.scale(
          scale: currentScale,
          child: Container(
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(12),
              color: const Color.fromRGBO(255, 255, 255, 0.05),
              border: Border.all(color: const Color.fromRGBO(255, 255, 255, 0.1)),
            ),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(12),
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                child: Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: SlideTransition(
                    position: AlwaysStoppedAnimation(currentSlide),
                    child: Opacity(
                      opacity: currentOpacity,
                      child: child,
                    ),
                  ),
                ),
              ),
            ),
          ),
        );
      },
      child: widget.child,
    );
  }
}
