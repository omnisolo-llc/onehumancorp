import 'package:http/http.dart' as http;
import 'dart:convert';
import 'dart:io' show Platform;
import 'package:flutter/foundation.dart' show kIsWeb;

class ApiService {
  Future<void> submitBusinessData(Map<String, dynamic> data) async {
    String baseUrl = 'http://127.0.0.1:18789';
    if (!kIsWeb && Platform.isAndroid) {
        baseUrl = 'http://10.0.2.2:18789';
    } else if (const bool.hasEnvironment('API_URL')) {
        baseUrl = const String.fromEnvironment('API_URL');
    }
    final response = await http.post(
      Uri.parse('\$baseUrl/api/business'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode(data),
    );
    if (response.statusCode != 200) {
      throw Exception('Failed to submit business data');
    }
  }
}
