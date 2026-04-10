import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final EdgeInsetsGeometry? margin;
  final EdgeInsetsGeometry? padding;
  final Color? color;
  final double? elevation;
  final ShapeBorder? shape;

  const GlassCard({
    super.key,
    required this.child,
    this.margin,
    this.padding,
    this.color,
    this.elevation,
    this.shape,
  });

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
        curve: Curves.easeOut,
        child: Container(
          margin: widget.margin,
          padding: widget.padding,
          decoration: ShapeDecoration(
            color: widget.color ?? Colors.white.withOpacity(0.03),
            shape:
                widget.shape ??
                RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(12.0),
                  side: BorderSide(
                    color: Colors.white.withOpacity(0.08),
                    width: 1.0,
                  ),
                ),
            shadows: widget.elevation != null && widget.elevation! > 0
                ? [
                    BoxShadow(
                      color: Colors.black.withOpacity(0.1),
                      blurRadius: widget.elevation! * 2,
                      offset: Offset(0, widget.elevation!),
                    ),
                  ]
                : null,
          ),
          child: ClipRRect(
            borderRadius: widget.shape is RoundedRectangleBorder
                ? (widget.shape as RoundedRectangleBorder).borderRadius.resolve(
                    Directionality.of(context),
                  )
                : BorderRadius.circular(12.0),
            child: BackdropFilter(
              filter: ImageFilter.compose(
                outer: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                inner: const ColorFilter.matrix([
                  1.7874,
                  -0.7152,
                  -0.0722,
                  0,
                  0,
                  -0.2126,
                  1.2848,
                  -0.0722,
                  0,
                  0,
                  -0.2126,
                  -0.7152,
                  1.9278,
                  0,
                  0,
                  0,
                  0,
                  0,
                  1,
                  0,
                ]),
              ),
              child: widget.child,
            ),
          ),
        ),
      ),
    );
  }
}
