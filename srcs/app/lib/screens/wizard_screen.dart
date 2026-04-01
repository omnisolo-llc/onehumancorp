import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dart:ui';
import 'package:http/http.dart' as http;
import 'package:ohc_app/services/auth_service.dart';

// ── Providers ─────────────────────────────────────────────────────────────

/// Provider that fetches the wizard status from the backend.
final wizardStatusProvider = FutureProvider.autoDispose<WizardStatus>((
  ref,
) async {
  final user = ref.watch(authStateProvider).valueOrNull;
  if (user == null) return WizardStatus.empty();
  final baseUrl = const String.fromEnvironment(
    'BACKEND_URL',
    defaultValue: 'http://localhost:18789',
  );
  final resp = await http.get(
    Uri.parse('$baseUrl/api/wizard/status'),
    headers: {'Authorization': 'Bearer ${user.token}'},
  );
  if (resp.statusCode != 200) return WizardStatus.empty();
  final json = jsonDecode(resp.body) as Map<String, dynamic>;
  return WizardStatus.fromJson(json);
});

// ── Model ──────────────────────────────────────────────────────────────────

class WizardStatus {
  final bool configured;
  final bool serverStep;
  final bool aiProviderStep;
  final bool centrifugeStep;

  const WizardStatus({
    required this.configured,
    required this.serverStep,
    required this.aiProviderStep,
    required this.centrifugeStep,
  });

  factory WizardStatus.empty() => const WizardStatus(
    configured: false,
    serverStep: false,
    aiProviderStep: false,
    centrifugeStep: false,
  );

  factory WizardStatus.fromJson(Map<String, dynamic> json) {
    final steps = json['steps'] as Map<String, dynamic>? ?? {};
    return WizardStatus(
      configured: json['configured'] as bool? ?? false,
      serverStep: steps['server'] as bool? ?? false,
      aiProviderStep: steps['ai_provider'] as bool? ?? false,
      centrifugeStep: steps['centrifuge'] as bool? ?? false,
    );
  }
}

// ── Screen ─────────────────────────────────────────────────────────────────

/// A multi-step configuration wizard inspired by the openclaw.ai gateway
/// configuration guide.  Guides the user through:
///   1. Server settings (listen address, database path)
///   2. AI provider configuration (Minimax API key, model selection)
///   3. Real-time messaging setup (Centrifuge WebSocket URL)
class SetupWizardScreen extends ConsumerStatefulWidget {
  const SetupWizardScreen({super.key});

  @override
  ConsumerState<SetupWizardScreen> createState() => _SetupWizardScreenState();
}

class _SetupWizardScreenState extends ConsumerState<SetupWizardScreen> {
  int _step = 0;
  bool _saving = false;
  String? _error;

  // Step 1 – Server
  final _listenAddrCtrl = TextEditingController(text: '0.0.0.0:18789');
  final _dbPathCtrl = TextEditingController(text: 'ohc.db');

  // Step 2 – AI Provider
  final _minimaxKeyCtrl = TextEditingController();
  final _modelCtrl = TextEditingController(text: 'abab6.5s');

  // Step 3 – Centrifuge
  final _centrifugeUrlCtrl = TextEditingController(
    text: 'ws://localhost:8000/connection/websocket',
  );

  @override
  void dispose() {
    _listenAddrCtrl.dispose();
    _dbPathCtrl.dispose();
    _minimaxKeyCtrl.dispose();
    _modelCtrl.dispose();
    _centrifugeUrlCtrl.dispose();
    super.dispose();
  }

