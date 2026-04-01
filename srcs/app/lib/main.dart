import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/router.dart';
import 'package:ohc_app/services/powersync_service.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize PowerSync dynamically based on the current mode (Standalone vs Cloud-Native).
  final isStandalone = const bool.fromEnvironment('OHC_STANDALONE', defaultValue: false);
  final powerSyncService = PowerSyncService();
  try {
    await powerSyncService.init(isStandalone);
  } catch (e) {
    debugPrint("PowerSync initialization failed: $e");
  }

  runApp(
    ProviderScope(
      overrides: [
        powerSyncServiceProvider.overrideWithValue(powerSyncService),
      ],
      child: const OhcApp(),
    ),
  );
}

class OhcApp extends ConsumerWidget {
  const OhcApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final router = ref.watch(routerProvider);
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
