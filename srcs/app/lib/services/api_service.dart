import 'dart:convert';
import 'package:http/http.dart' as http;

class ApiService {
  Future<void> submitBusinessData(Map<String, dynamic> data) async {
    final response = await http.post(
      Uri.parse('/api/onboarding/start'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode(data),
    );
    if (response.statusCode >= 200 && response.statusCode < 300) {
      print('Business data submitted successfully: $data');
    } else {
      print('Failed to submit business data: ${response.statusCode}');
    }
  }
}
