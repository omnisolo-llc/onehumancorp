import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatelessWidget {
  final Widget? child;
  final EdgeInsetsGeometry? margin;
  final Color? color;
  final EdgeInsetsGeometry? padding;
  final double blurRadius;
  final BorderRadiusGeometry? borderRadius;

  const GlassCard({
    super.key,
    this.child,
    this.margin,
    this.color,
    this.padding,
    this.blurRadius = 20.0,
    this.borderRadius,
  });

  @override
  Widget build(BuildContext context) {
    final effectiveBorderRadius = borderRadius ?? BorderRadius.circular(16);
    final effectiveColor = color ?? const Color.fromRGBO(255, 255, 255, 0.03);

    return Container(
      margin: margin,
      child: ClipRRect(
        borderRadius: effectiveBorderRadius,
        child: BackdropFilter(
          filter: ImageFilter.compose(
            outer: const ColorFilter.matrix(<double>[
              1.7874,
              -0.7152,
              -0.0722,
              0,
              0,
              -0.2126,
              1.2848,
              -0.0722,
              0,
              0,
              -0.2126,
              -0.7152,
              1.9278,
              0,
              0,
              0,
              0,
              0,
              1,
              0,
            ]),
            inner: ImageFilter.blur(sigmaX: blurRadius, sigmaY: blurRadius),
          ),
          child: Container(
            padding: padding,
            decoration: BoxDecoration(
              color: effectiveColor,
              borderRadius: effectiveBorderRadius as BorderRadius?,
              border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
            ),
            child: child,
          ),
        ),
      ),
    );
  }
}
