import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/router.dart';
import 'package:ohc_app/services/powersync_service.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';

void main() {
  runApp(const ProviderScope(child: OhcApp()));
}

class OhcApp extends ConsumerWidget {
  const OhcApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final router = ref.watch(routerProvider);

    // Dynamic sync initialization for PowerSync when authenticated
    ref.listen<AsyncValue<AuthUser?>>(authStateProvider, (prev, next) {
      final user = next.valueOrNull;
      final settings = ref.read(clientSettingsProvider).valueOrNull;
      final powerSyncService = ref.read(powerSyncServiceProvider);

      if (user != null && settings != null && settings.runMode == RunMode.cloud) {
        // We initialize PowerSync dynamically only in Cloud Mode to bridge
        // Postgres -> local SQLite
        powerSyncService.init(settings.backendUrl, user.token);
      } else {
        powerSyncService.dispose();
      }
    });

    return MaterialApp.router(
      title: 'One Human Corp',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: const Color(0xFF6366F1), // indigo-500
          brightness: Brightness.light,
        ),
        useMaterial3: true,
        fontFamily: 'Inter',
      ),
      darkTheme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: const Color(0xFF6366F1),
          brightness: Brightness.dark,
        ),
        useMaterial3: true,
        fontFamily: 'Inter',
      ),
      themeMode: ThemeMode.system,
      routerConfig: router,
    );
  }
}
