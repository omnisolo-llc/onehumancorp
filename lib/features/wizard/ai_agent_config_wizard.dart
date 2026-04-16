import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class AiAgentConfigWizard extends ConsumerStatefulWidget {
  const AiAgentConfigWizard({super.key});

  @override
  ConsumerState<AiAgentConfigWizard> createState() => _AiAgentConfigWizardState();
}

class _AiAgentConfigWizardState extends ConsumerState<AiAgentConfigWizard> {
  int _step = 0;
  bool _isLoading = false;
  bool _isDeploying = false;
  bool _expertMode = false;

  String _selectedRole = '';
  String _selectedProvider = '';
  final _nameController = TextEditingController();

  final Map<String, bool> _capabilities = {
    'Read my emails': false,
    'Send messages on my behalf': false,
    'Manage calendar': false,
    'Execute code': false,
  };

  double _workHours = 4.0;
  double get _estimatedCost => _workHours * 2.50;

  final List<Map<String, dynamic>> _agentCategories = [
    {
      'category': 'Support',
      'roles': [
        {'id': 'customer_support', 'name': 'Customer Support Agent', 'desc': 'Handles tickets and user queries'},
        {'id': 'technical_support', 'name': 'Technical Support', 'desc': 'Troubleshoots complex issues'}
      ]
    },
    {
      'category': 'Engineering',
      'roles': [
        {'id': 'code_builder', 'name': 'Code Builder', 'desc': 'Writes and reviews software'},
        {'id': 'qa_tester', 'name': 'QA Tester', 'desc': 'Writes automated tests'}
      ]
    },
    {
      'category': 'Data',
      'roles': [
        {'id': 'data_analyst', 'name': 'Data Analyst', 'desc': 'Creates SQL queries and charts'}
      ]
    },
    {
      'category': 'Marketing',
      'roles': [
        {'id': 'seo_specialist', 'name': 'SEO Specialist', 'desc': 'Optimizes content'}
      ]
    },
    {
      'category': 'Finance',
      'roles': [
        {'id': 'accountant', 'name': 'Accountant', 'desc': 'Manages books'}
      ]
    },
    {
      'category': 'Operations',
      'roles': [
        {'id': 'ops_manager', 'name': 'Operations Manager', 'desc': 'Oversees tasks'}
      ]
    }
  ];

  final List<Map<String, String>> _providers = [
    {'id': 'openai', 'name': 'OpenAI', 'baseUrl': 'https://api.openai.com/v1'},
    {'id': 'anthropic', 'name': 'Anthropic', 'baseUrl': 'https://api.anthropic.com/v1'},
  ];

  @override
  void dispose() {
    _nameController.dispose();
    super.dispose();
  }

  String _formatRole(String r) {
    if (r.isEmpty) return '';
    return r[0].toUpperCase() + r.substring(1);
  }

