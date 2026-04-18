/// Stub implementation of the PowerSync package for Bazel builds.
/// Only the types used in this codebase are defined here.
library powersync;

import 'dart:async';

/// Represents a column definition in a PowerSync table.
class Column {
  final String name;
  final String type;

  const Column._(this.name, this.type);

  static Column text(String name) => Column._(name, 'TEXT');
  static Column integer(String name) => Column._(name, 'INTEGER');
  static Column real(String name) => Column._(name, 'REAL');
}

/// Represents an indexed column for a PowerSync index.
class IndexedColumn {
  final String column;
  const IndexedColumn(this.column);
}

/// Represents an index on a PowerSync table.
class Index {
  final String name;
  final List<IndexedColumn> columns;
  const Index(this.name, this.columns);
}

/// Represents a table definition in a PowerSync schema.
class Table {
  final String name;
  final List<Column> columns;
  final List<Index> indexes;

  const Table(this.name, this.columns, {this.indexes = const []});
}

/// Represents the schema for a PowerSync database.
class Schema {
  final List<Table> tables;
  const Schema(this.tables);
}

/// Credentials used to authenticate with a PowerSync backend.
class PowerSyncCredentials {
  final String endpoint;
  final String token;

  const PowerSyncCredentials({required this.endpoint, required this.token});
}

/// Abstract backend connector for PowerSync.
abstract class PowerSyncBackendConnector {
  Future<PowerSyncCredentials?> fetchCredentials();
  Future<void> uploadData(PowerSyncDatabase database);
}

/// Represents a PowerSync database instance.
class PowerSyncDatabase {
  PowerSyncDatabase({required Schema schema, required String path});

  Future<void> initialize() async {}

  Future<void> connect({required PowerSyncBackendConnector connector}) async {}

  void disconnect() {}

  Stream<List<Map<String, dynamic>>> watch(String query,
      {List<dynamic> parameters = const []}) {
    return const Stream.empty();
  }

  Future<List<Map<String, dynamic>>> getAll(String query,
      {List<dynamic> parameters = const []}) async {
    return [];
  }

  Future<void> execute(String query,
      [List<dynamic> parameters = const []]) async {}
}
