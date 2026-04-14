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

    testWidgets('renders basic UI structure', (tester) async {
      when(() => mockCentrifugeService.subscribe('mesh:tasks'))
          .thenAnswer((_) => StreamController<CentrifugeMessage>.broadcast().stream);

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

      expect(find.text('Swarm Memory Mesh'), findsOneWidget);
      expect(find.text('Live Mesh Activity'), findsOneWidget);
      expect(find.text('Durable Swarm Memory'), findsOneWidget);
    });
  });
}
