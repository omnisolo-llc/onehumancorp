with open('srcs/app/lib/screens/meetings_screen.dart', 'r') as f:
    content = f.read()

content = "import 'dart:ui';\n" + content

old_card = """class _RoomCardState extends State<_RoomCard> {
  bool _joining = false;

  Color _statusColor(BuildContext context) {"""

new_card = """class _RoomCardState extends State<_RoomCard> {
  bool _joining = false;
  bool _isHovered = false;

  Color _statusColor(BuildContext context) {"""

content = content.replace(old_card, new_card)

old_build = """  @override
  Widget build(BuildContext context) {
    final room = widget.room;
    final participantCount = room['participant_count'] as int? ?? 0;
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Row(
          children: ["""

new_build = """  @override
  Widget build(BuildContext context) {
    final room = widget.room;
    final participantCount = room['participant_count'] as int? ?? 0;
    final colors = Theme.of(context).colorScheme;
    return Padding(
      padding: const EdgeInsets.only(bottom: 12),
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
                      ? colors.surfaceContainerHighest.withOpacity(0.3)
                      : colors.surface.withOpacity(0.1),
                  borderRadius: BorderRadius.circular(16),
                  border: Border.all(
                    color: _isHovered
                        ? colors.outlineVariant
                        : colors.outlineVariant.withOpacity(0.5),
                  ),
                ),
                child: Padding(
                  padding: const EdgeInsets.all(16),
                  child: Row(
                    children: ["""

content = content.replace(old_build, new_build)

old_close = """          ],
        ),
      ),
    );
  }
}"""

new_close = """                    ],
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}"""

content = content.replace(old_close, new_close)

with open('srcs/app/lib/screens/meetings_screen.dart', 'w') as f:
    f.write(content)
