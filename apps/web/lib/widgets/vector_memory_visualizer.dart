import 'dart:ui';
import 'package:flutter/material.dart';

class VectorMemoryVisualizerWidget extends StatefulWidget {
  final List<double> vector;
  final String content;

  const VectorMemoryVisualizerWidget({
    Key? key,
    required this.vector,
    required this.content,
  }) : super(key: key);

  @override
  State<VectorMemoryVisualizerWidget> createState() => _VectorMemoryVisualizerWidgetState();
}

class _VectorMemoryVisualizerWidgetState extends State<VectorMemoryVisualizerWidget> with SingleTickerProviderStateMixin {
  late AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 2),
    )..repeat(reverse: true);
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(
              color: const Color.fromRGBO(255, 255, 255, 0.1),
            ),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                widget.content,
                style: const TextStyle(
                  fontFamily: 'Outfit',
                  color: Colors.white,
                  fontSize: 14,
                ),
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
              ),
              const SizedBox(height: 12),
              AnimatedBuilder(
                animation: _controller,
                builder: (context, child) {
                  return Wrap(
                    spacing: 4,
                    runSpacing: 4,
                    children: widget.vector.take(20).map((val) {
                      return Opacity(
                        opacity: 0.3 + (0.7 * _controller.value * val.abs()),
                        child: Container(
                          width: 8,
                          height: 8,
                          decoration: BoxDecoration(
                            color: val > 0 ? Colors.greenAccent : Colors.blueAccent,
                            shape: BoxShape.circle,
                          ),
                        ),
                      );
                    }).toList(),
                  );
                },
              ),
            ],
          ),
        ),
      ),
    );
  }
}
