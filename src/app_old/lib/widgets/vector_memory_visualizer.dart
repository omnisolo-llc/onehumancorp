import 'dart:ui';
import 'package:flutter/material.dart';

class VectorMemoryVisualizerWidget extends StatefulWidget {
  const VectorMemoryVisualizerWidget({Key? key}) : super(key: key);

  @override
  State<VectorMemoryVisualizerWidget> createState() => _VectorMemoryVisualizerWidgetState();
}

class _VectorMemoryVisualizerWidgetState extends State<VectorMemoryVisualizerWidget> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _pulseAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1500),
    )..repeat(reverse: true);

    _pulseAnimation = Tween<double>(begin: 0.2, end: 0.8).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeInOut),
    );
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
        filter: ImageFilter.compose(
          outer: const ColorFilter.matrix(<double>[
            1.168, -0.153, -0.015, 0, 0,
            -0.046, 1.061, -0.015, 0, 0,
            -0.046, -0.152, 1.198, 0, 0,
            0, 0, 0, 1, 0,
          ]),
          inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        ),
        child: Container(
          padding: const EdgeInsets.all(24.0),
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            borderRadius: BorderRadius.circular(16),
            border: Border.all(color: const Color.fromRGBO(255, 255, 255, 0.1)),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              const Text(
                'AutoDream Consolidation',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 16),
              SizedBox(
                height: 100,
                child: AnimatedBuilder(
                  animation: _pulseAnimation,
                  builder: (context, child) {
                    return Stack(
                      alignment: Alignment.center,
                      children: [
                        // Background layer (large, slow moving)
                        Transform.translate(
                          offset: Offset(0, -10 + (_pulseAnimation.value * 20)),
                          child: _buildVectorGrid(context, 12, 0.2, Colors.purpleAccent, 8.0),
                        ),
                        // Middle layer (medium speed)
                        Transform.translate(
                          offset: Offset(-5 + (_pulseAnimation.value * 10), 5 - (_pulseAnimation.value * 10)),
                          child: _buildVectorGrid(context, 8, 0.4, Colors.blueAccent, 6.0),
                        ),
                        // Foreground layer (fast moving, bright)
                        Transform.translate(
                          offset: Offset(10 - (_pulseAnimation.value * 20), 10 - (_pulseAnimation.value * 20)),
                          child: _buildVectorGrid(context, 5, 0.8, Colors.cyanAccent, 4.0),
                        ),
                      ],
                    );
                  },
                ),
              ),
              const SizedBox(height: 16),
              const Text(
                'pgvector dimension: 1536',
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 12,
                  color: Colors.white54,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildVectorGrid(BuildContext context, int count, double baseOpacity, Color color, double baseSize) {
    return Wrap(
      spacing: 8,
      runSpacing: 8,
      alignment: WrapAlignment.center,
      children: List.generate(count, (index) {
        final opacity = (baseOpacity + ((index % 3) * 0.1)).clamp(0.0, 1.0);
        return Container(
          width: baseSize + (index % 4),
          height: baseSize + (index % 4),
          decoration: BoxDecoration(
            color: color.withOpacity(opacity),
            shape: BoxShape.circle,
            boxShadow: [
              BoxShadow(
                color: color.withOpacity(opacity * 0.8),
                blurRadius: 5 * opacity,
                spreadRadius: 2 * opacity,
              ),
            ],
          ),
        );
      }),
    );
  }
}
