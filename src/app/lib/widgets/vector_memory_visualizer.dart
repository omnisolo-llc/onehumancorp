import 'dart:math';
import 'package:flutter/material.dart';

class VectorMemoryVisualizerWidget extends StatefulWidget {
  final List<double> vectorState;
  final bool isPulsing;

  const VectorMemoryVisualizerWidget({
    super.key,
    required this.vectorState,
    this.isPulsing = false,
  });

  @override
  State<VectorMemoryVisualizerWidget> createState() => _VectorMemoryVisualizerWidgetState();
}

class _VectorMemoryVisualizerWidgetState extends State<VectorMemoryVisualizerWidget>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _pulseAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1500),
    )..repeat(reverse: true);

    _pulseAnimation = Tween<double>(begin: 1.0, end: 1.2).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeInOut),
    );
  }

  @override
  void didUpdateWidget(VectorMemoryVisualizerWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.isPulsing && !oldWidget.isPulsing) {
      _triggerHapticPulse();
    }
  }

  void _triggerHapticPulse() {
    // In a real device we might use HapticFeedback.heavyImpact()
    // For visual pulse, we can temporarily speed up the animation
    _controller.duration = const Duration(milliseconds: 300);
    _controller.repeat(reverse: true);
    Future.delayed(const Duration(milliseconds: 900), () {
      if (mounted) {
        _controller.duration = const Duration(milliseconds: 1500);
        _controller.repeat(reverse: true);
      }
    });
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 200,
      width: double.infinity,
      decoration: BoxDecoration(
        color: Colors.black87,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: widget.isPulsing ? Colors.blue.withOpacity(0.8) : Colors.white24,
          width: widget.isPulsing ? 2.0 : 1.0,
        ),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(16),
        child: AnimatedBuilder(
          animation: _pulseAnimation,
          builder: (context, child) {
            return CustomPaint(
              painter: _VectorParallaxPainter(
                vectorState: widget.vectorState,
                pulseScale: _pulseAnimation.value,
                isPulsing: widget.isPulsing,
              ),
            );
          },
        ),
      ),
    );
  }
}

class _VectorParallaxPainter extends CustomPainter {
  final List<double> vectorState;
  final double pulseScale;
  final bool isPulsing;

  _VectorParallaxPainter({
    required this.vectorState,
    required this.pulseScale,
    required this.isPulsing,
  });

  @override
  void paint(Canvas canvas, Size size) {
    if (vectorState.isEmpty) return;

    final paint = Paint()
      ..style = PaintingStyle.fill
      ..color = Colors.blue.withOpacity(0.5);

    final highlightPaint = Paint()
      ..style = PaintingStyle.fill
      ..color = Colors.cyanAccent.withOpacity(0.8);

    final random = Random(42); // Seeded for consistent visual structure

    // Draw 3 parallax layers to represent deep dimensional state
    for (int layer = 0; layer < 3; layer++) {
      final layerScale = 1.0 + (layer * 0.2);
      final dotRadius = (2.0 + layer) * (isPulsing ? pulseScale : 1.0);

      final layerPaint = layer == 2 && isPulsing ? highlightPaint : paint;
      layerPaint.color = layerPaint.color.withOpacity(0.3 + (layer * 0.2));

      int itemsPerLayer = min(50, vectorState.length ~/ 3);
      for (int i = 0; i < itemsPerLayer; i++) {
        // use vector state to drive some visualization aspects
        final valIndex = (layer * itemsPerLayer + i) % vectorState.length;
        final val = vectorState[valIndex];

        // Map abstract 1536d values to 2D space deterministically
        final x = (random.nextDouble() * size.width);
        final y = (random.nextDouble() * size.height) + (val * 10 * layerScale);

        canvas.drawCircle(Offset(x, y), dotRadius * val.abs().clamp(0.5, 3.0), layerPaint);
      }
    }
  }

  @override
  bool shouldRepaint(covariant _VectorParallaxPainter oldDelegate) {
    return oldDelegate.pulseScale != pulseScale ||
           oldDelegate.isPulsing != isPulsing ||
           oldDelegate.vectorState != vectorState;
  }
}
