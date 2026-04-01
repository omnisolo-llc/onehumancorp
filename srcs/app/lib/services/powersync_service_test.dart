import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/services/powersync_service.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:ohc_app/services/auth_service.dart';

class MockAuthService extends Mock implements AuthService {}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  late MockAuthService mockAuthService;

  setUp(() {
    mockAuthService = MockAuthService();
  });

  test('PowerSyncService skips init in cloud mode', () async {
    final settings = ClientSettings(
      backendUrl: 'http://localhost:8080',
      standaloneMode: false,
    );

    final service = PowerSyncService(
      settings: settings,
      authService: mockAuthService,
    );

    await service.init();
    expect(service.db, isNull);
  });
}
