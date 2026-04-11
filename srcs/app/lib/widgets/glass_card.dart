import 'package:flutter/material.dart';
import 'dart:ui';

class GlassCard extends StatefulWidget {
  final Widget child;
  final double blur;
  final double scaleOnHover;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final BorderRadius? borderRadius;
  final ShapeBorder? shape;
  final Clip? clipBehavior;
  final VoidCallback? onTap;

  const GlassCard({
    super.key,
    required this.child,
    this.blur = 20.0,
    this.scaleOnHover = 1.02,
    this.padding,
    this.margin,
    this.borderRadius,
    this.shape,
    this.clipBehavior,
    this.onTap,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final isDark = theme.brightness == Brightness.dark;

    // OHC specific color matrix - high saturation for premium look
    final colorMatrix = [
      1.2, 0.0, 0.0, 0.0, 0.0,
      0.0, 1.2, 0.0, 0.0, 0.0,
      0.0, 0.0, 1.2, 0.0, 0.0,
      0.0, 0.0, 0.0, 1.0, 0.0,
    ];

    BorderRadius effectiveBorderRadius = widget.borderRadius ?? BorderRadius.circular(16.0);
    if (widget.shape is RoundedRectangleBorder) {
      effectiveBorderRadius = (widget.shape as RoundedRectangleBorder).borderRadius as BorderRadius;
    }

    Widget content = AnimatedScale(
      scale: _isHovered ? widget.scaleOnHover : 1.0,
      duration: const Duration(milliseconds: 200),
      curve: Curves.easeOutCubic,
      child: ClipRRect(
        borderRadius: effectiveBorderRadius,
        clipBehavior: widget.clipBehavior ?? Clip.antiAlias,
        child: BackdropFilter(
          filter: ImageFilter.compose(
            outer: ColorFilter.matrix(colorMatrix),
            inner: ImageFilter.blur(sigmaX: widget.blur, sigmaY: widget.blur),
          ),
          child: Container(
            padding: widget.padding,
            decoration: BoxDecoration(
              color: isDark
                  ? Colors.white.withValues(alpha: 0.05)
                  : Colors.black.withValues(alpha: 0.02),
              borderRadius: effectiveBorderRadius,
              border: Border.all(
                color: isDark
                    ? Colors.white.withValues(alpha: 0.1)
                    : Colors.black.withValues(alpha: 0.05),
                width: 1.0,
              ),
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withValues(alpha: 0.05),
                  blurRadius: 10,
                  spreadRadius: -5,
                ),
              ],
            ),
            child: Material(
              color: Colors.transparent,
              child: widget.child,
            ),
          ),
        ),
      ),
    );

    if (widget.onTap != null) {
      content = GestureDetector(
        onTap: widget.onTap,
        child: content,
      );
    }

    Widget region = MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      cursor: widget.onTap != null ? SystemMouseCursors.click : SystemMouseCursors.basic,
      child: content,
    );

    if (widget.margin != null) {
      return Padding(
        padding: widget.margin!,
        child: region,
      );
    }

    return region;
  }
}
