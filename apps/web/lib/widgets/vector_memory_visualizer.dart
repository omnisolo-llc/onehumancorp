import 'dart:ui';
import 'package:flutter/material.dart';

class VectorMemoryVisualizerWidget extends StatefulWidget {
  final double embeddingActivity;

  const VectorMemoryVisualizerWidget({Key? key, this.embeddingActivity = 0.5}) : super(key: key);

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
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
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
                    return Row(
                      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                      children: List.generate(5, (index) {
                        // Create a cascading effect
                        final opacity = (_pulseAnimation.value * widget.embeddingActivity + (index * 0.1)) % 1.0;
                        return Container(
                          width: 16,
                          height: 16 + (opacity * 24),
                          decoration: BoxDecoration(
                            color: Colors.cyanAccent.withOpacity(0.5 + (opacity * 0.5)),
                            borderRadius: BorderRadius.circular(8),
                            boxShadow: [
                              BoxShadow(
                                color: Colors.cyanAccent.withOpacity(opacity * 0.8),
                                blurRadius: 10 * opacity,
                                spreadRadius: 2 * opacity,
                              ),
                            ],
                          ),
                        );
                      }),
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
}
