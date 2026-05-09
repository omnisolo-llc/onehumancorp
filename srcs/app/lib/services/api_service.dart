class ApiService {
  static Map<String, dynamic>? _savedState;

  Future<void> submitBusinessData(Map<String, dynamic> data) async {
    // Simulate network delay
    await Future.delayed(const Duration(seconds: 2));
    print('Business data submitted: $data');
    _savedState = data;
  }

  void saveWizardState(Map<String, dynamic> data) {
    _savedState = data;
  }

  Map<String, dynamic>? getWizardState() {
    return _savedState;
  }
}
