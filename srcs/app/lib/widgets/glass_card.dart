import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final EdgeInsetsGeometry padding;
  final EdgeInsetsGeometry margin;
  final bool animateHover;
  final VoidCallback? onTap;
  final Color? baseColor;

  const GlassCard({
    super.key,
    required this.child,
    this.padding = const EdgeInsets.all(24),
    this.margin = EdgeInsets.zero,
    this.animateHover = false,
    this.onTap,
    this.baseColor,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> with SingleTickerProviderStateMixin {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final effectiveColor = widget.baseColor ?? Theme.of(context).colorScheme.primary;

    Widget content = ClipRRect(
      borderRadius: BorderRadius.circular(16),
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
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 300),
          margin: widget.margin,
          decoration: BoxDecoration(
            color: _isHovered
                ? effectiveColor.withValues(alpha: 0.08)
                : effectiveColor.withValues(alpha: 0.03),
            borderRadius: BorderRadius.circular(16),
            border: Border.all(
              color: _isHovered
                  ? Colors.white.withValues(alpha: 0.3)
                  : Colors.white.withValues(alpha: 0.1),
            ),
          ),
          child: Material(
            color: Colors.transparent,
            child: widget.onTap != null
                ? InkWell(
                    onTap: widget.onTap,
                    borderRadius: BorderRadius.circular(16),
                    splashColor: effectiveColor.withValues(alpha: 0.1),
                    highlightColor: effectiveColor.withValues(alpha: 0.05),
                    child: Padding(
                      padding: widget.padding,
                      child: widget.child,
                    ),
                  )
                : Padding(
                    padding: widget.padding,
                    child: widget.child,
                  ),
          ),
        ),
      ),
    );

    if (widget.animateHover) {
      content = MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        child: AnimatedScale(
          scale: _isHovered ? 1.02 : 1.0,
          duration: const Duration(milliseconds: 200),
          curve: Curves.easeOutCubic,
          child: content,
        ),
      );
    }

    return content;
  }
}
