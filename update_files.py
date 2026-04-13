import sys

# 1. Router
with open('srcs/app/lib/router.dart', 'r') as f:
    router_content = f.read()

if "prompt_tuning_wizard_screen.dart" not in router_content:
    import_stmt = "import 'package:ohc_app/screens/prompt_tuning_wizard_screen.dart';\n"
    router_content = router_content.replace("import 'package:ohc_app/screens/agents_screen.dart';", f"import 'package:ohc_app/screens/agents_screen.dart';\n{import_stmt}")

if "/agents/:id/tune" not in router_content:
    route_stmt = """          GoRoute(
            path: '/agents/:id/tune',
            builder: (context, state) => PromptTuningWizardScreen(agentId: state.pathParameters['id']!),
          ),\n"""
    router_content = router_content.replace("          GoRoute(\n            path: '/agents/hire',", f"{route_stmt}          GoRoute(\n            path: '/agents/hire',")

with open('srcs/app/lib/router.dart', 'w') as f:
    f.write(router_content)

# 2. Agents screen
with open('srcs/app/lib/screens/agents_screen.dart', 'r') as f:
    agents_content = f.read()

target = "                                const SizedBox(width: 16),\n                                AnimatedContainer("
menu_code = """                                const SizedBox(width: 8),
                                IconButton(
                                  icon: const Icon(Icons.more_vert),
                                  onPressed: () {
                                    showModalBottomSheet(
                                      context: context,
                                      builder: (ctx) => SafeArea(
                                        child: Column(
                                          mainAxisSize: MainAxisSize.min,
                                          children: [
                                            ListTile(
                                              leading: const Icon(Icons.tune),
                                              title: const Text('Tune this agent'),
                                              onTap: () {
                                                Navigator.pop(ctx);
                                                context.go('/agents/${widget.agent.name}/tune');
                                              },
                                            ),
                                          ],
                                        ),
                                      ),
                                    );
                                  },
                                ),
                                const SizedBox(width: 8),
                                AnimatedContainer("""

if "Icons.more_vert" not in agents_content:
    agents_content = agents_content.replace(target, menu_code)
    if "import 'package:go_router/go_router.dart';" not in agents_content:
        agents_content = "import 'package:go_router/go_router.dart';\n" + agents_content
    with open('srcs/app/lib/screens/agents_screen.dart', 'w') as f:
        f.write(agents_content)

# 3. API service
with open('srcs/app/lib/services/api_service.dart', 'r') as f:
    api_content = f.read()

api_code = """  Future<void> updateAgentPrompt(String agentId, String prompt) async {
    final res = await _client.post(
      Uri.parse('$baseUrl/api/agents/tune'),
      headers: _headers,
      body: jsonEncode({'agentId': agentId, 'prompt': prompt}),
    );
    _checkStatus(res);
  }
"""

if "updateAgentPrompt" not in api_content:
    api_content = api_content.replace("  // ── Meetings", f"{api_code}\n  // ── Meetings")
    with open('srcs/app/lib/services/api_service.dart', 'w') as f:
        f.write(api_content)
