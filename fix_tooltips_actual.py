import os
import re

for root, dirs, files in os.walk('srcs/app/lib'):
    for file in files:
        if file.endswith('.dart'):
            filepath = os.path.join(root, file)
            with open(filepath, 'r') as f:
                content = f.read()

            content = content.replace("IconButton(\n            onPressed: _refresh,\n            icon: const Icon(Icons.refresh),\n          )", "IconButton(\n            onPressed: _refresh,\n            icon: const Icon(Icons.refresh),\n            tooltip: 'Refresh',\n          )")
            content = content.replace("IconButton(\n            icon: const Icon(Icons.meeting_room),\n            onPressed: () => _showRoomPicker(context),\n          )", "IconButton(\n            icon: const Icon(Icons.meeting_room),\n            tooltip: 'Switch room',\n            onPressed: () => _showRoomPicker(context),\n          )")

            content = content.replace("IconButton(onPressed: _refresh, icon: const Icon(Icons.refresh))", "IconButton(onPressed: _refresh, icon: const Icon(Icons.refresh), tooltip: 'Refresh')")

            content = content.replace("trailing: IconButton(\n                  icon: const Icon(Icons.delete_outline, color: Colors.red),\n                  onPressed: () => _handleDeleteUser(user.id),\n                )", "trailing: IconButton(\n                  icon: const Icon(Icons.delete_outline, color: Colors.red),\n                  tooltip: 'Delete user',\n                  onPressed: () => _handleDeleteUser(user.id),\n                )")

            content = content.replace("IconButton(\n                                icon: const Icon(Icons.open_in_new, size: 16),\n                                onPressed: () {}, // Link preview\n                              )", "IconButton(\n                                icon: const Icon(Icons.open_in_new, size: 16),\n                                tooltip: 'Open staging URL',\n                                onPressed: () {}, // Link preview\n                              )")

            content = content.replace("IconButton(\n            icon: const Icon(Icons.refresh),\n            onPressed: () => ref.invalidate(_securityProvider),\n          )", "IconButton(\n            icon: const Icon(Icons.refresh),\n            tooltip: 'Re-scan',\n            onPressed: () => ref.invalidate(_securityProvider),\n          )")

            content = content.replace("IconButton(\n            icon: const Icon(Icons.refresh),\n            onPressed: () => ref.invalidate(_logsProvider(_lines)),\n          )", "IconButton(\n            icon: const Icon(Icons.refresh),\n            tooltip: 'Refresh logs',\n            onPressed: () => ref.invalidate(_logsProvider(_lines)),\n          )")

            content = content.replace("IconButton(\n                  icon: const Icon(Icons.edit_outlined),\n                  onPressed: () => _showEditKeyDialog(context),\n                )", "IconButton(\n                  icon: const Icon(Icons.edit_outlined),\n                  tooltip: 'Edit API key',\n                  onPressed: () => _showEditKeyDialog(context),\n                )")

            content = content.replace("leading: IconButton(\n          icon: const Icon(Icons.close),\n          onPressed: () => context.go('/agents'),\n        )", "leading: IconButton(\n          icon: const Icon(Icons.close),\n          tooltip: 'Close wizard',\n          onPressed: () => context.go('/agents'),\n        )")

            content = content.replace("trailing: IconButton(\n                icon: const Icon(Icons.edit),\n                onPressed: () => _editBackendUrl(context, ref, settings.backendUrl),\n              )", "trailing: IconButton(\n                icon: const Icon(Icons.edit),\n                tooltip: 'Edit Backend URL',\n                onPressed: () => _editBackendUrl(context, ref, settings.backendUrl),\n              )")

            content = content.replace("suffixIcon: IconButton(\n                icon: Icon(\n                  _obscureKey ? Icons.visibility : Icons.visibility_off,\n                ),\n                onPressed: () {\n                  setState(() {\n                    _obscureKey = !_obscureKey;\n                  });\n                },\n              )", "suffixIcon: IconButton(\n                icon: Icon(\n                  _obscureKey ? Icons.visibility : Icons.visibility_off,\n                ),\n                tooltip: _obscureKey ? 'Show API Key' : 'Hide API Key',\n                onPressed: () {\n                  setState(() {\n                    _obscureKey = !_obscureKey;\n                  });\n                },\n              )")

            with open(filepath, 'w') as f:
                f.write(content)
