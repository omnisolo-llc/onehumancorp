import re

with open('srcs/app/lib/screens/channels_screen.dart', 'r') as f:
    content = f.read()

search_str = """class _ChannelCard extends StatelessWidget {
  final ChatChannel channel;
  const _ChannelCard({required this.channel});

  String _icon() {
    for (final def in _channelDefs) {
      if (def.type == channel.backend.type) return def.icon;
    }
    return '💬';
  }

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      child: ListTile(
        leading: Text(_icon(), style: const TextStyle(fontSize: 28)),
        title: Text(
          channel.name,
          style: const TextStyle(fontWeight: FontWeight.w600),
        ),
        subtitle: Text(channel.backend.displayName),
        trailing: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Chip(
              label: Text(channel.enabled ? 'Enabled' : 'Disabled'),
              backgroundColor:
                  channel.enabled
                      ? Theme.of(context).colorScheme.secondaryContainer
                      : Theme.of(context).colorScheme.surfaceContainerHighest,
            ),
          ],
        ),
      ),
    );
  }
}"""

replace_str = """import 'dart:ui';

class _ChannelCard extends StatefulWidget {
  final ChatChannel channel;
  const _ChannelCard({required this.channel});

  @override
  State<_ChannelCard> createState() => _ChannelCardState();
}

class _ChannelCardState extends State<_ChannelCard> {
  bool _isHovered = false;

  String _icon() {
    for (final def in _channelDefs) {
      if (def.type == widget.channel.backend.type) return def.icon;
    }
    return '💬';
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        child: Padding(
          padding: const EdgeInsets.only(bottom: 12),
          child: ClipRRect(
            borderRadius: BorderRadius.circular(16),
            child: BackdropFilter(
              filter: ImageFilter.compose(
                outer: const ColorFilter.matrix(<double>[
                  1.168, -0.153, -0.015, 0, 0,
                  -0.046, 1.061, -0.015, 0, 0,
                  -0.046, -0.152, 1.198, 0, 0,
                  0, 0, 0, 1, 0,
                ]),
                inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
              ),
              child: AnimatedContainer(
                duration: const Duration(milliseconds: 200),
                decoration: BoxDecoration(
                  color: _isHovered
                      ? colors.surfaceContainerHighest.withValues(alpha: 0.3)
                      : colors.surface.withValues(alpha: 0.1),
                  borderRadius: BorderRadius.circular(16),
                  border: Border.all(
                    color: _isHovered
                        ? colors.outlineVariant
                        : colors.outlineVariant.withValues(alpha: 0.5),
                  ),
                ),
                child: ListTile(
                  contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                  leading: Text(_icon(), style: const TextStyle(fontSize: 28)),
                  title: Text(
                    widget.channel.name,
                    style: const TextStyle(fontWeight: FontWeight.w600),
                  ),
                  subtitle: Text(widget.channel.backend.displayName),
                  trailing: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Chip(
                        label: Text(widget.channel.enabled ? 'Enabled' : 'Disabled'),
                        backgroundColor:
                            widget.channel.enabled
                                ? colors.secondaryContainer
                                : colors.surfaceContainerHighest,
                      ),
                    ],
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

# Manually import dart:ui if not present
if "import 'dart:ui';" not in content:
    content = content.replace("import 'package:flutter/material.dart';", "import 'package:flutter/material.dart';\nimport 'dart:ui';")

new_content = content.replace(search_str, replace_str)
with open('srcs/app/lib/screens/channels_screen.dart', 'w') as f:
    f.write(new_content)
