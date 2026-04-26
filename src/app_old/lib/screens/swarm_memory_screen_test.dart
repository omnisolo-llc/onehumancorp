import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/swarm_memory_screen.dart';
import 'package:ohc_app/services/centrifuge_service.dart';
import 'package:ohc_app/services/powersync_service.dart';
import 'package:powersync/powersync.dart';

class MockCentrifugeService extends Mock implements CentrifugeService {}
class MockPowerSyncService extends Mock implements PowerSyncService {}
class MockPowerSyncDatabase extends Mock implements PowerSyncDatabase {}

void main() {
  group('SwarmMemoryScreen Widget Tests', () {
    late MockCentrifugeService mockCentrifugeService;
    late MockPowerSyncService mockPowerSyncService;
    late MockPowerSyncDatabase mockPowerSyncDatabase;

    setUp(() {
      mockCentrifugeService = MockCentrifugeService();
      mockPowerSyncService = MockPowerSyncService();
      mockPowerSyncDatabase = MockPowerSyncDatabase();

      when(() => mockPowerSyncService.db).thenReturn(mockPowerSyncDatabase);
    });

    testWidgets('renders basic UI structure with Glassmorphism', (tester) async {
      tester.view.physicalSize = const Size(2400, 1600);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      when(() => mockCentrifugeService.subscribe('mesh:tasks'))
          .thenAnswer((_) => const Stream.empty());

      when(() => mockPowerSyncDatabase.watch(any()))
          .thenAnswer((_) => const Stream.empty());

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            centrifugeServiceProvider.overrideWithValue(mockCentrifugeService),
            powersyncProvider.overrideWithValue(mockPowerSyncService),
          ],
          child: const MaterialApp(
            home: SwarmMemoryScreen(),
          ),
        ),
      );

      // Verify Screen title
      expect(find.text('Swarm Memory Mesh'), findsOneWidget);
      expect(find.text('Live Mesh Activity'), findsOneWidget);
      expect(find.text('Durable Swarm Memory'), findsOneWidget);
      expect(find.text('AutoDream Pipelines'), findsOneWidget);

      // Verify Scaffold and AppBar are transparent for Glassmorphism
      final scaffold = tester.widget<Scaffold>(find.byType(Scaffold));
      expect(scaffold.backgroundColor, Colors.transparent);

      final appBar = tester.widget<AppBar>(find.byType(AppBar));
      expect(appBar.backgroundColor, Colors.transparent);
    });

    testWidgets('shows loading state for durable memory when db is null', (tester) async {
      tester.view.physicalSize = const Size(2400, 1600);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      when(() => mockCentrifugeService.subscribe('mesh:tasks'))
          .thenAnswer((_) => const Stream.empty());

      when(() => mockPowerSyncService.db).thenReturn(null);

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            centrifugeServiceProvider.overrideWithValue(mockCentrifugeService),
            powersyncProvider.overrideWithValue(mockPowerSyncService),
          ],
          child: const MaterialApp(
            home: SwarmMemoryScreen(),
          ),
        ),
      );

      expect(find.byType(CircularProgressIndicator), findsOneWidget);
    });

    testWidgets('shows empty state for durable memory', (tester) async {
      tester.view.physicalSize = const Size(2400, 1600);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      when(() => mockCentrifugeService.subscribe('mesh:tasks'))
          .thenAnswer((_) => const Stream.empty());

      when(() => mockPowerSyncDatabase.watch(any()))
          .thenAnswer((_) => const Stream.empty());

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            centrifugeServiceProvider.overrideWithValue(mockCentrifugeService),
            powersyncProvider.overrideWithValue(mockPowerSyncService),
          ],
          child: const MaterialApp(
            home: SwarmMemoryScreen(),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('No memories found.'), findsOneWidget);
    });
  });
}
