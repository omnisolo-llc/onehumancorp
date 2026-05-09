class ApiService {
  Future<void> submitBusinessData(Map<String, dynamic> data) async {
    // Simulate network delay
    await Future.delayed(const Duration(seconds: 2));
    print('Business data submitted: $data');
  }

  Future<String> generateDescription(String name) async {
    return "Generated description for $name";
  }

  Future<void> saveState(dynamic state) async {
    print("State saved to backend");
  }
}
