import 'dart:ui';
import 'dart:math' as math;
import 'package:flutter/material.dart';

class VectorMemoryVisualizerWidget extends StatefulWidget {
  final List<double> vectorState;
  final bool isPulsing;

  const VectorMemoryVisualizerWidget({
    Key? key,
    required this.vectorState,
    this.isPulsing = false,
  }) : super(key: key);

  @override
  _VectorMemoryVisualizerWidgetState createState() => _VectorMemoryVisualizerWidgetState();
}

class _VectorMemoryVisualizerWidgetState extends State<VectorMemoryVisualizerWidget> with SingleTickerProviderStateMixin {
  late AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 10),
    )..repeat();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.black.withOpacity(0.5),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: widget.isPulsing ? Colors.blue.withOpacity(0.8) : Colors.white.withOpacity(0.2),
          width: widget.isPulsing ? 2.0 : 1.0,
        ),
        boxShadow: widget.isPulsing
            ? [
                BoxShadow(
                  color: Colors.blue.withOpacity(0.5),
                  blurRadius: 20,
                  spreadRadius: 5,
                )
              ]
            : [],
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(16),
        child: BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 10, sigmaY: 10),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                'Vector Memory State (1536-dim)',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 16),
              SizedBox(
                height: 200,
                width: double.infinity,
                child: AnimatedBuilder(
                  animation: _controller,
                  builder: (context, child) {
                    return CustomPaint(
                      painter: ParallaxVectorPainter(
                        vectorState: widget.vectorState,
                        animationValue: _controller.value,
                        isPulsing: widget.isPulsing,
                      ),
                    );
                  },
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class ParallaxVectorPainter extends CustomPainter {
  final List<double> vectorState;
  final double animationValue;
  final bool isPulsing;

  ParallaxVectorPainter({
    required this.vectorState,
    required this.animationValue,
    required this.isPulsing,
  });

  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = Colors.blue.withOpacity(0.5)
      ..style = PaintingStyle.fill;

    final double dotRadius = isPulsing ? 3.0 : 2.0;

    for (int i = 0; i < vectorState.length; i++) {
      final value = vectorState[i];
      // Create a multi-layered parallax effect based on vector values
      final double x = (i % 40) * (size.width / 40.0);
      final double y = (i ~/ 40) * (size.height / (vectorState.length / 40.0));

      final offset = Offset(
        x + math.sin(animationValue * 2 * math.pi + value) * 10,
        y + math.cos(animationValue * 2 * math.pi + value) * 10,
      );

      final colorOpacity = (value.abs() * 0.8 + 0.2).clamp(0.0, 1.0);
      paint.color = Colors.blueAccent.withOpacity(colorOpacity);

      canvas.drawCircle(offset, dotRadius, paint);
    }
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => true;
}
