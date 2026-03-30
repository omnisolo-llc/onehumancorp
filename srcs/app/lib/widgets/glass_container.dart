import 'dart:ui';
import 'package:flutter/material.dart';

class GlassContainer extends StatelessWidget {
  final Widget child;
  final double? width;
  final double? height;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final BorderRadiusGeometry borderRadius;

  const GlassContainer({
    super.key,
    required this.child,
    this.width,
    this.height,
    this.padding,
    this.margin,
    this.borderRadius = const BorderRadius.all(Radius.circular(16)),
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      width: width,
      height: height,
      margin: margin,
      decoration: BoxDecoration(
        borderRadius: borderRadius,
        border: Border.all(color: Colors.white.withOpacity(0.08)),
        color: Colors.white.withOpacity(0.03),
      ),
      child: ClipRRect(
        borderRadius: borderRadius,
        child: BackdropFilter(
          filter: ImageFilter.compose(
            outer: ColorFilter.matrix(const [
              2, 0, 0, 0, 0,
              0, 2, 0, 0, 0,
              0, 0, 2, 0, 0,
              0, 0, 0, 1, 0,
            ]), // saturate(200%)
            inner: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
          ),
          child: Container(
            padding: padding,
            child: child,
          ),
        ),
      ),
    );
  }
}
