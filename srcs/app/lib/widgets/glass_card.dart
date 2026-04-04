import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatelessWidget {
  final Widget child;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final double? width;
  final double? height;

  const GlassCard({
    super.key,
    required this.child,
    this.padding,
    this.margin,
    this.width,
    this.height,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: margin,
      width: width,
      height: height,
      child: ClipRRect(
        borderRadius: BorderRadius.circular(12),
        child: BackdropFilter(
          // blur(20px)
          filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
          child: Container(
            // background: rgba(255, 255, 255, 0.03)
            decoration: BoxDecoration(
              color: const Color(0x08FFFFFF),
              borderRadius: BorderRadius.circular(12),
              border: Border.all(
                color: Colors.white.withValues(alpha: 0.1),
                width: 1,
              ),
            ),
            padding: padding,
            // Apply saturate(200%) using ColorFilter
            child: ColorFiltered(
              colorFilter: const ColorFilter.matrix([
                // Matrix for saturation 200%
                0.213 + 0.787 * 2.0,
                0.715 - 0.715 * 2.0,
                0.072 - 0.072 * 2.0,
                0,
                0,
                0.213 - 0.213 * 2.0,
                0.715 + 0.285 * 2.0,
                0.072 - 0.072 * 2.0,
                0,
                0,
                0.213 - 0.213 * 2.0,
                0.715 - 0.715 * 2.0,
                0.072 + 0.928 * 2.0,
                0,
                0,
                0, 0, 0, 1, 0,
              ]),
              child: child,
            ),
          ),
        ),
      ),
    );
  }
}
