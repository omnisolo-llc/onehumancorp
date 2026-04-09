import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final EdgeInsetsGeometry? margin;
  final EdgeInsetsGeometry? padding;
  final Color? color;

  const GlassCard({super.key, required this.child, this.margin, this.padding, this.color});

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOutQuart,
        child: Padding(
          padding: widget.margin ?? const EdgeInsets.all(4.0),
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
                padding: widget.padding,
                decoration: BoxDecoration(
                  color: widget.color ?? Theme.of(context).colorScheme.surface.withValues(alpha: 0.05),
                  borderRadius: BorderRadius.circular(16),
                  border: Border.all(
                    color: Colors.white.withValues(alpha: 0.1),
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
