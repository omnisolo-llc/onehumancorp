import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/router.dart';
import 'package:ohc_app/services/powersync_service.dart';
import 'package:ohc_app/widgets/undercover_mode_toggle.dart';

void main() {
  runApp(const ProviderScope(child: OhcApp()));
}

class OhcApp extends ConsumerWidget {
  const OhcApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // Watch powersync to trigger initialization
    ref.watch(powersyncProvider);

    final router = ref.watch(routerProvider);
    final isUndercover = ref.watch(undercoverModeProvider);

    // In Undercover Mode, we override the theme to be highly saturated and high-contrast
    // to match the "Glassmorphism, High-Saturate Blurs" mandate across the app.
    final seedColor = isUndercover ? const Color(0xFFFF0055) : const Color(0xFF6366F1);

    return MaterialApp.router(
      title: 'One Human Corp',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: seedColor,
          brightness: Brightness.light,
        ),
        useMaterial3: true,
        fontFamily: 'Inter',
      ),
      darkTheme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: seedColor,
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
