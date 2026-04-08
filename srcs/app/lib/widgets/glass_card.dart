import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final double blur;
  final double scaleOnHover;
  final EdgeInsetsGeometry padding;
  final EdgeInsetsGeometry? margin;

  const GlassCard({
    super.key,
    required this.child,
    this.blur = 20.0,
    this.scaleOnHover = 1.02,
    this.padding = EdgeInsets.zero,
    this.margin,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    Widget card = MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? widget.scaleOnHover : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOutCubic,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(16.0),
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: ImageFilter.blur(sigmaX: widget.blur, sigmaY: widget.blur),
              inner: ColorFilter.matrix(const [
                2, 0, 0, 0, 0, // Saturate red
                0, 2, 0, 0, 0, // Saturate green
                0, 0, 2, 0, 0, // Saturate blue
                0, 0, 0, 1, 0, // Alpha
              ]),
            ),
            child: Container(
              decoration: BoxDecoration(
                color: Colors.white.withOpacity(0.03),
                borderRadius: BorderRadius.circular(16.0),
                border: Border.all(
                  color: Colors.white.withOpacity(0.1),
                ),
              ),
              child: Padding(
                padding: widget.padding,
                child: widget.child,
              ),
            ),
          ),
        ),
      ),
    );

    if (widget.margin != null) {
      return Padding(
        padding: widget.margin!,
        child: card,
      );
    }
    return card;
  }
}
