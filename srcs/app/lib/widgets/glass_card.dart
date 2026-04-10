import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final double blur;
  final double opacity;
  final Color? color;
  final BorderRadius? borderRadius;
  final VoidCallback? onTap;
  final double scaleOnHover;

  const GlassCard({
    super.key,
    required this.child,
    this.padding,
    this.margin,
    this.blur = 20.0,
    this.opacity = 0.2,
    this.color,
    this.borderRadius,
    this.onTap,
    this.scaleOnHover = 1.02,
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

    final baseColor = widget.color ?? (isDark ? Colors.white : Colors.black);
    final resolvedColor = baseColor.withValues(alpha: widget.opacity);
    final radius = widget.borderRadius ?? BorderRadius.circular(16);

    Widget content = AnimatedScale(
      scale: _isHovered ? widget.scaleOnHover : 1.0,
      duration: const Duration(milliseconds: 200),
      curve: Curves.easeOutCubic,
      child: Container(
        margin: widget.margin,
        decoration: BoxDecoration(
          borderRadius: radius,
          border: Border.all(
            color: (isDark ? Colors.white : Colors.black).withValues(alpha: 0.1),
            width: 1,
          ),
          boxShadow: [
            BoxShadow(
              color: Colors.black.withValues(alpha: 0.05),
              blurRadius: 10,
              offset: const Offset(0, 4),
            ),
          ],
        ),
        child: ClipRRect(
          borderRadius: radius,
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: ImageFilter.blur(sigmaX: widget.blur, sigmaY: widget.blur),
              inner: const ColorFilter.matrix([
                1.2, 0, 0, 0, 0,
                0, 1.2, 0, 0, 0,
                0, 0, 1.2, 0, 0,
                0, 0, 0, 1, 0,
              ]),
            ),
            child: Material(
              color: resolvedColor,
              // Removed type: MaterialType.transparency to fix assertion
              child: InkWell(
                onTap: widget.onTap,
                borderRadius: radius,
                child: Padding(
                  padding: widget.padding ?? EdgeInsets.zero,
                  child: widget.child,
                ),
              ),
            ),
          ),
        ),
      ),
    );

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: content,
    );
  }
}
