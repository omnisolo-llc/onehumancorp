import 'dart:convert';
import 'package:http/http.dart' as http;

class ApiService {
  final http.Client? client;

  ApiService({this.client});

  Future<void> submitBusinessData(Map<String, dynamic> data) async {
    final url = Uri.parse('http://localhost:8080/api/v1/business');
    final httpClient = client ?? http.Client();
    try {
      final response = await httpClient.post(
        url,
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode(data),
      );
      if (response.statusCode != 200 && response.statusCode != 201) {
        throw Exception('Failed to submit business data: ${response.statusCode}');
      }
    } finally {
      if (client == null) {
        httpClient.close();
      }
    }
  }
}
