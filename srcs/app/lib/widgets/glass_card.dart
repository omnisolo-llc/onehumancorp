import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget? child;
  final EdgeInsetsGeometry? margin;
  final Color? color;
  final double? elevation;
  final ShapeBorder? shape;
  final Color? surfaceTintColor;
  final Color? shadowColor;
  final Clip? clipBehavior;

  const GlassCard({
    super.key,
    this.child,
    this.margin,
    this.color,
    this.elevation,
    this.shape,
    this.surfaceTintColor,
    this.shadowColor,
    this.clipBehavior,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final defaultRadius = BorderRadius.circular(12.0);
    BorderRadiusGeometry resolvedRadius = defaultRadius;

    if (widget.shape is RoundedRectangleBorder) {
      resolvedRadius = (widget.shape as RoundedRectangleBorder).borderRadius;
    }

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOutCubic,
        child: Container(
          margin: widget.margin ?? const EdgeInsets.all(4.0),
          child: ClipRRect(
            borderRadius: resolvedRadius is BorderRadius ? resolvedRadius : defaultRadius,
            child: BackdropFilter(
              filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
              child: Container(
                decoration: BoxDecoration(
                  color: widget.color ?? Colors.white.withOpacity(0.03),
                  borderRadius: resolvedRadius is BorderRadius ? resolvedRadius : defaultRadius,
                  border: Border.all(
                    color: Colors.white.withOpacity(0.1),
                    width: 1.0,
                  ),
                ),
                child: widget.child,
              ),
            ),
          ),
        ),
      ),
    );
  }
}
