import 'package:flutter/material.dart';
import 'dart:ui';

class GlassCard extends StatefulWidget {
  final Widget? child;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final Color? color;
  final Color? borderColor;
  final ShapeBorder? shape;
  final double? elevation;
  final VoidCallback? onTap;

  const GlassCard({
    super.key,
    this.child,
    this.padding,
    this.margin,
    this.color,
    this.borderColor,
    this.shape,
    this.elevation,
    this.onTap,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final themeColor = widget.color ?? Theme.of(context).colorScheme.primary;
    final bColor = widget.borderColor ?? Colors.white.withValues(alpha: 0.1);

    // We try to extract a border radius from the shape, otherwise default to 12.
    BorderRadiusGeometry borderRadius = BorderRadius.circular(12);
    if (widget.shape is RoundedRectangleBorder) {
      borderRadius = (widget.shape as RoundedRectangleBorder).borderRadius;
    }

    Widget content = Container(
      margin: widget.margin,
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOut,
        child: ClipRRect(
          borderRadius: borderRadius,
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: const ColorFilter.matrix(<double>[
                1.168, -0.153, -0.015, 0, 0,
                -0.046, 1.061, -0.015, 0, 0,
                -0.046, -0.152, 1.198, 0, 0,
                0, 0, 0, 1, 0,
              ]),
              inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            ),
            child: Container(
              padding: widget.padding,
              decoration: BoxDecoration(
                color: themeColor.withValues(alpha: 0.05),
                borderRadius: borderRadius,
                border: Border.all(color: bColor),
              ),
              child: widget.child,
            ),
          ),
        ),
      ),
    );

    if (widget.onTap != null) {
      content = GestureDetector(
        onTap: widget.onTap,
        child: content,
      );
    }

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: content,
    );
  }
}
