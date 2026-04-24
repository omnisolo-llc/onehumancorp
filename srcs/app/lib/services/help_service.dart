import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/api_service.dart';

final helpServiceProvider = Provider((ref) {
  final api = ref.watch(apiServiceProvider);
  return HelpService(api);
});

class HelpService {
  final ApiService? _api;

  HelpService(this._api);

  Future<Map<String, String>> fetchTooltips() async {
    if (_api == null) return {};
    try {
      final data = await _api!.fetchTooltips();
      return data.map((key, value) => MapEntry(key, value['description'] as String));
    } catch (e) {
      return {};
    }
  }

  Future<Map<String, dynamic>> getHelpChatReply(String message) async {
    if (_api == null) return {'reply': 'API not available', 'link': ''};
    try {
      return await _api!.getHelpChatReply(message);
    } catch (e) {
      return {'reply': 'Connection error', 'link': ''};
    }
  }
}

final tooltipsProvider = FutureProvider<Map<String, String>>((ref) async {
  return ref.watch(helpServiceProvider).fetchTooltips();
});
