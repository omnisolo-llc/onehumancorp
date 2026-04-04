import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/swarm_memory_screen.dart';
import 'package:ohc_app/services/centrifuge_service.dart';
import 'package:ohc_app/services/powersync_service.dart';
import 'package:powersync/powersync.dart';
import 'package:sqlite3/sqlite3.dart' as sqlite; // Need to create a real ResultSet if possible or use a mock.

class MockCentrifugeService extends Mock implements CentrifugeService {}
class MockPowerSyncService extends Mock implements PowerSyncService {}
class MockPowerSyncDatabase extends Mock implements PowerSyncDatabase {}

// We can create a real ResultSet using `sqlite.ResultSet`. Wait, `ResultSet` takes `columnNames`, `tableNames`, `rows`.
// ResultSet constructor in sqlite3: ResultSet(List<String> columnNames, List<String>? tableNames, List<List<Object?>> rows)
// Let's check how to construct it or use a fake.
// Another option is to use a fake object that extends `sqlite.ResultSet` is not possible because it's final.
// However, the `powersyncProvider` return is mocked, and `watch` returns a Stream<sqlite.ResultSet>.
// But wait, the previous code had `List<dynamic>` when `snapshot.data` was read:
// `final rows = snapshot.data ?? [];`
// `final row = rows[index];`
// `final value = (row is Map) ? row['value'] as String? : (row as dynamic).read('value') as String?;`
// If we just return `Stream<dynamic>.value( [ {'value': 'x', 'updated_at': 'y'} ] )`, it will compile if we use a dynamic mock.
// Wait, `watch` method signature in `PowerSyncDatabase` is `Stream<sqlite.ResultSet> watch(String sql, {List<Object?> parameters = const []})`.
// So mocktail enforces the return type.
// We must return a `sqlite.ResultSet`.

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
          .thenAnswer((_) => const Stream.empty());

      // Try constructing a real ResultSet with empty data
      final emptyResultSet = sqlite.ResultSet([], null, []);
      when(() => mockPowerSyncDatabase.watch(any()))
          .thenAnswer((_) => Stream.value(emptyResultSet));

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
    });

    testWidgets('shows loading state for durable memory when db is null', (tester) async {
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
      when(() => mockCentrifugeService.subscribe('mesh:tasks'))
          .thenAnswer((_) => const Stream.empty());

      final emptyResultSet = sqlite.ResultSet([], null, []);
      when(() => mockPowerSyncDatabase.watch(any()))
          .thenAnswer((_) => Stream.value(emptyResultSet));

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

      await tester.pumpAndSettle();

      expect(find.text('No memories found.'), findsOneWidget);
    });

    testWidgets('shows animated memory cards when data is present', (tester) async {
      when(() => mockCentrifugeService.subscribe('mesh:tasks'))
          .thenAnswer((_) => Stream.value(
                CentrifugeMessage(
                    id: 'task_123',
                    channelId: 'mesh:tasks',
                    authorId: 'agent_1',
                    authorName: 'Agent Alpha',
                    body: 'Completed task X',
                    sentAt: DateTime.now(),
                ),
              ));

      final resultSet = sqlite.ResultSet(
        ['value', 'updated_at'],
        null,
        [
          ['Stored memory of task X', '2023-10-01 10:00']
        ]
      );
      when(() => mockPowerSyncDatabase.watch(any()))
          .thenAnswer((_) => Stream.value(resultSet));

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

      await tester.pumpAndSettle();

      // Check for live mesh message
      expect(find.text('Agent Alpha'), findsOneWidget);
      expect(find.text('Completed task X'), findsOneWidget);

      // Check for durable memory
      expect(find.text('Stored memory of task X'), findsOneWidget);
      expect(find.text('2023-10-01 10:00'), findsOneWidget);

      // Verify that AnimatedScale and MouseRegion widgets are present
      expect(find.byType(AnimatedScale), findsWidgets);
      expect(find.byType(MouseRegion), findsWidgets);
    });
  });
}
