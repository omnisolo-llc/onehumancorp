import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final double? width;
  final double? height;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final VoidCallback? onTap;

  // Dummy parameters to absorb Card attributes safely
  final dynamic elevation;
  final dynamic color;
  final dynamic shape;
  final dynamic clipBehavior;
  final dynamic semanticContainer;
  final dynamic shadowColor;
  final dynamic surfaceTintColor;

  const GlassCard({
    super.key,
    required this.child,
    this.width,
    this.height,
    this.padding,
    this.margin,
    this.onTap,
    this.elevation,
    this.color,
    this.shape,
    this.clipBehavior,
    this.semanticContainer,
    this.shadowColor,
    this.surfaceTintColor,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    return Container(
      width: widget.width,
      height: widget.height,
      margin: widget.margin ?? const EdgeInsets.all(8.0),
      child: MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        cursor: widget.onTap != null ? SystemMouseCursors.click : MouseCursor.defer,
        child: GestureDetector(
          onTap: widget.onTap,
          child: AnimatedScale(
            scale: _isHovered ? 1.02 : 1.0,
            duration: const Duration(milliseconds: 200),
            curve: Curves.easeInOut,
            child: ClipRRect(
              borderRadius: BorderRadius.circular(16.0),
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                child: Container(
                  padding: widget.padding,
                  decoration: BoxDecoration(
                    color: Theme.of(context).cardColor.withAlpha(150),
                    borderRadius: BorderRadius.circular(16.0),
                    border: Border.all(
                      color: Colors.white.withAlpha(50),
                      width: 1.0,
                    ),
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
