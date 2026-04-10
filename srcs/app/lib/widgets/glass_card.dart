import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget? child;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final ShapeBorder? shape;
  final Color? color;
  final double? elevation;
  final Clip? clipBehavior;

  const GlassCard({
    super.key,
    this.child,
    this.padding,
    this.margin,
    this.shape,
    this.color,
    this.elevation,
    this.clipBehavior,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final cardColor = widget.color ?? theme.colorScheme.surface.withValues(alpha: 0.05);
    final shape = widget.shape ?? RoundedRectangleBorder(borderRadius: BorderRadius.circular(16));

    Widget content = Container(
      padding: widget.padding,
      decoration: ShapeDecoration(
        color: cardColor,
        shape: shape,
      ),
      clipBehavior: widget.clipBehavior ?? Clip.none,
      child: widget.child,
    );

    // Fallback to RoundedRectangleBorder if shape is not OutlinedBorder
    OutlinedBorder outlinedShape = shape is OutlinedBorder
        ? shape
        : RoundedRectangleBorder(borderRadius: BorderRadius.circular(16));

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOut,
        child: Padding(
          padding: widget.margin ?? EdgeInsets.zero,
          child: ClipPath(
            clipper: ShapeBorderClipper(shape: shape),
            child: BackdropFilter(
              filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
              child: Container(
                decoration: ShapeDecoration(
                  color: Colors.transparent,
                  shape: outlinedShape.copyWith(
                    side: BorderSide(
                      color: Colors.white.withValues(alpha: 0.1),
                    ),
                  ),
                ),
                child: content,
              ),
            ),
          ),
        ),
      ),
    );
  }
}
