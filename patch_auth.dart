<<<<<<< SEARCH
  Future<AuthUser> login(String email, String password) async {
=======
  Future<AuthUser> oauthLogin(String provider) async {
    final response = await _client.post(
      Uri.parse('$baseUrl/api/auth/oauth/login'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode({'provider': provider}),
    );
    if (response.statusCode == 200) {
      final data = jsonDecode(response.body) as Map<String, dynamic>;
      final token = data['token'] as String;
      final user = data['user'] as Map<String, dynamic>;
      return AuthUser.fromJson(user, token);
    }
    throw Exception('OAuth login failed: ${response.statusCode}');
  }

  Future<AuthUser> login(String email, String password) async {
>>>>>>> REPLACE
