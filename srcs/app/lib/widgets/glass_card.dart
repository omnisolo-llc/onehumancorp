import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final Color? color;
  final EdgeInsetsGeometry margin;

  const GlassCard({
    super.key,
    required this.child,
    this.color,
    this.margin = const EdgeInsets.all(4.0),
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final themeColor = widget.color ?? Theme.of(context).colorScheme.primary;

    return Padding(
      padding: widget.margin,
      child: MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        child: AnimatedScale(
          scale: _isHovered ? 1.02 : 1.0,
          duration: const Duration(milliseconds: 200),
          curve: Curves.easeOutQuart,
          child: ClipRRect(
            borderRadius: BorderRadius.circular(12),
            child: BackdropFilter(
              filter: ImageFilter.compose(
                outer: ColorFilter.matrix(<double>[
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
              child: Material(
                type: MaterialType.transparency,
                child: Container(
                  padding: EdgeInsets.zero,
                  decoration: BoxDecoration(
                    color: themeColor.withValues(alpha: 0.05),
                    border: Border.all(color: themeColor.withValues(alpha: 0.1)),
                    borderRadius: BorderRadius.circular(12),
                  ),
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
