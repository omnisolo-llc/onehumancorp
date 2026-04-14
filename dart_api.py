with open("srcs/app/lib/services/api_service.dart", "r") as f:
    code = f.read()

new_method = """
  Future<void> trackSovereignToCloudInvite(String inviter, String assetId) async {
    final response = await http.post(
      Uri.parse('$baseUrl/api/growth/viral-bridge'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer $token',
      },
      body: jsonEncode({
        'inviter': inviter,
        'asset_id': assetId,
      }),
    );
    if (response.statusCode != 202) {
      throw Exception('Failed to track sovereign to cloud invite');
    }
  }
"""

if "trackSovereignToCloudInvite" not in code:
    code = code.replace("Future<void> createReferral", new_method + "\n  Future<void> createReferral")
    with open("srcs/app/lib/services/api_service.dart", "w") as f:
        f.write(code)
