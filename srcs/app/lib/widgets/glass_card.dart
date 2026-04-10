import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget? child;
  final Color? color;
  final EdgeInsetsGeometry? margin;
  final EdgeInsetsGeometry? padding;
  final ShapeBorder? shape;
  final double? elevation;

  const GlassCard({
    Key? key,
    this.child,
    this.color,
    this.margin,
    this.padding,
    this.shape,
    this.elevation,
  }) : super(key: key);

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    // Standard W3C saturate(200%) ColorFilter.matrix values
    const ColorFilter saturateFilter = ColorFilter.matrix(<double>[
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
    ]);

    final shape =
        widget.shape ??
        RoundedRectangleBorder(borderRadius: BorderRadius.circular(12.0));

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOut,
        child: Padding(
          padding: widget.margin ?? const EdgeInsets.all(4.0),
          child: ClipPath(
            clipper: ShapeBorderClipper(shape: shape),
            child: BackdropFilter(
              filter: ImageFilter.compose(
                outer: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                inner: saturateFilter,
              ),
              child: Container(
                padding: widget.padding,
                decoration: ShapeDecoration(
                  color:
                      widget.color ?? const Color.fromRGBO(255, 255, 255, 0.03),
                  shape: shape,
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
