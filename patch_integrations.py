import re

with open('srcs/app/lib/screens/integrations_screen.dart', 'r') as f:
    content = f.read()

# Make sure dart:ui is imported
if "import 'dart:ui';" not in content:
    content = content.replace("import 'package:flutter/material.dart';", "import 'package:flutter/material.dart';\nimport 'dart:ui';")


# --- Refactor _IntegrationCard ---
search_card = """    return Semantics(
      label: 'Connect to ${widget.title}. ${widget.subtitle}',
      button: true,
      child: Card(
        child: InkWell(
          onTap: widget.onConnect,
          borderRadius: BorderRadius.circular(12),
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Container(
                      padding: const EdgeInsets.all(8),
                      decoration: BoxDecoration(
                        color: widget.color.withValues(alpha: 0.1),
                        borderRadius: BorderRadius.circular(8),
                      ),
                      child: Icon(widget.icon, color: widget.color, size: 24),
                    ),
                    const Spacer(),
                    Text(
                      'Inactive',
                      style: TextStyle(
                        fontSize: 10,
                        color: Theme.of(
                          context,
                        ).colorScheme.onSurfaceVariant.withValues(alpha: 0.7),
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 16),
                Text(
                  widget.title,
                  style: const TextStyle(
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  widget.subtitle,
                  style: TextStyle(
                    fontSize: 12,
                    color: colors.onSurfaceVariant,
                  ),
                ),
                const SizedBox(height: 24),
                SizedBox(
                  width: double.infinity,
                  child: OutlinedButton(
                    onPressed: widget.onConnect,
                    child: const Text('Configure'),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );"""

replace_card = """    return Semantics(
      label: 'Connect to ${widget.title}. ${widget.subtitle}',
      button: true,
      child: MouseRegion(
        onEnter: (_) => setState(() => _hovering = true),
        onExit: (_) => setState(() => _hovering = false),
        child: AnimatedScale(
          scale: _hovering ? 1.02 : 1.0,
          duration: const Duration(milliseconds: 200),
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
                  color: _hovering
                      ? colors.surfaceContainerHighest.withValues(alpha: 0.3)
                      : colors.surface.withValues(alpha: 0.1),
                  borderRadius: BorderRadius.circular(16),
                  border: Border.all(
                    color: _hovering
                        ? colors.outlineVariant
                        : colors.outlineVariant.withValues(alpha: 0.5),
                  ),
                ),
                child: InkWell(
                  onTap: widget.onConnect,
                  borderRadius: BorderRadius.circular(16),
                  child: Padding(
                    padding: const EdgeInsets.all(24),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            Container(
                              padding: const EdgeInsets.all(8),
                              decoration: BoxDecoration(
                                color: widget.color.withValues(alpha: 0.1),
                                borderRadius: BorderRadius.circular(8),
                              ),
                              child: Icon(widget.icon, color: widget.color, size: 24),
                            ),
                            const Spacer(),
                            Text(
                              'Inactive',
                              style: TextStyle(
                                fontSize: 10,
                                color: colors.onSurfaceVariant.withValues(alpha: 0.7),
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 16),
                        Text(
                          widget.title,
                          style: const TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          widget.subtitle,
                          style: TextStyle(
                            fontSize: 12,
                            color: colors.onSurfaceVariant,
                          ),
                        ),
                        const SizedBox(height: 24),
                        SizedBox(
                          width: double.infinity,
                          child: OutlinedButton(
                            onPressed: widget.onConnect,
                            child: const Text('Configure'),
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );"""

content = content.replace(search_card, replace_card)

# --- Refactor _MCPToolTile ---
search_mcp = """class _MCPToolTile extends StatelessWidget {
  final Map<String, dynamic> tool;

  const _MCPToolTile({required this.tool});

  @override
  Widget build(BuildContext context) {
    final name = tool['name'] as String? ?? 'Unknown Tool';
    final description = tool['description'] as String? ?? '';

    return Semantics(
      label: 'Invoke MCP Tool: $name. $description',
      button: true,
      excludeSemantics: true,
      child: Card(
        margin: const EdgeInsets.only(bottom: 12),
        child: InkWell(
          borderRadius: BorderRadius.circular(12),
          onTap: () {},
          child: ListTile(
            leading: const Icon(Icons.build_circle_outlined),
            title: Text(name),
            subtitle: Text(
              description,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
            ),
            trailing: OutlinedButton(
              onPressed: () {}, // Invoke dialog
              child: const Text('Invoke'),
            ),
          ),
        ),
      ),
    );
  }
}"""

replace_mcp = """class _MCPToolTile extends StatefulWidget {
  final Map<String, dynamic> tool;

  const _MCPToolTile({required this.tool});

  @override
  State<_MCPToolTile> createState() => _MCPToolTileState();
}

class _MCPToolTileState extends State<_MCPToolTile> {
  bool _hovering = false;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final name = widget.tool['name'] as String? ?? 'Unknown Tool';
    final description = widget.tool['description'] as String? ?? '';

    return Semantics(
      label: 'Invoke MCP Tool: $name. $description',
      button: true,
      excludeSemantics: true,
      child: MouseRegion(
        onEnter: (_) => setState(() => _hovering = true),
        onExit: (_) => setState(() => _hovering = false),
        child: AnimatedScale(
          scale: _hovering ? 1.02 : 1.0,
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
                    color: _hovering
                        ? colors.surfaceContainerHighest.withValues(alpha: 0.3)
                        : colors.surface.withValues(alpha: 0.1),
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(
                      color: _hovering
                          ? colors.outlineVariant
                          : colors.outlineVariant.withValues(alpha: 0.5),
                    ),
                  ),
                  child: InkWell(
                    borderRadius: BorderRadius.circular(16),
                    onTap: () {},
                    child: ListTile(
                      contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                      leading: const Icon(Icons.build_circle_outlined),
                      title: Text(name),
                      subtitle: Text(
                        description,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                      ),
                      trailing: OutlinedButton(
                        onPressed: () {}, // Invoke dialog
                        child: const Text('Invoke'),
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
  }
}"""

content = content.replace(search_mcp, replace_mcp)

with open('srcs/app/lib/screens/integrations_screen.dart', 'w') as f:
    f.write(content)
