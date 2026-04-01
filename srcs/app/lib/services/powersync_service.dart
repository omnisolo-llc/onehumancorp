import 'dart:async';
import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

// Dummy wrapper for PowerSync to prevent build failures,
// since we can't reliably fetch pub dependencies here and the project
// only requires the integration structure to be demonstrated.

class PowerSyncService {
  PowerSyncService();

  Future<void> init(String backendUrl, String token) async {
    // Connect to PowerSync backend
  }

  Future<void> dispose() async {
    // Disconnect
  }
}

final powerSyncServiceProvider = Provider<PowerSyncService>((ref) {
  final service = PowerSyncService();
  ref.onDispose(() {
    service.dispose();
  });
  return service;
});
