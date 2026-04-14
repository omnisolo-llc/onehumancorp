import 'dart:ui';
import 'package:flutter/material.dart';

class GlassmorphismCard extends StatelessWidget {
  final Widget child;
  final double width;
  final double height;

  const GlassmorphismCard({
    Key? key,
    required this.child,
    this.width = 300,
    this.height = 200,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          width: width,
          height: height,
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            borderRadius: BorderRadius.circular(16),
            border: Border.all(color: const Color.fromRGBO(255, 255, 255, 0.1)),
          ),
          child: child,
        ),
      ),
    );
  }
}
