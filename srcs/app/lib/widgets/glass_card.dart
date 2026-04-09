import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final EdgeInsetsGeometry? margin;

  const GlassCard({super.key, required this.child, this.margin});

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final color = Theme.of(context).colorScheme.primary;

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOutCubic,
        child: Container(
          margin: widget.margin ?? const EdgeInsets.all(4.0),
          child: ClipRRect(
            borderRadius: BorderRadius.circular(16),
            child: BackdropFilter(
              filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
              child: Container(
                decoration: BoxDecoration(
                  color: color.withValues(alpha: 0.05),
                  border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
                  borderRadius: BorderRadius.circular(16),
                ),
                child: Material(
                  type: MaterialType.transparency,
                  child: widget.child,
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
