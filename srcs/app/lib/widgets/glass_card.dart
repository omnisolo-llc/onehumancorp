import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final VoidCallback? onTap;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final double? maxWidth;
  final Color? baseColor;
  final bool animateEntrance;
  final int entranceDelayMs;

  const GlassCard({
    super.key,
    required this.child,
    this.onTap,
    this.padding = const EdgeInsets.all(24),
    this.margin,
    this.maxWidth,
    this.baseColor,
    this.animateEntrance = false,
    this.entranceDelayMs = 0,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> with SingleTickerProviderStateMixin {
  bool _isHovered = false;
  late AnimationController _controller;
  late Animation<double> _fadeAnimation;
  late Animation<Offset> _slideAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 600),
    );
    _fadeAnimation = Tween<double>(begin: widget.animateEntrance ? 0.0 : 1.0, end: 1.0)
        .animate(CurvedAnimation(parent: _controller, curve: Curves.easeOut));
    _slideAnimation = Tween<Offset>(
      begin: widget.animateEntrance ? const Offset(0, 0.2) : Offset.zero,
      end: Offset.zero,
    ).animate(CurvedAnimation(parent: _controller, curve: Curves.easeOutQuart));

    if (widget.animateEntrance) {
      Future.delayed(Duration(milliseconds: widget.entranceDelayMs), () {
        if (mounted) _controller.forward();
      });
    } else {
      _controller.value = 1.0;
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final effectiveColor = widget.baseColor ?? colors.surface;

    // OHC Glassmorphism specific color matrix
    final colorMatrix = <double>[
      1.168, -0.153, -0.015, 0, 0,
      -0.046, 1.061, -0.015, 0, 0,
      -0.046, -0.152, 1.198, 0, 0,
      0, 0, 0, 1, 0,
    ];

    Widget content = AnimatedContainer(
      duration: const Duration(milliseconds: 300),
      padding: widget.padding,
      decoration: BoxDecoration(
        color: _isHovered
            ? effectiveColor.withValues(alpha: 0.15)
            : effectiveColor.withValues(alpha: 0.08),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: _isHovered
              ? colors.outlineVariant
              : colors.outlineVariant.withValues(alpha: 0.5),
        ),
      ),
      child: widget.child,
    );

    if (widget.onTap != null) {
      content = Material(
        color: Colors.transparent,
        child: InkWell(
          borderRadius: BorderRadius.circular(16),
          onTap: widget.onTap,
          splashColor: effectiveColor.withValues(alpha: 0.1),
          highlightColor: effectiveColor.withValues(alpha: 0.05),
          child: content,
        ),
      );
    }

    return FadeTransition(
      opacity: _fadeAnimation,
      child: SlideTransition(
        position: _slideAnimation,
        child: Container(
          margin: widget.margin,
          child: ConstrainedBox(
            constraints: widget.maxWidth != null ? BoxConstraints(maxWidth: widget.maxWidth!) : const BoxConstraints(),
            child: MouseRegion(
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
                      outer: ColorFilter.matrix(colorMatrix),
                      inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                    ),
                    child: content,
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
