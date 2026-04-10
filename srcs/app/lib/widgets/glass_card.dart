import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final BoxConstraints? constraints;
  final VoidCallback? onTap;
  final Color? color;

  const GlassCard({
    super.key,
    required this.child,
    this.padding,
    this.margin,
    this.constraints,
    this.onTap,
    this.color,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;
  bool _isPressed = false;

  @override
  Widget build(BuildContext context) {
    final themeColor = widget.color ?? Theme.of(context).colorScheme.primary;

    Widget cardContent = ClipRRect(
      borderRadius: BorderRadius.circular(12),
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
        child: Container(
          padding: widget.padding,
          decoration: BoxDecoration(
            color: themeColor.withValues(alpha: 0.03),
            border: Border.all(color: themeColor.withValues(alpha: 0.08)),
            borderRadius: BorderRadius.circular(12),
          ),
          child: Material(
            type: MaterialType.transparency,
            child: widget.child,
          ),
        ),
      ),
    );

    if (widget.constraints != null) {
      cardContent = ConstrainedBox(
        constraints: widget.constraints!,
        child: cardContent,
      );
    }

    if (widget.margin != null) {
      cardContent = Padding(
        padding: widget.margin!,
        child: cardContent,
      );
    }

    Widget animatedCard = AnimatedScale(
      scale: _isHovered || _isPressed ? 1.02 : 1.0,
      duration: const Duration(milliseconds: 200),
      curve: Curves.easeOutCubic,
      child: cardContent,
    );

    if (widget.onTap != null) {
      return MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        cursor: SystemMouseCursors.click,
        child: GestureDetector(
          onTapDown: (_) => setState(() => _isPressed = true),
          onTapUp: (_) => setState(() => _isPressed = false),
          onTapCancel: () => setState(() => _isPressed = false),
          onTap: widget.onTap,
          child: animatedCard,
        ),
      );
    } else {
      return MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        child: animatedCard,
      );
    }
  }
}
