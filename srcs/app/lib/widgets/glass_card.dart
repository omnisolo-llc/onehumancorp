import 'package:flutter/material.dart';
import 'dart:ui';

class GlassCard extends StatefulWidget {
  final Widget? child;
  final Color? color;
  final Color? shadowColor;
  final Color? surfaceTintColor;
  final double? elevation;
  final ShapeBorder? shape;
  final bool borderOnForeground;
  final EdgeInsetsGeometry? margin;
  final Clip? clipBehavior;
  final bool semanticContainer;
  final VoidCallback? onTap;

  const GlassCard({
    super.key,
    this.color,
    this.shadowColor,
    this.surfaceTintColor,
    this.elevation,
    this.shape,
    this.borderOnForeground = true,
    this.margin,
    this.clipBehavior,
    this.semanticContainer = true,
    this.child,
    this.onTap,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final effectiveColor =
        widget.color ?? Theme.of(context).colorScheme.surface;
    final effectiveMargin = widget.margin ?? const EdgeInsets.all(4.0);

    BorderRadiusGeometry borderRadius = BorderRadius.circular(16.0);
    if (widget.shape is RoundedRectangleBorder) {
      borderRadius = (widget.shape as RoundedRectangleBorder).borderRadius;
    }

    Widget content = ClipRRect(
      borderRadius: borderRadius.resolve(Directionality.maybeOf(context)),
      clipBehavior: widget.clipBehavior ?? Clip.antiAlias,
      child: BackdropFilter(
        filter: ImageFilter.compose(
          outer: const ColorFilter.matrix(<double>[
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
          inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        ),
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 300),
          decoration: BoxDecoration(
            color:
                _isHovered
                    ? effectiveColor.withValues(alpha: 0.08)
                    : effectiveColor.withValues(alpha: 0.03),
            borderRadius: borderRadius,
            border: Border.all(
              color:
                  _isHovered
                      ? Colors.white.withValues(alpha: 0.3)
                      : Colors.white.withValues(alpha: 0.1),
            ),
          ),
          child: Material(
            color: Colors.transparent,
            child:
                widget.onTap != null
                    ? InkWell(
                      onTap: widget.onTap,
                      borderRadius: borderRadius.resolve(
                        Directionality.maybeOf(context),
                      ),
                      splashColor: effectiveColor.withValues(alpha: 0.1),
                      highlightColor: effectiveColor.withValues(alpha: 0.05),
                      child: widget.child,
                    )
                    : widget.child,
          ),
        ),
      ),
    );

    if (widget.semanticContainer) {
      content = Semantics(
        container: true,
        explicitChildNodes: !widget.semanticContainer,
        child: content,
      );
    }

    return Padding(
      padding: effectiveMargin,
      child: MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        child: AnimatedScale(
          scale: _isHovered ? 1.02 : 1.0,
          duration: const Duration(milliseconds: 200),
          curve: Curves.easeOutCubic,
          child: content,
        ),
      ),
    );
  }
}
