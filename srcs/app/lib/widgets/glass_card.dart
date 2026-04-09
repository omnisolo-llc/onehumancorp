import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget? child;
  final Color? color;
  final double? elevation;
  final EdgeInsetsGeometry? margin;
  final ShapeBorder? shape;
  final Clip? clipBehavior;
  final bool semanticContainer;

  const GlassCard({
    super.key,
    this.child,
    this.color,
    this.elevation,
    this.margin,
    this.shape,
    this.clipBehavior,
    this.semanticContainer = true,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final effectiveColor =
        widget.color ?? const Color.fromRGBO(255, 255, 255, 0.03);
    final hoverColor = const Color.fromRGBO(255, 255, 255, 0.08);

    BorderRadiusGeometry radius = BorderRadius.circular(16);
    if (widget.shape is RoundedRectangleBorder) {
      radius = (widget.shape as RoundedRectangleBorder).borderRadius;
    }

    return Padding(
      padding: widget.margin ?? const EdgeInsets.all(4.0),
      child: MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        child: AnimatedScale(
          scale: _isHovered ? 1.02 : 1.0,
          duration: const Duration(milliseconds: 200),
          curve: Curves.easeOutCubic,
          child: ClipRRect(
            borderRadius: radius.resolve(Directionality.of(context)),
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
              child: AnimatedContainer(
                duration: const Duration(milliseconds: 300),
                decoration: BoxDecoration(
                  color: _isHovered ? hoverColor : effectiveColor,
                  borderRadius: radius.resolve(Directionality.of(context)),
                  border: Border.all(
                    color: _isHovered
                        ? Colors.white.withValues(alpha: 0.3)
                        : Colors.white.withValues(alpha: 0.1),
                  ),
                ),
                child: Semantics(
                  container: widget.semanticContainer,
                  child: Material(
                    type: MaterialType.transparency,
                    clipBehavior: widget.clipBehavior ?? Clip.none,
                    child: widget.child,
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
