import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final EdgeInsetsGeometry padding;
  final EdgeInsetsGeometry margin;
  final bool animateOnHover;

  const GlassCard({
    super.key,
    required this.child,
    this.padding = EdgeInsets.zero,
    this.margin = const EdgeInsets.all(4.0),
    this.animateOnHover = true,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final scale = (_isHovered && widget.animateOnHover) ? 1.02 : 1.0;

    Widget content = Padding(
      padding: widget.padding,
      child: widget.child,
    );

    return Container(
      margin: widget.margin,
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(12),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withValues(alpha: 0.1),
            blurRadius: 8,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        child: AnimatedScale(
          scale: scale,
          duration: const Duration(milliseconds: 200),
          curve: Curves.easeInOut,
          child: ClipRRect(
            borderRadius: BorderRadius.circular(12),
            child: BackdropFilter(
              filter: ImageFilter.compose(
                outer: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
                inner: const ColorFilter.matrix([
                  2, 0, 0, 0, 0,
                  0, 2, 0, 0, 0,
                  0, 0, 2, 0, 0,
                  0, 0, 0, 1, 0,
                ]), // 200% saturation approx
              ),
              child: Container(
                decoration: BoxDecoration(
                  color: Colors.white.withValues(alpha: 0.05),
                  borderRadius: BorderRadius.circular(12),
                  border: Border.all(
                    color: Colors.white.withValues(alpha: 0.08),
                    width: 1,
                  ),
                ),
                child: Material(
                  type: MaterialType.transparency,
                  child: content,
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
