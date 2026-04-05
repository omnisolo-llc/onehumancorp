import re

with open("srcs/app/lib/services/powersync_service.dart", "r") as f:
    content = f.read()

connector_class = """class _BackendConnector extends PowerSyncBackendConnector {
  final String backendUrl;
  final Ref ref;

  _BackendConnector({required this.backendUrl, required this.ref});

  @override
  Future<PowerSyncCredentials?> fetchCredentials() async {
    final user = ref.read(authStateProvider).valueOrNull;
    if (user == null) {
      return null;
    }

    try {
      final client = HttpClient();
      final request = await client.getUrl(Uri.parse('$backendUrl/api/auth/powersync/token'));
      request.headers.add('Authorization', 'Bearer ${user.token}');
      final response = await request.close();

      if (response.statusCode == 200) {
        final body = await response.transform(utf8.decoder).join();
        final data = json.decode(body);
        return PowerSyncCredentials(
          endpoint: backendUrl,
          token: data['token'],
        );
      }
    } catch (e) {
      // Fallback
    }

    return PowerSyncCredentials(
      endpoint: backendUrl,
      token: user.token,
    );
  }

  @override
  Future<void> uploadData(PowerSyncDatabase database) async {
    final batch = await database.getCrudBatch();
    if (batch == null) return;

    // Implement upload logic to cloud API if needed.
    await batch.complete();
  }
}
"""

content = re.sub(r'class _BackendConnector extends PowerSyncBackendConnector \{.*?\}', connector_class, content, flags=re.DOTALL)
content = "import 'dart:convert';\n" + content

with open("srcs/app/lib/services/powersync_service.dart", "w") as f:
    f.write(content)
