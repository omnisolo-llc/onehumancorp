import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final EdgeInsetsGeometry? margin;
  final EdgeInsetsGeometry? padding;
  final Color? color;
  final ShapeBorder? shape;
  final double? elevation;
  final VoidCallback? onTap;

  const GlassCard({
    super.key,
    required this.child,
    this.margin,
    this.padding,
    this.color,
    this.shape,
    this.elevation,
    this.onTap,
  });

  @override
  _GlassCardState createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final shape = widget.shape ?? RoundedRectangleBorder(borderRadius: BorderRadius.circular(16));
    BorderRadiusGeometry? borderRadius;
    if (shape is RoundedRectangleBorder) {
      borderRadius = shape.borderRadius;
    } else {
      borderRadius = BorderRadius.circular(16);
    }

    final cardContent = AnimatedScale(
      scale: _isHovered ? 1.02 : 1.0,
      duration: const Duration(milliseconds: 200),
      child: MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        child: GestureDetector(
          onTap: widget.onTap,
          child: Container(
            margin: widget.margin,
            child: ClipRRect(
              borderRadius: borderRadius.resolve(Directionality.maybeOf(context)),
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                child: Container(
                  padding: widget.padding ?? const EdgeInsets.all(16.0),
                  decoration: BoxDecoration(
                    color: (widget.color ?? Theme.of(context).colorScheme.surface).withValues(alpha: 0.05),
                    border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
                    borderRadius: borderRadius.resolve(Directionality.maybeOf(context)),
                  ),
                  child: widget.child,
                ),
              ),
            ),
          ),
        ),
      ),
    );

    return cardContent;
  }
}
