import 'dart:ui';
import 'package:flutter/material.dart';

class AutoDreamPipelineWidget extends StatefulWidget {
  const AutoDreamPipelineWidget({Key? key}) : super(key: key);

  @override
  State<AutoDreamPipelineWidget> createState() => _AutoDreamPipelineWidgetState();
}

class _AutoDreamPipelineWidgetState extends State<AutoDreamPipelineWidget> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _flowAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 3),
    )..repeat();

    _flowAnimation = Tween<double>(begin: 0.0, end: 1.0).animate(
      CurvedAnimation(parent: _controller, curve: Curves.linear),
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  Widget _buildNode(String label, IconData icon, Color color) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(
          width: 48,
          height: 48,
          decoration: BoxDecoration(
            color: color.withOpacity(0.2),
            shape: BoxShape.circle,
            border: Border.all(color: color.withOpacity(0.5), width: 2),
            boxShadow: [
              BoxShadow(
                color: color.withOpacity(0.3),
                blurRadius: 10,
                spreadRadius: 2,
              )
            ],
          ),
          child: Icon(icon, color: color, size: 24),
        ),
        const SizedBox(height: 8),
        Text(
          label,
          style: const TextStyle(
            fontFamily: 'Inter',
            fontSize: 10,
            color: Colors.white70,
            fontWeight: FontWeight.w500,
          ),
          textAlign: TextAlign.center,
        )
      ],
    );
  }

  Widget _buildAnimatedConnection(Color color) {
    return Expanded(
      child: AnimatedBuilder(
        animation: _flowAnimation,
        builder: (context, child) {
          return CustomPaint(
            painter: _ConnectionPainter(
              progress: _flowAnimation.value,
              color: color,
            ),
            child: const SizedBox(height: 24),
          );
        },
      ),
    );
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
                'AutoDream Pipeline Stream',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 24),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  _buildNode('Extract', Icons.data_object, Colors.blueAccent),
                  _buildAnimatedConnection(Colors.blueAccent),
                  _buildNode('Analyze', Icons.psychology, Colors.purpleAccent),
                  _buildAnimatedConnection(Colors.purpleAccent),
                  _buildNode('Embed', Icons.scatter_plot, Colors.cyanAccent),
                  _buildAnimatedConnection(Colors.cyanAccent),
                  _buildNode('Store', Icons.save_alt, Colors.greenAccent),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _ConnectionPainter extends CustomPainter {
  final double progress;
  final Color color;

  _ConnectionPainter({required this.progress, required this.color});

  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = color.withOpacity(0.2)
      ..strokeWidth = 2
      ..style = PaintingStyle.stroke;

    final dotPaint = Paint()
      ..color = color
      ..style = PaintingStyle.fill;

    // Draw base line
    canvas.drawLine(
      Offset(0, size.height / 2),
      Offset(size.width, size.height / 2),
      paint,
    );

    // Draw moving dot
    final dotX = size.width * progress;
    canvas.drawCircle(Offset(dotX, size.height / 2), 4, dotPaint);

    // Draw trail
    final trailPaint = Paint()
      ..shader = LinearGradient(
        colors: [color.withOpacity(0.0), color.withOpacity(0.8)],
        stops: const [0.0, 1.0],
      ).createShader(Rect.fromLTWH(dotX - 20, 0, 20, size.height))
      ..strokeWidth = 3
      ..style = PaintingStyle.stroke;

    if (dotX > 20) {
      canvas.drawLine(
        Offset(dotX - 20, size.height / 2),
        Offset(dotX, size.height / 2),
        trailPaint,
      );
    }
  }

  @override
  bool shouldRepaint(covariant _ConnectionPainter oldDelegate) {
    return oldDelegate.progress != progress || oldDelegate.color != color;
  }
}