  void _handleDeploy() async {
    setState(() => _isDeploying = true);
    await Future.delayed(const Duration(seconds: 2));

    if (mounted) {
      setState(() => _isDeploying = false);
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Agent Activated Successfully!')),
      );
    }
  }

  Widget _buildStep1Gallery() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Step 1 — Choose an Agent', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 8),
        const Text('Select a pre-configured agent to add to your team.', style: TextStyle(fontFamily: 'Inter')),
        const SizedBox(height: 16),
        ..._agentCategories.map((cat) {
          return Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Padding(
                padding: const EdgeInsets.symmetric(vertical: 8.0),
                child: Text(cat['category'], style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 16, fontFamily: 'Inter')),
              ),
              GridView.count(
                shrinkWrap: true,
                physics: const NeverScrollableScrollPhysics(),
                crossAxisCount: MediaQuery.of(context).size.width > 600 ? 3 : 1,
                childAspectRatio: 2.5,
                mainAxisSpacing: 8,
                crossAxisSpacing: 8,
                children: (cat['roles'] as List).map<Widget>((role) {
                  final isSelected = _selectedRole == role['id'];
                  return InkWell(
                    onTap: () {
                      setState(() {
                        _selectedRole = role['id'];
                        _nameController.text = role['name'];
                      });
                    },
                    child: Container(
                      padding: const EdgeInsets.all(12),
                      decoration: BoxDecoration(
                        color: isSelected
                            ? Theme.of(context).colorScheme.primary.withOpacity(0.2)
                            : Theme.of(context).colorScheme.surface.withOpacity(0.1),
                        borderRadius: BorderRadius.circular(12),
                        border: Border.all(
                          color: isSelected ? Theme.of(context).colorScheme.primary : Theme.of(context).colorScheme.onSurface.withOpacity(0.2),
                        ),
                      ),
                      child: Row(
                        children: [
                          CircleAvatar(
                            backgroundColor: Theme.of(context).colorScheme.primary.withOpacity(0.1),
                            child: const Icon(Icons.smart_toy),
                          ),
                          const SizedBox(width: 12),
                          Expanded(
                            child: SingleChildScrollView(
                              child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              mainAxisAlignment: MainAxisAlignment.center,
                              children: [
                                Text(role['name'], style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
                                Text(role['desc'], style: const TextStyle(fontSize: 12, fontFamily: 'Inter'), maxLines: 2, overflow: TextOverflow.ellipsis),
                              ],
                            ),
                            ),
                          ),
                          if (isSelected) const Icon(Icons.check_circle, color: Colors.green),
                        ],
                      ),
                    ),
                  );
                }).toList(),
              ),
              const SizedBox(height: 16),
            ],
          );
        }).toList(),
      ],
    );
  }

  Widget _buildStep2Topology() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            const Text('Step 2 — Sub-agent Topology', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
            Row(
              children: [
                const Text('Expert Mode', style: TextStyle(fontFamily: 'Inter', fontSize: 12)),
                Switch(value: _expertMode, onChanged: (v) => setState(() => _expertMode = v)),
              ],
            )
          ],
        ),
        const SizedBox(height: 8),
        const Text('Drag and drop to connect this agent to others.', style: TextStyle(fontFamily: 'Inter')),
        const SizedBox(height: 16),
        Container(
          height: 200,
          decoration: BoxDecoration(color: Colors.black12, borderRadius: BorderRadius.circular(12), border: Border.all(color: Colors.white24)),
          child: Stack(
            children: [
              Positioned(
                left: 50,
                top: 50,
                child: DraggableNode(label: 'Agent 1', color: Colors.blue),
              ),
              Positioned(
                left: 200,
                top: 100,
                child: DraggableNode(label: 'Agent 2', color: Colors.green),
              ),
            ],
          ),
        ),
        if (_expertMode) ...[
          const SizedBox(height: 16),
          const Text('Advanced Provider Settings', style: TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Inter')),
          const SizedBox(height: 8),
          ..._providers.map((p) => RadioListTile<String>(
            title: Text(p['name']!, style: const TextStyle(fontFamily: 'Inter')),
            subtitle: Text(p['baseUrl']!, style: const TextStyle(fontFamily: 'Inter')),
            value: p['id']!,
            groupValue: _selectedProvider,
            onChanged: (val) => setState(() => _selectedProvider = val!),
          )),
          const SizedBox(height: 8),
          const TextField(decoration: InputDecoration(labelText: 'Token Limit Override', border: OutlineInputBorder())),
        ]
      ],
    );
  }

  Widget _buildStep3Capabilities() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Step 3 — Capability Selection', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 8),
        const Text('What should this agent be allowed to do?', style: TextStyle(fontFamily: 'Inter')),
        const SizedBox(height: 16),
        ..._capabilities.keys.map((cap) {
          return CheckboxListTile(
            title: Text(cap, style: const TextStyle(fontFamily: 'Inter')),
            subtitle: _expertMode ? Text('PERMISSION_${cap.replaceAll(' ', '_').toUpperCase()}', style: const TextStyle(fontSize: 10, fontFamily: 'monospace')) : null,
            value: _capabilities[cap],
            onChanged: (val) {
              if (val != null) setState(() => _capabilities[cap] = val);
            },
          );
        }).toList(),
      ],
    );
  }

  Widget _buildStep4Resources() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Step 4 — Resource Limits', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 8),
        const Text('How much should this agent work per day?', style: TextStyle(fontFamily: 'Inter')),
        const SizedBox(height: 16),
        Row(
          children: [
            Expanded(
              child: Slider(
                value: _workHours,
                min: 1,
                max: 24,
                divisions: 23,
                label: '${_workHours.round()} hours',
                onChanged: (val) => setState(() => _workHours = val),
              ),
            ),
            Text('${_workHours.round()}h', style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Inter')),
          ],
        ),
        const SizedBox(height: 16),
        Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(color: Theme.of(context).colorScheme.primary.withOpacity(0.1), borderRadius: BorderRadius.circular(12)),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text('Estimated Daily Cost:', style: TextStyle(fontFamily: 'Inter')),
              Text('\$${_estimatedCost.toStringAsFixed(2)}', style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
            ],
          ),
        ),
      ],
    );
  }

  Widget _buildStep5Review() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Step 5 — Confirm Deployment', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        TextField(
          controller: _nameController,
          decoration: const InputDecoration(labelText: 'Agent Name (Optional override)', border: OutlineInputBorder()),
        ),
        const SizedBox(height: 16),
        ClipRRect(
          borderRadius: BorderRadius.circular(16),
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            child: Container(
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.surface.withOpacity(0.1),
                borderRadius: BorderRadius.circular(16),
                border: Border.all(color: Theme.of(context).colorScheme.onSurface.withOpacity(0.2)),
              ),
              child: ListTile(
                leading: CircleAvatar(
                  backgroundColor: Theme.of(context).colorScheme.primary.withOpacity(0.2),
                  child: const Icon(Icons.smart_toy),
                ),
                title: Text(_nameController.text.isNotEmpty ? _nameController.text : 'New Agent', style: const TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                subtitle: Text('Role: ${_formatRole(_selectedRole)} • Cost: \$${_estimatedCost.toStringAsFixed(2)}/day', style: const TextStyle(fontFamily: 'Inter')),
              ),
            ),
          ),
        ),
        const SizedBox(height: 16),
        Text(
          'This agent will be immediately provisioned with a SPIFFE identity and connected to the orchestration hub.',
          style: TextStyle(color: Theme.of(context).colorScheme.onSurfaceVariant.withOpacity(0.7), fontFamily: 'Inter'),
        ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    return Stepper(
      type: StepperType.vertical,
      currentStep: _step,
      onStepContinue: () {
        if (_step < 4) {
          setState(() => _step += 1);
        }
      },
      onStepCancel: () {
        if (_step > 0) {
          setState(() => _step -= 1);
        }
      },
      controlsBuilder: (context, details) {
        return Padding(
          padding: const EdgeInsets.only(top: 24.0),
          child: Row(
            children: [
              if (_step < 4)
                ElevatedButton(
                  onPressed: (_step == 0 && _selectedRole.isEmpty) ? null : details.onStepContinue,
                  child: const Text('Next'),
                )
              else
                ElevatedButton(
                  onPressed: _isDeploying ? null : _handleDeploy,
                  child: _isDeploying
                    ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                    : const Text('Deploy Agent'),
                ),
              const SizedBox(width: 12),
              if (_step > 0)
                TextButton(
                  onPressed: details.onStepCancel,
                  child: const Text('Back'),
                ),
            ],
          ),
        );
      },
      steps: [
        Step(title: const Text('Role'), isActive: _step >= 0, content: _buildStep1Gallery()),
        Step(title: const Text('Topology'), isActive: _step >= 1, content: _buildStep2Topology()),
        Step(title: const Text('Capabilities'), isActive: _step >= 2, content: _buildStep3Capabilities()),
        Step(title: const Text('Resources'), isActive: _step >= 3, content: _buildStep4Resources()),
        Step(title: const Text('Confirm'), isActive: _step >= 4, content: _buildStep5Review()),
      ],
    );
  }
}

class DraggableNode extends StatefulWidget {
  final String label;
  final Color color;

  const DraggableNode({Key? key, required this.label, required this.color}) : super(key: key);

  @override
  State<DraggableNode> createState() => _DraggableNodeState();
}

class _DraggableNodeState extends State<DraggableNode> {
  Offset position = const Offset(0, 0);

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onPanUpdate: (details) {
        setState(() {
          position += details.delta;
        });
      },
      child: Transform.translate(
        offset: position,
        child: Container(
          padding: const EdgeInsets.all(12),
          decoration: BoxDecoration(
            color: widget.color.withOpacity(0.8),
            borderRadius: BorderRadius.circular(8),
            boxShadow: const [BoxShadow(color: Colors.black26, blurRadius: 4, spreadRadius: 2)],
          ),
          child: Text(
            widget.label,
            style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold),
          ),
        ),
      ),
    );
  }
}
