import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/services/api_service.dart';

final dashboardProvider = FutureProvider.autoDispose<DashboardSnapshot>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) throw Exception('API not available');
  return api.getDashboard();
});
