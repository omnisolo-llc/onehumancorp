import 'dart:ui';
import 'package:flutter/material.dart';

class PgvectorMemoryVisualizerWidget extends StatefulWidget {
  const PgvectorMemoryVisualizerWidget({Key? key}) : super(key: key);

  @override
  State<PgvectorMemoryVisualizerWidget> createState() => _PgvectorMemoryVisualizerWidgetState();
}

class _PgvectorMemoryVisualizerWidgetState extends State<PgvectorMemoryVisualizerWidget> with SingleTickerProviderStateMixin {
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
                'Pgvector Memory Visualizer',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 16),
              SizedBox(
                height: 40,
                child: AnimatedBuilder(
                  animation: _pulseAnimation,
                  builder: (context, child) {
                    return Row(
                      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                      children: List.generate(8, (index) {
                        final opacity = (_pulseAnimation.value + (index * 0.125)) % 1.0;
                        return Container(
                          width: 8,
                          height: 8 + (opacity * 24),
                          decoration: BoxDecoration(
                            color: Colors.purpleAccent.withOpacity(0.5 + (opacity * 0.5)),
                            borderRadius: BorderRadius.circular(4),
                            boxShadow: [
                              BoxShadow(
                                color: Colors.purpleAccent.withOpacity(opacity * 0.8),
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
            ],
          ),
        ),
      ),
    );
  }
}
