import 'dart:ui';
import 'package:flutter/material.dart';

class GlassContainer extends StatelessWidget {
  final Widget child;
  final EdgeInsetsGeometry padding;

  const GlassContainer({
    super.key,
    required this.child,
    this.padding = const EdgeInsets.all(16.0),
  });

  @override
  Widget build(BuildContext context) {
    // The design system tokens enforce: backdrop-filter: blur(20px) saturate(200%)
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.compose(
          outer: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
          inner: const ColorFilter.matrix(<double>[
            2.0, 0, 0, 0, 0, // R
            0, 2.0, 0, 0, 0, // G
            0, 0, 2.0, 0, 0, // B
            0, 0, 0, 1, 0, // A
          ]),
        ),
        child: Container(
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.05),
            border: Border.all(
              color: const Color.fromRGBO(255, 255, 255, 0.1),
              width: 1.0,
            ),
            borderRadius: BorderRadius.circular(16),
          ),
          padding: padding,
          child: child,
        ),
      ),
    );
  }
}