  Future<void> _save() async {
    final user = ref.read(authStateProvider).valueOrNull;
    if (user == null) return;
    setState(() {
      _saving = true;
      _error = null;
    });

    final baseUrl = const String.fromEnvironment(
      'BACKEND_URL',
      defaultValue: 'http://localhost:18789',
    );

    final body = <String, dynamic>{
      'listen_addr': _listenAddrCtrl.text.trim(),
      'db_path': _dbPathCtrl.text.trim(),
      'centrifuge_url': _centrifugeUrlCtrl.text.trim(),
    };
    if (_minimaxKeyCtrl.text.trim().isNotEmpty) {
      body['minimax_api_key'] = _minimaxKeyCtrl.text.trim();
      body['ai_providers'] = [
        {
          'name': 'minimax',
          'api_key': _minimaxKeyCtrl.text.trim(),
          'model': _modelCtrl.text.trim(),
          'enabled': true,
        },
      ];
    }

    try {
      final resp = await http.post(
        Uri.parse('$baseUrl/api/wizard/configure'),
        headers: {
          'Authorization': 'Bearer ${user.token}',
          'Content-Type': 'application/json',
        },
        body: jsonEncode(body),
      );
      if (resp.statusCode == 200) {
        ref.invalidate(wizardStatusProvider);
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: const Text('Configuration saved successfully!'),
              backgroundColor: Theme.of(context).colorScheme.primary,
            ),
          );
        }
      } else {
        setState(() => _error = 'Save failed: ${resp.statusCode} ${resp.body}');
      }
    } catch (e) {
      setState(() => _error = 'Error: $e');
    } finally {
      if (mounted) setState(() => _saving = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final statusAsync = ref.watch(wizardStatusProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'Setup Wizard',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
        actions: [
          statusAsync.maybeWhen(
            data:
                (s) =>
                    s.configured
                        ? Padding(
                          padding: const EdgeInsets.only(right: 16),
                          child: Icon(
                            Icons.check_circle,
                            color: Theme.of(context).colorScheme.primary,
                          ),
                        )
                        : const SizedBox.shrink(),
            orElse: () => const SizedBox.shrink(),
          ),
        ],
      ),
      body: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            // Status banner
            statusAsync.when(
              loading: () => const LinearProgressIndicator(),
              error: (e, _) => const SizedBox.shrink(),
              data: (s) => _StatusBanner(status: s),
            ),
            const SizedBox(height: 24),

            // Step indicator
            _StepIndicator(current: _step),
            const SizedBox(height: 24),

            // Step content
            Expanded(
              child: IndexedStack(
                index: _step,
                children: [
                  _ServerStep(
                    listenAddrCtrl: _listenAddrCtrl,
                    dbPathCtrl: _dbPathCtrl,
                  ),
                  _AiProviderStep(
                    keyCtrl: _minimaxKeyCtrl,
                    modelCtrl: _modelCtrl,
                  ),
                  _CentrifugeStep(urlCtrl: _centrifugeUrlCtrl),
                ],
              ),
            ),

            if (_error != null) ...[
              const SizedBox(height: 8),
              Text(
                _error!,
                style: TextStyle(color: Theme.of(context).colorScheme.error),
              ),
            ],
            const SizedBox(height: 16),

            // Navigation buttons
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                if (_step > 0)
                  Semantics(
                    label: 'Go back to previous step',
                    child: Tooltip(
                      message: 'Previous step',
                      child: OutlinedButton(
                        onPressed: () => setState(() => _step--),
                        child: const Text('Back'),
                      ),
                    ),
                  )
                else
                  const SizedBox.shrink(),
                if (_step < 2)
                  Semantics(
                    label: 'Proceed to next step',
                    child: Tooltip(
                      message: 'Next step',
                      child: FilledButton(
                        onPressed: () => setState(() => _step++),
                        child: const Text('Next'),
                      ),
                    ),
                  )
                else
                  Semantics(
                    label: 'Save and finish wizard',
                    child: Tooltip(
                      message: 'Save all configuration settings',
                      child: FilledButton(
                        onPressed: _saving ? null : _save,
                        child:
                            _saving
                                ? const SizedBox(
                                  width: 20,
                                  height: 20,
                                  child: CircularProgressIndicator(
                                    strokeWidth: 2,
                                  ),
                                )
                                : const Text('Save Configuration'),
                      ),
                    ),
                  ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

// ── Step Indicator ─────────────────────────────────────────────────────────

class _StepIndicator extends StatelessWidget {
  final int current;
  const _StepIndicator({required this.current});

  static const _labels = ['Server', 'AI Provider', 'Real-time'];

  @override
  Widget build(BuildContext context) {
    return Row(
      children: List.generate(_labels.length, (i) {
        final done = i < current;
        final active = i == current;
        return Expanded(
          child: Semantics(
            label:
                'Step ${i + 1}: ${_labels[i]}, Status: ${done
                    ? 'Completed'
                    : active
                    ? 'Active'
                    : 'Pending'}',
            child: Row(
              children: [
                CircleAvatar(
                  radius: 16,
                  backgroundColor:
                      done
                          ? Theme.of(context).colorScheme.primary
                          : active
                          ? Theme.of(context).colorScheme.secondary
                          : Theme.of(
                            context,
                          ).colorScheme.surfaceContainerHighest,
                  child:
                      done
                          ? Icon(
                            Icons.check,
                            size: 16,
                            color: Theme.of(context).colorScheme.onPrimary,
                          )
                          : Text(
                            '${i + 1}',
                            style: TextStyle(
                              color:
                                  active
                                      ? Theme.of(
                                        context,
                                      ).colorScheme.onSecondary
                                      : null,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                ),
                const SizedBox(width: 4),
                Text(
                  _labels[i],
                  style: TextStyle(
                    fontWeight: active ? FontWeight.bold : FontWeight.normal,
                  ),
                ),
                if (i < _labels.length - 1) const Expanded(child: Divider()),
              ],
            ),
          ),
        );
      }),
    );
  }
}

// ── Status Banner ──────────────────────────────────────────────────────────

class _StatusBanner extends StatelessWidget {
  final WizardStatus status;
  const _StatusBanner({required this.status});

  @override
  Widget build(BuildContext context) {
    Widget buildGlassCard({required Color color, required Widget child}) {
      return ClipRRect(
        borderRadius: BorderRadius.circular(12),
        child: BackdropFilter(
          filter: ImageFilter.compose(
            outer: ColorFilter.matrix(<double>[
              1.787,
              -0.715,
              -0.072,
              0,
              0,
              -0.213,
              1.285,
              -0.072,
              0,
              0,
              -0.213,
              -0.715,
              1.928,
              0,
              0,
              0,
              0,
              0,
              1,
              0,
            ]),
            inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
          ),
          child: Container(
            decoration: BoxDecoration(
              color: color.withOpacity(0.03),
              border: Border.all(color: color.withOpacity(0.08)),
              borderRadius: BorderRadius.circular(12),
            ),
            child: child,
          ),
        ),
      );
    }

    if (status.configured) {
      return Semantics(
        label: 'Platform is fully configured',
        child: buildGlassCard(
          color: Theme.of(context).colorScheme.primary,
          child: ListTile(
            leading: Icon(
              Icons.check_circle,
              color: Theme.of(context).colorScheme.primary,
            ),
            title: const Text(
              'Platform is fully configured',
              style: TextStyle(fontWeight: FontWeight.bold),
            ),
            subtitle: const Text('All wizard steps have been completed.'),
          ),
        ),
      );
    }
    final missing = <String>[];
    if (!status.serverStep) missing.add('Server');
    if (!status.aiProviderStep) missing.add('AI Provider');
    if (!status.centrifugeStep) missing.add('Centrifuge');
    return Semantics(
      label: 'Configuration incomplete. Remaining: ${missing.join(', ')}',
      child: buildGlassCard(
        color: Theme.of(context).colorScheme.error,
        child: ListTile(
          leading: Icon(
            Icons.warning_amber,
            color: Theme.of(context).colorScheme.error,
          ),
          title: const Text(
            'Configuration incomplete',
            style: TextStyle(fontWeight: FontWeight.bold),
          ),
          subtitle: Text('Remaining: ${missing.join(', ')}'),
        ),
      ),
    );
  }
}

// ── Step 1 – Server ────────────────────────────────────────────────────────

class _ServerStep extends StatelessWidget {
  final TextEditingController listenAddrCtrl;
  final TextEditingController dbPathCtrl;
  const _ServerStep({required this.listenAddrCtrl, required this.dbPathCtrl});

  @override
  Widget build(BuildContext context) {
    return ListView(
      children: [
        Text('Server Settings', style: Theme.of(context).textTheme.titleLarge),
        const SizedBox(height: 8),
        const Text(
          'Configure the OHC backend server address and database path. '
          'These settings control where the backend listens for connections '
          'and where it stores persistent data.',
        ),
        const SizedBox(height: 24),
        TextField(
          controller: listenAddrCtrl,
          decoration: const InputDecoration(
            labelText: 'Listen Address',
            hintText: 'host:port',
            helperText: 'Host and port the server will bind to',
            border: OutlineInputBorder(),
          ),
        ),
        const SizedBox(height: 16),
        TextField(
          controller: dbPathCtrl,
          decoration: const InputDecoration(
            labelText: 'Database Path',
            hintText: 'ohc.db',
            helperText: 'Path to the SQLite database file',
            border: OutlineInputBorder(),
          ),
        ),
      ],
    );
  }
}

// ── Step 2 – AI Provider ───────────────────────────────────────────────────

class _AiProviderStep extends StatefulWidget {
  final TextEditingController keyCtrl;
  final TextEditingController modelCtrl;
  const _AiProviderStep({required this.keyCtrl, required this.modelCtrl});

  @override
  State<_AiProviderStep> createState() => _AiProviderStepState();
}

class _AiProviderStepState extends State<_AiProviderStep> {
  bool _obscureKey = true;

  @override
  Widget build(BuildContext context) {
    return ListView(
      children: [
        Text(
          'AI Provider — Minimax',
          style: Theme.of(context).textTheme.titleLarge,
        ),
        const SizedBox(height: 8),
        const Text(
          'Connect the OHC platform to the Minimax AI service. '
          'Your API key is stored locally and sent securely to the backend. '
          'You can skip this step and configure providers later in '
          'Settings → AI Providers.',
        ),
        const SizedBox(height: 24),
        Semantics(
          label: 'Minimax API Key Input',
          textField: true,
          child: TextField(
            controller: widget.keyCtrl,
            obscureText: _obscureKey,
            decoration: InputDecoration(
              labelText: 'Minimax API Key',
              hintText: 'sk-…',
              helperText: 'Obtain from platform.minimaxi.com',
              border: const OutlineInputBorder(),
              suffixIcon: IconButton(
                icon: Icon(
                  _obscureKey ? Icons.visibility : Icons.visibility_off,
                ),
                tooltip: _obscureKey ? 'Show API Key' : 'Hide API Key',
                onPressed: () {
                  setState(() {
                    _obscureKey = !_obscureKey;
                  });
                },
              ),
            ),
          ),
        ),
        const SizedBox(height: 16),
        Semantics(
          label: 'Default Model Input',
          textField: true,
          child: TextField(
            controller: widget.modelCtrl,
            decoration: const InputDecoration(
              labelText: 'Default Model',
              hintText: 'abab6.5s',
              helperText: 'The Minimax model to use for agent reasoning',
              border: OutlineInputBorder(),
            ),
          ),
        ),
      ],
    );
  }
}

// ── Step 3 – Centrifuge ────────────────────────────────────────────────────

class _CentrifugeStep extends StatelessWidget {
  final TextEditingController urlCtrl;
  const _CentrifugeStep({required this.urlCtrl});

  @override
  Widget build(BuildContext context) {
    return ListView(
      children: [
        Text(
          'Real-time Messaging (Centrifuge)',
          style: Theme.of(context).textTheme.titleLarge,
        ),
        const SizedBox(height: 8),
        const Text(
          'OHC uses the Centrifuge real-time messaging protocol to deliver '
          'live meeting room updates and native chat notifications to mobile '
          'and web clients without polling. '
          'The default URL points to the built-in Centrifuge endpoint served '
          'by the OHC backend.',
        ),
        const SizedBox(height: 24),
        TextField(
          controller: urlCtrl,
          decoration: const InputDecoration(
            labelText: 'Centrifuge WebSocket URL',
            hintText: 'ws://localhost:8000/connection/websocket',
            helperText: 'WebSocket endpoint of the Centrifuge server',
            border: OutlineInputBorder(),
          ),
        ),
        const SizedBox(height: 16),
        Semantics(
          label: 'Channel Convention Info Card',
          child: const Card(
            child: Padding(
              padding: EdgeInsets.all(12),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'Channel convention',
                    style: TextStyle(fontWeight: FontWeight.bold),
                  ),
                  SizedBox(height: 4),
                  Text('• meeting:<id>  — live transcript updates'),
                  Text('• chat:<room>   — real-time chat messages'),
                  Text('• agent:<id>    — per-agent inbox notifications'),
                ],
              ),
            ),
          ),
        ),
      ],
    );
  }
}
