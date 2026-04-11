import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget? child;
  final Color? color;
  final EdgeInsetsGeometry? margin;
  final ShapeBorder? shape;

  const GlassCard({
    super.key,
    this.child,
    this.color,
    this.margin,
    this.shape,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final themeColor = widget.color ?? Theme.of(context).colorScheme.surface;

    // We try to extract radius from shape if it's a RoundedRectangleBorder
    BorderRadiusGeometry borderRadius = BorderRadius.circular(12);
    if (widget.shape is RoundedRectangleBorder) {
      borderRadius = (widget.shape as RoundedRectangleBorder).borderRadius;
    }

    Widget cardContent = MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOutQuart,
        child: ClipRRect(
          borderRadius: borderRadius,
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: ColorFilter.matrix(<double>[
                1.168, -0.153, -0.015, 0, 0,
                -0.046, 1.061, -0.015, 0, 0,
                -0.046, -0.152, 1.198, 0, 0,
                0, 0, 0, 1, 0,
              ]),
              inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            ),
            child: Container(
              decoration: BoxDecoration(
                color: themeColor.withValues(alpha: 0.1),
                borderRadius: borderRadius is BorderRadius ? borderRadius : BorderRadius.circular(12),
                border: Border.all(color: Colors.white.withValues(alpha: 0.2)),
              ),
              child: widget.child,
            ),
          ),
        ),
      ),
    );

    if (widget.margin != null) {
      return Padding(
        padding: widget.margin!,
        child: cardContent,
      );
    }

    return cardContent;
  }
}
