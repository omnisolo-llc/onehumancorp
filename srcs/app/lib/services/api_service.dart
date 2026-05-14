import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';

class ApiService {
  static final Map<String, String> _mockMemory = {};

  Future<void> submitBusinessData(Map<String, dynamic> data) async {
    // Mock network delay
    await Future.delayed(const Duration(milliseconds: 500));
    print('Business data submitted successfully (Mock)');
  }

  Future<void> saveState(Map<String, dynamic> state, String tenantId) async {
    try {
      final prefs = await SharedPreferences.getInstance();
      await prefs.setString('wizard_state_$tenantId', json.encode(state));
    } catch (e) {
      print('Error saving state to shared_prefs, falling back to memory: $e');
      _mockMemory[tenantId] = json.encode(state);
    }
  }

  Future<Map<String, dynamic>?> getState(String tenantId) async {
    try {
      final prefs = await SharedPreferences.getInstance();
      final savedState = prefs.getString('wizard_state_$tenantId');
      if (savedState != null && savedState.isNotEmpty) {
        return json.decode(savedState);
      }
    } catch (e) {
      print('Error getting state from shared_prefs, falling back to memory: $e');
      final memState = _mockMemory[tenantId];
      if (memState != null && memState.isNotEmpty) {
        return json.decode(memState);
      }
    }
    return null;
  }
}
