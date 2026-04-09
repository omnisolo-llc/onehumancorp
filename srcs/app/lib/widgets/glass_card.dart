import 'package:flutter/material.dart';
import 'dart:ui';

class GlassCard extends StatefulWidget {
  final Widget child;
  final VoidCallback? onTap;
  final EdgeInsetsGeometry? margin;
  final Color? color;

  const GlassCard({
    super.key,
    required this.child,
    this.onTap,
    this.margin,
    this.color,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> with SingleTickerProviderStateMixin {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    Widget content = MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOutCubic,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(16),
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: ColorFilter.matrix(const <double>[
                1.168, -0.153, -0.015, 0, 0,
                -0.046, 1.061, -0.015, 0, 0,
                -0.046, -0.152, 1.198, 0, 0,
                0, 0, 0, 1, 0,
              ]),
              inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            ),
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 300),
              decoration: BoxDecoration(
                color: widget.color != null
                    ? widget.color!.withValues(alpha: 0.03)
                    : (_isHovered
                        ? colorScheme.surfaceContainerHighest.withValues(alpha: 0.3)
                        : colorScheme.surface.withValues(alpha: 0.1)),
                borderRadius: BorderRadius.circular(16),
                border: Border.all(
                  color: widget.color != null
                      ? widget.color!.withValues(alpha: 0.08)
                      : (_isHovered
                          ? colorScheme.outlineVariant
                          : colorScheme.outlineVariant.withValues(alpha: 0.5)),
                ),
              ),
              child: Material(
                color: Colors.transparent,
                child: InkWell(
                  onTap: widget.onTap,
                  borderRadius: BorderRadius.circular(16),
                  child: widget.child,
                ),
              ),
            ),
          ),
        ),
      ),
    );

    if (widget.margin != null) {
      return Padding(
        padding: widget.margin!,
        child: content,
      );
    }
    return content;
  }
}
