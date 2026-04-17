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
  Offset _mainNodePos = const Offset(50, 50);
  Offset _subNodePos = const Offset(200, 50);
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
                        final dummyAgent = Agent(id: '', name: '', role: role, status: '', organizationId: '', createdAt: DateTime.now());
                        return ChoiceChip(
                          label: Text(dummyAgent.formattedRole),
                          selected: isSelected,
                          onSelected: (selected) {
                            setState(
                              () => _selectedRole = selected ? role : '',
                            );
                            if (selected && _nameController.text.isEmpty) {
                              _nameController.text =
                                  'Senior ${dummyAgent.formattedRole}';
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
                    height: 250,
                    width: double.infinity,
                    clipBehavior: Clip.hardEdge,
                    decoration: BoxDecoration(
                      color: Theme.of(context).colorScheme.surface.withValues(alpha: 0.1),
                      border: Border.all(color: Colors.white24),
                      borderRadius: BorderRadius.circular(12),
                    ),
                    child: Stack(
                      children: [
                        // Dynamic Edge
                        CustomPaint(
                          size: const Size(double.infinity, 250),
                          painter: _TopologyEdgePainter(_mainNodePos, _subNodePos, Theme.of(context).colorScheme.primary),
                        ),
                        // Main Agent Node
                        Positioned(
                          left: _mainNodePos.dx,
                          top: _mainNodePos.dy,
                          child: GestureDetector(
                            onPanUpdate: (details) {
                              setState(() {
                                _mainNodePos = Offset(
                                  (_mainNodePos.dx + details.delta.dx).clamp(0.0, 300.0),
                                  (_mainNodePos.dy + details.delta.dy).clamp(0.0, 150.0),
                                );
                              });
                            },
                            child: Container(
                              width: 100,
                              padding: const EdgeInsets.all(12),
                              decoration: BoxDecoration(
                                color: Theme.of(context).colorScheme.primary.withValues(alpha: 0.2),
                                border: Border.all(color: Theme.of(context).colorScheme.primary),
                                borderRadius: BorderRadius.circular(16),
                                boxShadow: [
                                  BoxShadow(
                                    color: Theme.of(context).colorScheme.primary.withValues(alpha: 0.2),
                                    blurRadius: 10,
                                    spreadRadius: 2,
                                  ),
                                ],
                              ),
                              child: const Column(
                                children: [
                                  Icon(Icons.smart_toy, color: Colors.white, size: 24),
                                  SizedBox(height: 4),
                                  Text('This Agent', style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 10), textAlign: TextAlign.center,),
                                ],
                              ),
                            ),
                          ),
                        ),
                        // Sub Agent Node
                        Positioned(
                          left: _subNodePos.dx,
                          top: _subNodePos.dy,
                          child: GestureDetector(
                            onPanUpdate: (details) {
                              setState(() {
                                _subNodePos = Offset(
                                  (_subNodePos.dx + details.delta.dx).clamp(0.0, 300.0),
                                  (_subNodePos.dy + details.delta.dy).clamp(0.0, 150.0),
                                );
                              });
                            },
                            child: Container(
                              width: 100,
                              padding: const EdgeInsets.all(12),
                              decoration: BoxDecoration(
                                color: Colors.white10,
                                borderRadius: BorderRadius.circular(16),
                                border: Border.all(
                                  color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.2),
                                ),
                              ),
                              child: const Column(
                                children: [
                                  Icon(Icons.code, color: Colors.white70, size: 24),
                                  SizedBox(height: 4),
                                  Text('Code Builder', style: TextStyle(color: Colors.white70, fontSize: 10), textAlign: TextAlign.center,),
                                ],
                              ),
                            ),
                          ),
                        ),
                      ],
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
                          Agent(id: '', name: '', role: _selectedRole, status: '', organizationId: '', createdAt: DateTime.now()).formattedRole,
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

class _TopologyEdgePainter extends CustomPainter {
  final Offset start;
  final Offset end;
  final Color color;

  _TopologyEdgePainter(this.start, this.end, this.color);

  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = color.withValues(alpha: 0.5)
      ..strokeWidth = 2
      ..style = PaintingStyle.stroke;

    final startCenter = start + const Offset(50, 45); // Approximate center of 100x90 node
    final endCenter = end + const Offset(50, 45);

    final path = Path()
      ..moveTo(startCenter.dx, startCenter.dy)
      ..cubicTo(
        startCenter.dx + 50, startCenter.dy,
        endCenter.dx - 50, endCenter.dy,
        endCenter.dx, endCenter.dy,
      );

    canvas.drawPath(path, paint);
  }

  @override
  bool shouldRepaint(covariant _TopologyEdgePainter oldDelegate) {
    return oldDelegate.start != start || oldDelegate.end != end || oldDelegate.color != color;
  }
}
