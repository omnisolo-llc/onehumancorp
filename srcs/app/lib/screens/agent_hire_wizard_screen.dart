import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/models/agent.dart';
import 'package:ohc_app/services/api_service.dart';

class AgentHireWizardScreen extends ConsumerStatefulWidget {
  const AgentHireWizardScreen({super.key});

  @override
  ConsumerState<AgentHireWizardScreen> createState() =>
      _AgentHireWizardScreenState();
}

class _AgentHireWizardScreenState extends ConsumerState<AgentHireWizardScreen> {
  int _step = 0;
  String _selectedRole = '';
  String _selectedProvider = '';
  final _nameController = TextEditingController();

  // Topology state
  String _topologyPreset = 'Independent';

  // Capabilities selection state
  bool _capEmailRead = false;
  bool _capMessagingSend = false;
  bool _capDataAccess = false;

  // Resource limits state
  double _maxSessionsPerDay = 50.0;
  bool _showAdvanced = false;
  final _endpointUrlController = TextEditingController();
  final _tokenLimitController = TextEditingController();

  bool _isDeploying = false;
  bool _isLoading = true;
  List<String> _roles = [];
  List<AgentProvider> _providers = [];

  @override
  void initState() {
    super.initState();
    _fetchData();
  }

  Future<void> _fetchData() async {
    try {
      final api = ref.read(apiServiceProvider);
      if (api == null) return;

      final providers = await api.listAgentProviders();
      final rolesSet = <String>{};
      for (final p in providers) {
        rolesSet.addAll(p.supportedRoles);
      }

      if (mounted) {
        setState(() {
          _providers = providers;
          _roles = rolesSet.toList()..sort();
          if (_providers.isNotEmpty) {
            _selectedProvider = _providers.first.type;
          }
          _isLoading = false;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() => _isLoading = false);
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Failed to load providers: $e'),
            backgroundColor: Theme.of(context).colorScheme.error,
          ),
        );
      }
    }
  }

  @override
  void dispose() {
    _nameController.dispose();
    super.dispose();
  }

  String _formatRole(String role) {
    return role
        .replaceAll('_', ' ')
        .toLowerCase()
        .split(' ')
        .map((word) {
          if (word == 'ai') return 'AI';
          if (word == 'ceo') return 'CEO';
          if (word == 'qa') return 'QA';
          if (word == 'cfo') return 'CFO';
          if (word == 'seo') return 'SEO';
          if (word == 'llm') return 'LLM';
          if (word.isEmpty) return word;
          return word[0].toUpperCase() + word.substring(1);
        })
        .join(' ');
  }

  Future<void> _handleDeploy() async {
    setState(() => _isDeploying = true);
    try {
      final api = ref.read(apiServiceProvider);
      if (api != null) {
        // Note: Additional state collected (capabilities, limits, topology)
        // should be passed here when the API supports them. For now we use the required fields.
        await api.hireAgent(
          _nameController.text.trim(),
          _selectedRole,
          providerType: _selectedProvider,
        );
        if (mounted) {
          context.go('/agents');
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text(
                'Agent ${_nameController.text} hired successfully!',
              ),
            ),
          );
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Failed to hire agent: $e'),
            backgroundColor: Theme.of(context).colorScheme.error,
          ),
        );
      }
    } finally {
      if (mounted) setState(() => _isDeploying = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'Hire New Agent',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
        leading: IconButton(
          icon: const Icon(Icons.close),
          tooltip: 'Close wizard',
          onPressed: () => context.go('/agents'),
        ),
      ),
      body: Stepper(
        type: StepperType.horizontal,
        currentStep: _step,
        onStepContinue: () {
          if (_step < 6) {
            setState(() => _step++);
          } else {
            _handleDeploy();
          }
        },
        onStepCancel: () {
          if (_step > 0) {
            setState(() => _step--);
          }
        },
        controlsBuilder: (context, details) {
          return Padding(
            padding: const EdgeInsets.only(top: 24),
            child: Row(
              children: [
                if (_step < 6)
                  Semantics(
                    label: 'Proceed to next step',
                    child: Tooltip(
                      message: 'Next step',
                      child: ElevatedButton(
                        onPressed:
                            (_step == 0 && _selectedRole.isEmpty)
                                ? null
                                : details.onStepContinue,
                        child: const Text('Next'),
                      ),
                    ),
                  )
                else
                  Semantics(
                    label: 'Deploy the configured agent',
                    child: Tooltip(
                      message: 'Deploy agent to orchestration hub',
                      child: ElevatedButton(
                        onPressed: _isDeploying ? null : _handleDeploy,
                        child:
                            _isDeploying
                                ? const SizedBox(
                                  width: 20,
                                  height: 20,
                                  child: CircularProgressIndicator(
                                    strokeWidth: 2,
                                  ),
                                )
                                : const Text('Deploy Agent'),
                      ),
                    ),
                  ),
                const SizedBox(width: 12),
                if (_step > 0)
                  Semantics(
                    label: 'Go back to previous step',
                    child: Tooltip(
                      message: 'Previous step',
                      child: TextButton(
                        onPressed: details.onStepCancel,
                        child: const Text('Back'),
                      ),
                    ),
                  ),
              ],
            ),
          );
        },
        steps: [
          Step(
            title: const Text('Role'),
            isActive: _step >= 0,
            content: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  'Step 1 — Select Agent Role',
                  style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                ),
                const SizedBox(height: 8),
                const Text(
                  'Choose the primary capability profile for this new agent.',
                ),
                const SizedBox(height: 24),
                Wrap(
                  spacing: 12,
                  runSpacing: 12,
                  children:
                      _roles.map((role) {
                        final isSelected = _selectedRole == role;
                        return ChoiceChip(
                          label: Text(_formatRole(role)),
                          selected: isSelected,
                          onSelected: (selected) {
                            setState(
                              () => _selectedRole = selected ? role : '',
                            );
                            if (selected && _nameController.text.isEmpty) {
                              _nameController.text =
                                  'Senior ${_formatRole(role)}';
                            }
                          },
                        );
                      }).toList(),
                ),
              ],
            ),
          ),
          Step(
            title: const Text('Provider'),
            isActive: _step >= 1,
            content: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  'Step 2 — Choose AI Provider',
                  style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                ),
                const SizedBox(height: 8),
                const Text('Select the AI backend that will power this agent.'),
                const SizedBox(height: 16),
                if (_isLoading)
                  Center(
                    child: Padding(
                      padding: EdgeInsets.all(32.0),
                      child: CircularProgressIndicator(
                        color: Theme.of(context).colorScheme.primary,
                      ),
                    ),
                  )
                else if (_providers.isEmpty)
                  const Center(
                    child: Text(
                      'No AI providers available. Please configure one in Integrations.',
                    ),
                  )
                else
                  ..._providers.map(
                    (p) => RadioListTile<String>(
                      title: Text(p.label),
                      subtitle: Text(p.description),
                      value: p.type,
                      groupValue: _selectedProvider,
                      onChanged:
                          (val) => setState(() => _selectedProvider = val!),
                    ),
                  ),
              ],
            ),
          ),
          Step(
            title: const Text('Details'),
            isActive: _step >= 2,
            content: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  'Step 3 — Agent Details',
                  style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                ),
                const SizedBox(height: 16),
                TextField(
                  controller: _nameController,
                  decoration: const InputDecoration(
                    labelText: 'Agent Name',
                    border: OutlineInputBorder(),
                    hintText: 'e.g. Senior Software Engineer',
                  ),
                ),
                const SizedBox(height: 16),
                ListTile(
                  leading: const Icon(Icons.info_outline),
                  title: const Text(
                    'This name will appear in transcripts and the org chart.',
                  ),
                ),
              ],
            ),
          ),
          Step(
            title: const Text('Topology'),
            isActive: _step >= 3,
            content: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  'Step 4 — Sub-agent Topology',
                  style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                ),
                const SizedBox(height: 8),
                const Text('How does this agent interact with others?'),
                const SizedBox(height: 16),
                RadioListTile<String>(
                  title: const Text('Independent Worker'),
                  subtitle: const Text('Works alone without sub-agents.'),
                  value: 'Independent',
                  groupValue: _topologyPreset,
                  onChanged: (val) => setState(() => _topologyPreset = val!),
                ),
                RadioListTile<String>(
                  title: const Text('Delegator / Supervisor'),
                  subtitle: const Text(
                    'Can ask other agents for help (e.g. asking the Code Builder).',
                  ),
                  value: 'Delegator',
                  groupValue: _topologyPreset,
                  onChanged: (val) => setState(() => _topologyPreset = val!),
                ),
                if (_showAdvanced) ...[
                  const SizedBox(height: 16),
                  Container(
                    height: 150,
                    width: double.infinity,
                    decoration: BoxDecoration(
                      border: Border.all(color: Colors.white24),
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: const Center(
                      child: Text('Drag-and-drop Node Graph Placeholder'),
                    ),
                  ),
                ],
              ],
            ),
          ),
          Step(
            title: const Text('Capabilities'),
            isActive: _step >= 4,
            content: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  'Step 5 — Select Capabilities',
                  style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                ),
                const SizedBox(height: 8),
                const Text('What permissions should this agent have?'),
                const SizedBox(height: 16),
                CheckboxListTile(
                  title: const Text('Read my emails'),
                  subtitle: const Text('Grants EMAIL_READ permission'),
                  value: _capEmailRead,
                  onChanged: (val) => setState(() => _capEmailRead = val!),
                ),
                CheckboxListTile(
                  title: const Text('Send messages on my behalf'),
                  subtitle: const Text('Grants MESSAGING_SEND permission'),
                  value: _capMessagingSend,
                  onChanged: (val) => setState(() => _capMessagingSend = val!),
                ),
                CheckboxListTile(
                  title: const Text('Access business data'),
                  subtitle: const Text('Grants DATA_ACCESS permission'),
                  value: _capDataAccess,
                  onChanged: (val) => setState(() => _capDataAccess = val!),
                ),
              ],
            ),
          ),
          Step(
            title: const Text('Limits'),
            isActive: _step >= 5,
            content: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const Text(
                      'Step 6 — Resource Limits',
                      style: TextStyle(
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    Row(
                      children: [
                        const Text('Advanced Settings'),
                        Switch(
                          value: _showAdvanced,
                          onChanged:
                              (val) => setState(() => _showAdvanced = val),
                        ),
                      ],
                    ),
                  ],
                ),
                const SizedBox(height: 8),
                const Text('How much should this agent work per day?'),
                const SizedBox(height: 16),
                Row(
                  children: [
                    Expanded(
                      child: Slider(
                        value: _maxSessionsPerDay,
                        min: 10,
                        max: 200,
                        divisions: 19,
                        label: _maxSessionsPerDay.round().toString(),
                        onChanged:
                            (val) => setState(() => _maxSessionsPerDay = val),
                      ),
                    ),
                    Text('${_maxSessionsPerDay.round()} sessions'),
                  ],
                ),
                Text(
                  'Estimated cost: \$${(_maxSessionsPerDay * 0.05).toStringAsFixed(2)} / day',
                  style: TextStyle(
                    color: Theme.of(context).colorScheme.primary,
                  ),
                ),
                if (_showAdvanced) ...[
                  const SizedBox(height: 24),
                  const Text(
                    'Advanced Configuration',
                    style: TextStyle(fontWeight: FontWeight.bold),
                  ),
                  const SizedBox(height: 8),
                  const TextField(
                    decoration: InputDecoration(
                      labelText: 'Custom Endpoint URL (Optional)',
                      border: OutlineInputBorder(),
                    ),
                  ),
                  const SizedBox(height: 16),
                  const TextField(
                    decoration: InputDecoration(
                      labelText: 'Token Limit (Optional)',
                      border: OutlineInputBorder(),
                    ),
                    keyboardType: TextInputType.number,
                  ),
                ],
              ],
            ),
          ),
          Step(
            title: const Text('Confirm'),
            isActive: _step >= 6,
            content: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  'Step 7 — Confirm Deployment',
                  style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                ),
                const SizedBox(height: 16),
                ClipRRect(
                  borderRadius: BorderRadius.circular(16),
                  child: BackdropFilter(
                    filter: ImageFilter.compose(
                      outer: ColorFilter.matrix(const <double>[
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
                        color: Theme.of(
                          context,
                        ).colorScheme.surface.withValues(alpha: 0.1),
                        borderRadius: BorderRadius.circular(16),
                        border: Border.all(
                          color: Theme.of(
                            context,
                          ).colorScheme.onSurface.withValues(alpha: 0.2),
                          width: 1,
                        ),
                      ),
                      child: ListTile(
                        leading: CircleAvatar(
                          backgroundColor: Theme.of(
                            context,
                          ).colorScheme.primary.withValues(alpha: 0.2),
                          child: Text(
                            _selectedRole.isNotEmpty ? _selectedRole[0] : '?',
                            style: TextStyle(
                              color: Theme.of(context).colorScheme.primary,
                              fontFamily: 'Outfit',
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                        ),
                        title: Text(
                          _nameController.text,
                          style: const TextStyle(
                            fontFamily: 'Outfit',
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                        subtitle: Text(
                          _formatRole(_selectedRole),
                          style: const TextStyle(fontFamily: 'Inter'),
                        ),
                        trailing: Container(
                          padding: const EdgeInsets.symmetric(
                            horizontal: 12,
                            vertical: 6,
                          ),
                          decoration: BoxDecoration(
                            color: Theme.of(
                              context,
                            ).colorScheme.secondary.withValues(alpha: 0.15),
                            borderRadius: BorderRadius.circular(20),
                          ),
                          child: Text(
                            _selectedProvider.toUpperCase(),
                            style: TextStyle(
                              color: Theme.of(context).colorScheme.secondary,
                              fontFamily: 'Outfit',
                              fontWeight: FontWeight.bold,
                              fontSize: 12,
                            ),
                          ),
                        ),
                      ),
                    ),
                  ),
                ),
                const SizedBox(height: 16),
                Text(
                  'This agent will be immediately provisioned with a SPIFFE identity, connected to the orchestration hub, and assigned to the default org chart branch.',
                  style: TextStyle(
                    color: Theme.of(
                      context,
                    ).colorScheme.onSurfaceVariant.withValues(alpha: 0.7),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
