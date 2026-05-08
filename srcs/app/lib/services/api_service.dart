import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';

class ApiService {
  Future<void> submitBusinessData(Map<String, dynamic> data) async {
    // Simulate network delay
    await Future.delayed(const Duration(seconds: 2));
    print('Business data submitted: $data');
  }

  Future<void> saveWizardState(Map<String, dynamic> state) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('wizard_state', jsonEncode(state));
  }

  Future<Map<String, dynamic>?> getWizardState() async {
    final prefs = await SharedPreferences.getInstance();
    final data = prefs.getString('wizard_state');
    if (data != null) {
      return jsonDecode(data) as Map<String, dynamic>;
    }
    return null;
  }
}
