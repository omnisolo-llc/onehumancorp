import 'dart:convert';
import 'package:http/http.dart' as http;

class ApiService {
  Future<void> submitBusinessData(Map<String, dynamic> data) async {
    try {
      final response = await http.post(
        Uri.parse('http://127.0.0.1:8080/api/onboarding/start'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'name': data['companyName'] ?? 'Unknown',
          'category': data['industry'] ?? 'Unknown',
          'description': data['productDescription'] ?? '',
        }),
      );
      print('Business data submitted: ${response.statusCode}');
    } catch (e) {
      print('Error submitting data: $e');
    }
  }
}
