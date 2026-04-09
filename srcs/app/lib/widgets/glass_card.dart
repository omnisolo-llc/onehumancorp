import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget? child;
  final EdgeInsetsGeometry? margin;
  final EdgeInsetsGeometry? padding;

  const GlassCard({
    super.key,
    this.child,
    this.margin,
    this.padding,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  static const ColorFilter _saturationFilter = ColorFilter.matrix(<double>[
    0.2126 + 0.7874 * 2.0, 0.7152 - 0.7152 * 2.0, 0.0722 - 0.0722 * 2.0, 0, 0,
    0.2126 - 0.2126 * 2.0, 0.7152 + 0.2848 * 2.0, 0.0722 - 0.0722 * 2.0, 0, 0,
    0.2126 - 0.2126 * 2.0, 0.7152 - 0.7152 * 2.0, 0.0722 + 0.9278 * 2.0, 0, 0,
    0, 0, 0, 1, 0,
  ]);

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOut,
        child: Container(
          margin: widget.margin,
          child: ClipRRect(
            borderRadius: BorderRadius.circular(16),
            child: BackdropFilter(
              filter: ImageFilter.compose(
                outer: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                inner: _saturationFilter,
              ),
              child: Container(
                padding: widget.padding,
                decoration: BoxDecoration(
                  color: Colors.white.withValues(alpha: 0.03),
                  border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
                  borderRadius: BorderRadius.circular(16),
                ),
                child: Material(type: MaterialType.transparency, child: widget.child),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
