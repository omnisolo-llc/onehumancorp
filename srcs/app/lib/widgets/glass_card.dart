import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final double borderRadius;
  final Color? color;
  final double blur;

  const GlassCard({
    super.key,
    required this.child,
    this.padding = EdgeInsets.zero,
    this.margin,
    this.borderRadius = 16.0,
    this.color,
    this.blur = 20.0,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final defaultColor = Theme.of(context).colorScheme.primary;
    final cardColor = widget.color ?? defaultColor;

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOutCubic,
        child: Container(
          margin: widget.margin,
          child: ClipRRect(
            borderRadius: BorderRadius.circular(widget.borderRadius),
            child: BackdropFilter(
              filter: ImageFilter.compose(
                outer: const ColorFilter.matrix(<double>[
                  1.168, -0.153, -0.015, 0, 0,
                  -0.046, 1.061, -0.015, 0, 0,
                  -0.046, -0.152, 1.198, 0, 0,
                  0, 0, 0, 1, 0,
                ]),
                inner: ImageFilter.blur(sigmaX: widget.blur, sigmaY: widget.blur),
              ),
              child: Container(
                padding: widget.padding,
                decoration: BoxDecoration(
                  color: cardColor.withValues(alpha: 0.05),
                  border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
                  borderRadius: BorderRadius.circular(widget.borderRadius),
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
