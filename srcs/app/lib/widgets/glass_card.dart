import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final VoidCallback? onTap;
  final EdgeInsetsGeometry padding;
  final EdgeInsetsGeometry? margin;

  const GlassCard({
    super.key,
    required this.child,
    this.onTap,
    this.padding = EdgeInsets.zero,
    this.margin,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    Widget cardContent = Semantics(
      button: widget.onTap != null,
      child: MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        child: AnimatedScale(
          scale: _isHovered ? 1.02 : 1.0,
          duration: const Duration(milliseconds: 200),
          curve: Curves.easeOutCubic,
          child: AnimatedContainer(
            duration: const Duration(milliseconds: 300),
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(16),
              boxShadow: [
                if (_isHovered)
                  BoxShadow(
                    color: Colors.black.withValues(alpha: 0.2),
                    blurRadius: 10,
                    offset: const Offset(0, 4),
                  )
                else
                  BoxShadow(
                    color: Colors.black.withValues(alpha: 0.1),
                    blurRadius: 4,
                    offset: const Offset(0, 2),
                  ),
              ],
            ),
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
                    color: _isHovered
                        ? colorScheme.surfaceContainerHighest.withValues(alpha: 0.3)
                        : colorScheme.surface.withValues(alpha: 0.1),
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(
                      color: _isHovered
                          ? colorScheme.outlineVariant
                          : colorScheme.outlineVariant.withValues(alpha: 0.5),
                    ),
                  ),
                  child: Material(
                    color: Colors.transparent,
                    child: InkWell(
                      borderRadius: BorderRadius.circular(16),
                      onTap: widget.onTap,
                      child: Padding(
                        padding: widget.padding,
                        child: widget.child,
                      ),
                    ),
                  ),
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
        child: cardContent,
      );
    }
    return cardContent;
  }
}
