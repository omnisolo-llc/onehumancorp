import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:powersync/powersync.dart';
import 'package:path/path.dart';
import 'package:path_provider/path_provider.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';

final powersyncServiceProvider = Provider<PowerSyncService>((ref) {
  final apiService = ref.watch(apiServiceProvider);
  final authService = ref.watch(authServiceProvider);
  final settings = ref.watch(clientSettingsProvider).valueOrNull;

  return PowerSyncService(apiService, authService, settings);
});

// Using a basic schema based on the handled rules in the Go backend
const schema = Schema([
  Table('users', [
    Column.text('username'),
    Column.text('email'),
    Column.text('roles'),
    Column.integer('active'),
    Column.text('organization_id'),
    Column.text('oidc_subject'),
    Column.text('created_at'),
    Column.text('updated_at'),
  ]),
  Table('agent_missions', [
    Column.text('status'),
    Column.text('payload'),
    Column.text('created_at'),
  ]),
  Table('agent_status', [
    Column.text('role'),
    Column.text('status'),
    Column.text('last_heartbeat'),
  ]),
]);

class PowerSyncService {
  final ApiService? apiService;
  final AuthService authService;
  final ClientSettings? settings;
  late PowerSyncDatabase db;
  bool _isInitialized = false;

  PowerSyncService(this.apiService, this.authService, this.settings);

  Future<void> initialize() async {
    if (_isInitialized) return;

    final dir = await getApplicationSupportDirectory();
    final path = join(dir.path, 'powersync-local.db');

    db = PowerSyncDatabase(schema: schema, path: path);
    await db.initialize();

    if (settings != null) {
      final backendUrl = settings!.backendUrl;
      final isStandalone = settings!.standaloneMode;

      // We only connect PowerSync to the cloud if we are in Cloud/Hybrid mode with a valid backend
      if (!isStandalone && backendUrl.isNotEmpty) {
        final connector = OHCBackendConnector(backendUrl, authService);
        db.connect(connector: connector);
      }
    }

    _isInitialized = true;
  }
}

class OHCBackendConnector extends PowerSyncBackendConnector {
  final String backendUrl;
  final AuthService authService;

  OHCBackendConnector(this.backendUrl, this.authService);

  @override
  Future<PowerSyncCredentials?> fetchCredentials() async {
    // In Riverpod context we should read current auth token, but authService doesn't have a sync currentUser.
    // Assuming token is available somehow, for now returning dummy token if missing.
    // In a real app we would pass the ProviderContainer or use Riverpod correctly.
    const token = "dummy-token"; // FIXME: Wire auth token correctly in production

    // In a real scenario, this endpoint would return PowerSync specific credentials
    // For now, we mock the JWKS and URL based on the OHC deployment.
    // The Docker Compose config exposes PowerSync on 8081.
    final uri = Uri.parse(backendUrl);
    final psUrl = '${uri.scheme}://${uri.host}:8081';

    return PowerSyncCredentials(
      endpoint: psUrl,
      token: token,
    );
  }

  @override
  Future<void> uploadData(PowerSyncDatabase database) async {
    final transaction = await database.getNextCrudTransaction();
    if (transaction == null) {
      return;
    }

    try {
      // In OHC, we'd loop through crud operations and use API service endpoints
      // to update the remote database.
      for (final op in transaction.crud) {
        // Mocking upload behavior
        print('Uploading ${op.op} for ${op.table} with ID ${op.id}');
      }

      await transaction.complete();
    } catch (e) {
      // Depending on the error, you might want to reject the transaction
      // or just rethrow to retry later.
      rethrow;
    }
  }
}
