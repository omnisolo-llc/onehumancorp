import 'dart:async';
import 'dart:io';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:http/http.dart' as http;
import 'package:ohc_app/services/auth_service.dart';

enum HealthStatus { healthy, unhealthy, down }

final healthProvider = AsyncNotifierProvider<HealthNotifier, HealthStatus>(() {
  return HealthNotifier();
});

class HealthNotifier extends AsyncNotifier<HealthStatus> {
  Timer? _timer;

  @override
  Future<HealthStatus> build() async {
    _startPolling();
    return _checkHealth();
  }

  void _startPolling() {
    if (Platform.environment.containsKey('FLUTTER_TEST')) return;

    _timer?.cancel();
    _timer = Timer.periodic(const Duration(seconds: 10), (timer) async {
      state = AsyncData(await _checkHealth());
    });
  }

  Future<HealthStatus> _checkHealth() async {
    final url = ref.read(backendUrlProvider);
    try {
      final response = await http.get(Uri.parse('$url/healthz')).timeout(
        const Duration(seconds: 3),
      );
      if (response.statusCode == 200) {
        return HealthStatus.healthy;
      }
      return HealthStatus.unhealthy;
    } catch (_) {
      return HealthStatus.down;
    }
  }

  void dispose() {
    _timer?.cancel();
  }
}
