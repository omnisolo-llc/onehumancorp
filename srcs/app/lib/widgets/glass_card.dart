import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final EdgeInsetsGeometry? margin;
  final Color? color;

  const GlassCard({super.key, required this.child, this.margin, this.color});

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final color = widget.color ?? theme.colorScheme.primary;

    Widget cardContent = MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOut,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(16),
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: const ColorFilter.matrix(<double>[
                1.168,
                -0.153,
                -0.015,
                0,
                0,
                -0.046,
                1.061,
                -0.015,
                0,
                0,
                -0.046,
                -0.152,
                1.198,
                0,
                0,
                0,
                0,
                0,
                1,
                0,
              ]),
              inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            ),
            child: Container(
              decoration: BoxDecoration(
                color: color.withValues(alpha: 0.05),
                border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
                borderRadius: BorderRadius.circular(16),
              ),
              child: widget.child,
            ),
          ),
        ),
      ),
    );

    if (widget.margin != null) {
      return Padding(padding: widget.margin!, child: cardContent);
    }
    return cardContent;
  }
}
