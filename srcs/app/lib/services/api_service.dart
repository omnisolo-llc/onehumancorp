import 'dart:convert';
import 'package:http/http.dart' as http;

class ApiService {
  Future<void> submitBusinessData(Map<String, dynamic> data) async {
    try {
      final url = Uri.parse('http://127.0.0.1:18789/api/onboarding/start');
      final response = await http.post(
        url,
        headers: {'Content-Type': 'application/json'},
        body: json.encode(data),
      );

      if (response.statusCode >= 200 && response.statusCode < 300) {
        print('Business data submitted successfully: $data');
      } else {
        print('Failed to submit business data. Status code: ${response.statusCode}');
      }
    } catch (e) {
      print('Error submitting business data: $e');
    }
  }
}
