import 'dart:convert';
import 'dart:async';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:http/http.dart' as http;
import 'package:ohc_app/models/agent.dart';
import 'package:ohc_app/models/ai_provider.dart';
import 'package:ohc_app/models/channel.dart';
import 'package:ohc_app/models/security_issue.dart';
import 'package:ohc_app/models/skill.dart';
import 'package:ohc_app/models/handoff.dart';
import 'package:ohc_app/models/pipeline.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/models/settings.dart';
import 'package:ohc_app/models/user.dart';
import 'package:ohc_app/services/auth_service.dart';

/// API client for the OHC backend.
class ApiService {
  final String baseUrl;
  final String token;
  final http.Client _client;

  ApiService({required this.baseUrl, required this.token, http.Client? client})
    : _client = client ?? http.Client();

  // Custom timeout to ensure graceful degradation when backend is unreachable or latency spikes
  final Duration _timeout = const Duration(seconds: 15);

  Map<String, String> get _headers => {
    'Content-Type': 'application/json',
    'Authorization': 'Bearer $token',
  };

  // ── Agents ──────────────────────────────────────────────────────────────

  Future<List<Agent>> listAgents() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/agents'),
      headers: _headers,
    ).timeout(_timeout);
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list.map((e) => Agent.fromJson(e as Map<String, dynamic>)).toList();
  }

  Future<Agent> hireAgent(
    String name,
    String role, {
    String providerType = 'openclaw',
  }) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/agents/hire'),
      headers: _headers,
      body: jsonEncode({
        'name': name,
        'role': role,
        'providerType': providerType,
      }),
    );
    _checkStatus(res);
    return Agent.fromJson(jsonDecode(res.body) as Map<String, dynamic>);
  }

  Future<List<AgentProvider>> listAgentProviders() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/agents/providers'),
      headers: _headers,
    );
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list
        .map((e) => AgentProvider.fromJson(e as Map<String, dynamic>))
        .toList();
  }

  Future<void> fireAgent(String agentId) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/agents/fire'),
      headers: _headers,
      body: jsonEncode({'agent_id': agentId}),
    );
    _checkStatus(res);
  }

  Future<void> scaleAgents(String role, int count) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/v1/scale'),
      headers: _headers,
      body: jsonEncode({'role': role, 'count': count}),
    );
    _checkStatus(res);
  }

  // ── Dashboard & Analytics ────────────────────────────────────────────────

  Future<DashboardSnapshot> getDashboard() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/dashboard'),
      headers: _headers,
    );
    _checkStatus(res);
    return DashboardSnapshot.fromJson(
      jsonDecode(res.body) as Map<String, dynamic>,
    );
  }

  Future<void> seedScenario(String scenario) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/seed'),
      headers: _headers,
      body: jsonEncode({'scenario': scenario}),
    );
    _checkStatus(res);
  }

  // ── Handoffs & Approvals ─────────────────────────────────────────────────

  Future<List<HandoffPackage>> listHandoffs() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/handoffs'),
      headers: _headers,
    );
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list
        .map((e) => HandoffPackage.fromJson(e as Map<String, dynamic>))
        .toList();
  }

  Future<void> resolveHandoff(String handoffId, String resolution) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/handoffs/$handoffId/resolve'),
      headers: _headers,
      body: jsonEncode({'resolution': resolution}),
    );
    _checkStatus(res);
  }

  Future<void> decideApproval(String approvalId, String decision) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/approvals/$approvalId/decide'),
      headers: _headers,
      body: jsonEncode({'decision': decision}),
    );
    _checkStatus(res);
  }

  // ── Pipelines ─────────────────────────────────────────────────────────────

  Future<List<Pipeline>> listPipelines() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/pipelines'),
      headers: _headers,
    );
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list
        .map((e) => Pipeline.fromJson(e as Map<String, dynamic>))
        .toList();
  }

  Future<Pipeline> createPipeline(String name, String branch) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/pipelines'),
      headers: _headers,
      body: jsonEncode({'name': name, 'branch': branch}),
    );
    _checkStatus(res);
    return Pipeline.fromJson(jsonDecode(res.body) as Map<String, dynamic>);
  }

  Future<void> promotePipeline(String pipelineId) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/pipelines/$pipelineId/promote'),
      headers: _headers,
    );
    _checkStatus(res);
  }

  // ── Users & RBAC ─────────────────────────────────────────────────────────

  Future<List<UserPublic>> listUsers() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/users'),
      headers: _headers,
    );
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list
        .map((e) => UserPublic.fromJson(e as Map<String, dynamic>))
        .toList();
  }

  Future<UserPublic> createUser(Map<String, dynamic> data) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/users'),
      headers: _headers,
      body: jsonEncode(data),
    ).timeout(_timeout);
    _checkStatus(res);
    return UserPublic.fromJson(jsonDecode(res.body) as Map<String, dynamic>);
  }

  Future<void> deleteUser(String userId) async {
    final res = await _client.delete(
      Uri.parse('$baseUrl/api/users/$userId'),
      headers: _headers,
    ).timeout(_timeout);
    _checkStatus(res);
  }

  // ── MCP Tools ────────────────────────────────────────────────────────────

  Future<List<Map<String, dynamic>>> listMCPTools() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/mcp/tools'),
      headers: _headers,
    );
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list.cast<Map<String, dynamic>>();
  }

  Future<dynamic> invokeMCPTool(
    String toolId,
    String action,
    Map<String, dynamic> params,
  ) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/mcp/tools/$toolId/invoke'),
      headers: _headers,
      body: jsonEncode({'action': action, 'params': params}),
    );
    _checkStatus(res);
    return jsonDecode(res.body);
  }

  // ── Meetings ─────────────────────────────────────────────────────────────

  Future<List<Map<String, dynamic>>> listMeetings() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/meetings'),
      headers: _headers,
    );
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list.cast<Map<String, dynamic>>();
  }

  Future<Map<String, dynamic>> createMeeting(String name) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/meetings'),
      headers: _headers,
      body: jsonEncode({'name': name}),
    );
    _checkStatus(res);
    return jsonDecode(res.body) as Map<String, dynamic>;
  }

  Future<Map<String, dynamic>> joinMeeting(String meetingId) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/meetings/$meetingId/join'),
      headers: _headers,
    );
    _checkStatus(res);
    return jsonDecode(res.body) as Map<String, dynamic>;
  }

  // ── Channels ─────────────────────────────────────────────────────────────

  Future<List<ChatChannel>> listChannels() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/channels'),
      headers: _headers,
    ).timeout(_timeout);
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list
        .map((e) => ChatChannel.fromJson(e as Map<String, dynamic>))
        .toList();
  }

  Future<ChatChannel> addChannel({
    required String name,
    required String backend,
    required Map<String, String> config,
  }) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/channels'),
      headers: _headers,
      body: jsonEncode({'name': name, 'backend': backend, 'config': config}),
    );
    _checkStatus(res);
    return ChatChannel.fromJson(jsonDecode(res.body) as Map<String, dynamic>);
  }

  Future<void> deleteChannel(String channelId) async {
    final res = await _client.delete(
      Uri.parse('$baseUrl/api/channels/$channelId'),
      headers: _headers,
    );
    _checkStatus(res);
  }

  // ── Settings ─────────────────────────────────────────────────────────────

  Future<Settings> getSettings() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/settings'),
      headers: _headers,
    );
    _checkStatus(res);
    return Settings.fromJson(jsonDecode(res.body) as Map<String, dynamic>);
  }

  Future<void> saveSettings(Settings settings) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/settings'),
      headers: _headers,
      body: jsonEncode(settings.toJson()),
    );
    _checkStatus(res);
  }

  Future<void> saveAiProviderKey(String providerId, String apiKey) async {
    final res = await _client.patch(
      Uri.parse('$baseUrl/api/ai/providers/$providerId'),
      headers: _headers,
      body: jsonEncode({'api_key': apiKey}),
    );
    _checkStatus(res);
  }

  // ── AI Providers ──────────────────────────────────────────────────────────

  Future<List<AiProvider>> listAiProviders() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/ai/providers'),
      headers: _headers,
    );
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list
        .map((e) => AiProvider.fromJson(e as Map<String, dynamic>))
        .toList();
  }

  Future<AiProvider> addAiProvider({
    required String name,
    required String baseUrl,
    required String apiKey,
    required List<String> models,
  }) async {
    final res = await _client.post(
      Uri.parse('${this.baseUrl}/api/ai/providers'),
      headers: _headers,
      body: jsonEncode({
        'name': name,
        'base_url': baseUrl,
        'api_key': apiKey,
        'models': models,
      }),
    );
    _checkStatus(res);
    return AiProvider.fromJson(jsonDecode(res.body) as Map<String, dynamic>);
  }

  // ── Skills ────────────────────────────────────────────────────────────────

  Future<List<Skill>> listSkills() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/skills'),
      headers: _headers,
    );
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list.map((e) => Skill.fromJson(e as Map<String, dynamic>)).toList();
  }

  Future<void> installSkill(String name) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/skills/$name/install'),
      headers: _headers,
    );
    _checkStatus(res);
  }

  Future<void> uninstallSkill(String name) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/skills/$name/uninstall'),
      headers: _headers,
    );
    _checkStatus(res);
  }

  Future<void> setSkillEnabled(String name, bool enabled) async {
    final res = await _client.patch(
      Uri.parse('$baseUrl/api/skills/$name'),
      headers: _headers,
      body: jsonEncode({'enabled': enabled}),
    );
    _checkStatus(res);
  }

  // ── Security ──────────────────────────────────────────────────────────────

  Future<List<SecurityIssue>> listSecurityIssues() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/security/issues'),
      headers: _headers,
    );
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list
        .map((e) => SecurityIssue.fromJson(e as Map<String, dynamic>))
        .toList();
  }

  Future<void> fixSecurityIssue(String issueId) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/security/issues/$issueId/fix'),
      headers: _headers,
    );
    _checkStatus(res);
  }

  // ── Service logs ──────────────────────────────────────────────────────────

  Future<List<String>> getLogs({int lines = 100}) async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/service/logs?lines=$lines'),
      headers: _headers,
    );
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list.cast<String>();
  }

  Stream<String> streamScaleEvents() async* {
    final req = http.Request('GET', Uri.parse('$baseUrl/api/v1/scale/stream'));
    req.headers.addAll(_headers);
    final res = await _client.send(req);
    if (res.statusCode < 200 || res.statusCode >= 300) {
      throw Exception('API error ${res.statusCode}');
    }

    await for (final chunk in res.stream.transform(utf8.decoder)) {
      final lines = chunk.split('\n');
      for (final line in lines) {
        if (line.startsWith('data: ')) {
          final data = line.substring(6).trim();
          if (data.isNotEmpty) {
            yield data;
          }
        }
      }
    }
  }

  // ── Growth ───────────────────────────────────────────────────────────────

  Future<List<Map<String, dynamic>>> listLandingPageExperiments() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/growth/experiments'),
      headers: _headers,
    );
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list.cast<Map<String, dynamic>>();
  }

  Future<Map<String, dynamic>> createLandingPageExperiment(String title, double trafficSplit) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/growth/experiments'),
      headers: _headers,
      body: jsonEncode({
        'title': title,
        'trafficSplit': trafficSplit,
      }),
    );
    _checkStatus(res);
    return jsonDecode(res.body) as Map<String, dynamic>;
  }

  Future<List<Map<String, dynamic>>> listReferrals() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/growth/referrals'),
      headers: _headers,
    );
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list.cast<Map<String, dynamic>>();
  }

    // ── Shared Tasks ─────────────────────────────────────────────────────────

  Future<List<Map<String, dynamic>>> listSharedTasks() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/shared-tasks'),
      headers: _headers,
    );
    _checkStatus(res);
    final list = jsonDecode(res.body) as List<dynamic>;
    return list.cast<Map<String, dynamic>>();
  }

  // ── Helpers ──────────────────────────────────────────────────────────────

  Future<Map<String, dynamic>> getViralCoefficient() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/growth/viral-coefficient'),
      headers: _headers,
    );
    _checkStatus(res);
    return jsonDecode(res.body) as Map<String, dynamic>;
  }




  Future<void> trackDownload(String os, String version) async {
    final response = await http.post(
      Uri.parse('$baseUrl/api/growth/downloads'),
      headers: {
        'Content-Type': 'application/json',
      },
      body: jsonEncode({
        'os': os,
        'version': version,
      }),
    );

    if (response.statusCode != 200) {
      throw Exception('Failed to track download');
    }
  }

  Future<Map<String, dynamic>> getQuota() async {
    final res = await _client.get(
      Uri.parse('$baseUrl/api/growth/quota'),
      headers: _headers,
    );
    _checkStatus(res);
    return jsonDecode(res.body) as Map<String, dynamic>;
  }

  Future<void> createReferral(String userId, String referralCode) async {
    final response = await http.post(
      Uri.parse('$baseUrl/api/growth/referrals'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer $token',
      },
      body: jsonEncode({
        'userId': userId,
        'referralCode': referralCode,
      }),
    );
    _checkStatus(response);
  }

  void _checkStatus(http.Response res) {
    if (res.statusCode < 200 || res.statusCode >= 300) {
      throw Exception('API error ${res.statusCode}: ${res.body}');
    }
  }
}

// ── Providers ──────────────────────────────────────────────────────────────

final apiServiceProvider = Provider<ApiService?>((ref) {
  final user = ref.watch(authStateProvider).valueOrNull;
  if (user == null) return null;
  final url = ref.watch(backendUrlProvider);
  return ApiService(baseUrl: url, token: user.token);
});
