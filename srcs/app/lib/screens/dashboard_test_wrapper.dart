import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/models/agent.dart';
import 'package:ohc_app/models/organization.dart';
import 'package:ohc_app/services/api_service.dart';

class MockApi implements ApiService {
  @override
  Future<DashboardSnapshot> getDashboard() async {
    return DashboardSnapshot(
      agents: [
        Agent(id: '1', name: 'Alpha', role: 'Developer', isRunning: true, memoryUsageBytes: 1000, cpuUsagePercent: 50.0, startedAt: DateTime.now(), lastHeartbeat: DateTime.now()),
      ],
      organization: Organization(
        id: 'org1',
        name: 'Test Org',
        members: [
          Member(id: 'm1', name: 'User 1', email: 'test@test.com', role: 'Admin', isHuman: true, joinedAt: DateTime.now()),
        ]
      ),
      meetings: [],
      channels: [],
      pipelines: [],
      skills: [],
      statuses: [
        MissionStatus(missionId: 'm1', status: 'running', heartbeat: DateTime.now(), currentTask: 'test', memoryUsage: 1000, metrics: {})
      ]
    );
  }

  @override
  Future<void> scaleAgents(String role, int targetCount) async {}
}

void main() {
  runApp(
    ProviderScope(
      overrides: [
        apiServiceProvider.overrideWithValue(MockApi()),
      ],
      child: MaterialApp(
        theme: ThemeData(
          colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xFF6366F1)),
          useMaterial3: true,
          fontFamily: 'Inter',
        ),
        home: const DashboardScreen(),
      ),
    ),
  );
}
