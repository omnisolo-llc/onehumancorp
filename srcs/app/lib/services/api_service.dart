class ApiService {
  Future<void> submitBusinessData(Map<String, dynamic> data) async {
    // Simulate network delay
    await Future.delayed(const Duration(seconds: 2));
    print('Business data submitted: $data');
  }

  Future<void> saveState(Map<String, dynamic> state) async {
    await Future.delayed(const Duration(milliseconds: 100));
  }

  Future<Map<String, dynamic>> getState() async {
    await Future.delayed(const Duration(milliseconds: 100));
    return {};
  }
}
