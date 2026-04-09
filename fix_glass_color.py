with open('srcs/app/lib/widgets/glass_card.dart', 'r') as f:
    content = f.read()

content = content.replace(
"""class GlassCard extends StatefulWidget {
  final Widget child;
  final VoidCallback? onTap;
  final EdgeInsetsGeometry? margin;""",
"""class GlassCard extends StatefulWidget {
  final Widget child;
  final VoidCallback? onTap;
  final EdgeInsetsGeometry? margin;
  final Color? color;"""
)

content = content.replace(
"""  const GlassCard({
    super.key,
    required this.child,
    this.onTap,
    this.margin,
  });""",
"""  const GlassCard({
    super.key,
    required this.child,
    this.onTap,
    this.margin,
    this.color,
  });"""
)

content = content.replace(
"""              decoration: BoxDecoration(
                color: _isHovered
                    ? colorScheme.surfaceContainerHighest.withValues(alpha: 0.3)
                    : colorScheme.surface.withValues(alpha: 0.1),""",
"""              decoration: BoxDecoration(
                color: widget.color != null
                    ? widget.color!.withValues(alpha: 0.03)
                    : (_isHovered
                        ? colorScheme.surfaceContainerHighest.withValues(alpha: 0.3)
                        : colorScheme.surface.withValues(alpha: 0.1)),"""
)

content = content.replace(
"""                border: Border.all(
                  color: _isHovered
                      ? colorScheme.outlineVariant
                      : colorScheme.outlineVariant.withValues(alpha: 0.5),
                ),""",
"""                border: Border.all(
                  color: widget.color != null
                      ? widget.color!.withValues(alpha: 0.08)
                      : (_isHovered
                          ? colorScheme.outlineVariant
                          : colorScheme.outlineVariant.withValues(alpha: 0.5)),
                ),"""
)

with open('srcs/app/lib/widgets/glass_card.dart', 'w') as f:
    f.write(content)
