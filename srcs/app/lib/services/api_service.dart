import 'dart:convert';
import 'package:http/http.dart' as http;

class ApiService {
  final http.Client _client;

  ApiService({http.Client? client}) : _client = client ?? http.Client();

  Future<void> submitBusinessData(Map<String, dynamic> data) async {
    final url = Uri.parse('http://127.0.0.1:8080/api/onboarding/start');

    final goals = data['goals'] as List<dynamic>? ?? [];
    final description = 'Size: ${data['size']}, Goals: ${goals.join(", ")}';

    final payload = {
      'name': data['companyName'] ?? 'Unknown',
      'category': data['industry'] ?? 'Unknown',
      'description': description,
    };

    try {
      final response = await _client.post(
        url,
        headers: {'Content-Type': 'application/json'},
        body: json.encode(payload),
      );

      if (response.statusCode == 202) {
        print('Business data submitted successfully: $payload');
      } else {
        throw Exception('Failed to submit business data: ${response.statusCode}');
      }
    } catch (e) {
      print('Error submitting business data: $e');
      throw e;
    }
  }
}
