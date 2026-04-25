import 'package:flutter/material.dart';
import '../widgets/glass_card.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/models/agent.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/settings_service.dart';

class AgentHireWizardScreen extends ConsumerStatefulWidget {
  const AgentHireWizardScreen({super.key});

  @override
  ConsumerState<AgentHireWizardScreen> createState() =>
      _AgentHireWizardScreenState();
}

class _AgentHireWizardScreenState extends ConsumerState<AgentHireWizardScreen> {
  String _selectedRole = '';
  String _selectedProvider = '';
  final _nameController = TextEditingController();

  // Capabilities selection state
  bool _capEmailRead = false;
  bool _capMessagingSend = false;
  bool _capDataAccess = false;

  // Resource limits state
  double _scheduleSliderValue = 1.0; // 0=Weekly, 1=Daily, 2=Hourly, 3=Real-time
  double _maxSessionsPerDay = 50.0;
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
    _endpointUrlController.dispose();
    _tokenLimitController.dispose();
    super.dispose();
  }

  Future<void> _handleDeploy() async {
    setState(() => _isDeploying = true);
    try {
      final api = ref.read(apiServiceProvider);
      if (api != null) {
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

  IconData _getRoleIcon(String role) {
    if (role.toLowerCase().contains('support') || role.toLowerCase().contains('ambassador')) {
      return Icons.support_agent;
    } else if (role.toLowerCase().contains('marketing') || role.toLowerCase().contains('social') || role.toLowerCase().contains('promoter')) {
      return Icons.campaign;
    } else if (role.toLowerCase().contains('seo')) {
      return Icons.search;
    } else if (role.toLowerCase().contains('order') || role.toLowerCase().contains('operations') || role.toLowerCase().contains('manager')) {
      return Icons.inventory;
    } else if (role.toLowerCase().contains('email')) {
      return Icons.email;
    } else if (role.toLowerCase().contains('sales')) {
      return Icons.point_of_sale;
    } else if (role.toLowerCase().contains('finance') || role.toLowerCase().contains('accountant')) {
      return Icons.account_balance;
    } else if (role.toLowerCase().contains('legal') || role.toLowerCase().contains('protector')) {
      return Icons.gavel;
    } else if (role.toLowerCase().contains('advisory') || role.toLowerCase().contains('advisor')) {
      return Icons.lightbulb;
    }
    return Icons.smart_toy;
  }

  String _getRoleDescription(String role) {
    if (role.toLowerCase().contains('support') || role.toLowerCase().contains('ambassador')) {
      return 'Responds to customer messages and handles relationship management.';
    } else if (role.toLowerCase().contains('marketing') || role.toLowerCase().contains('social') || role.toLowerCase().contains('promoter')) {
      return 'Handles website design, social media, and promotions.';
    } else if (role.toLowerCase().contains('order') || role.toLowerCase().contains('operations') || role.toLowerCase().contains('manager')) {
      return 'Manages orders, inventory, and bookings.';
    } else if (role.toLowerCase().contains('sales')) {
      return 'Follows up with leads and suggests upsells.';
    } else if (role.toLowerCase().contains('finance') || role.toLowerCase().contains('accountant')) {
      return 'Tracks revenue and generates financial reports.';
    } else if (role.toLowerCase().contains('legal') || role.toLowerCase().contains('protector')) {
      return 'Generates policies and tracks compliance.';
    } else if (role.toLowerCase().contains('advisory') || role.toLowerCase().contains('advisor')) {
      return 'Analyzes business performance and gives advice.';
    }
    return 'An intelligent AI assistant for your business.';
  }

  @override
  Widget build(BuildContext context) {
    final expertMode = ref.watch(clientSettingsProvider).valueOrNull?.expertMode ?? false;

    return Scaffold(
      backgroundColor: const Color(0xFF0D0D1A),
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        title: const Text(
          'Manage my AI team',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, color: Colors.white),
        ),
        leading: IconButton(
          icon: const Icon(Icons.close, color: Colors.white),
          tooltip: 'Close wizard',
          onPressed: () => context.go('/agents'),
        ),
      ),
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF0D0D1A), Color(0xFF1A1A33)],
          ),
        ),
        child: _isLoading
            ? const Center(child: CircularProgressIndicator())
            : SingleChildScrollView(
                child: Center(
                  child: ConstrainedBox(
                    constraints: const BoxConstraints(maxWidth: 800),
                    child: Padding(
                      padding: const EdgeInsets.all(24.0),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          const Text(
                            'Agent Gallery',
                            style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
                          ),
                          const SizedBox(height: 8),
                          const Text(
                            'Select an AI team member to hire',
                            style: TextStyle(fontFamily: 'Inter', fontSize: 16, color: Colors.white70),
                          ),
                          const SizedBox(height: 24),
                          GridView.builder(
                            shrinkWrap: true,
                            physics: const NeverScrollableScrollPhysics(),
                            gridDelegate: const SliverGridDelegateWithMaxCrossAxisExtent(
                              maxCrossAxisExtent: 350,
                              mainAxisExtent: 180,
                              crossAxisSpacing: 16,
                              mainAxisSpacing: 16,
                            ),
                            itemCount: _roles.length,
                            itemBuilder: (context, index) {
                              final role = _roles[index];
                              final isSelected = _selectedRole == role;
                              final dummyAgent = Agent(id: '', name: '', role: role, status: '', organizationId: '', createdAt: DateTime.now());
                              final formattedRole = dummyAgent.formattedRole;
                              return InkWell(
                                onTap: () {
                                  setState(() {
                                    if (isSelected) {
                                      _selectedRole = '';
                                      _nameController.text = '';
                                    } else {
                                      _selectedRole = role;
                                      _nameController.text = '${formattedRole} Agent';
                                    }
                                  });
                                },
                                child: Container(
                                  decoration: BoxDecoration(
                                    color: isSelected ? Colors.blueAccent.withValues(alpha: 0.2) : Colors.white.withValues(alpha: 0.05),
                                    border: Border.all(color: isSelected ? Colors.blueAccent : Colors.white.withValues(alpha: 0.1)),
                                    borderRadius: BorderRadius.circular(16),
                                  ),
                                  padding: const EdgeInsets.all(16),
                                  child: Column(
                                    crossAxisAlignment: CrossAxisAlignment.start,
                                    children: [
                                      Row(
                                        children: [
                                          CircleAvatar(
                                            backgroundColor: Colors.blueAccent.withValues(alpha: 0.2),
                                            child: Icon(_getRoleIcon(role), color: Colors.blueAccent),
                                          ),
                                          const SizedBox(width: 12),
                                          Expanded(
                                            child: Text(
                                              formattedRole,
                                              style: const TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, fontSize: 16, color: Colors.white),
                                              maxLines: 1,
                                              overflow: TextOverflow.ellipsis,
                                            ),
                                          ),
                                        ],
                                      ),
                                      const SizedBox(height: 12),
                                      Expanded(
                                        child: Text(
                                          _getRoleDescription(role),
                                          style: const TextStyle(fontFamily: 'Inter', fontSize: 12, color: Colors.white70),
                                          maxLines: 3,
                                          overflow: TextOverflow.ellipsis,
                                        ),
                                      ),
                                      Row(
                                        mainAxisAlignment: MainAxisAlignment.end,
                                        children: [
                                          Flexible(
                                            child: Text(
                                              isSelected ? 'Remove' : 'Add to my team',
                                              style: TextStyle(fontFamily: 'Inter', fontSize: 12, color: isSelected ? Colors.redAccent : Colors.greenAccent),
                                              overflow: TextOverflow.ellipsis,
                                            ),
                                          ),
                                          const SizedBox(width: 4),
                                          Switch(
                                            value: isSelected,
                                            onChanged: (val) {
                                              setState(() {
                                                if (val) {
                                                  _selectedRole = role;
                                                  _nameController.text = '${formattedRole} Agent';
                                                } else {
                                                  _selectedRole = '';
                                                  _nameController.text = '';
                                                }
                                              });
                                            },
                                            activeColor: Colors.blueAccent,
                                          ),
                                        ],
                                      ),
                                    ],
                                  ),
                                ),
                              );
                            },
                          ),
                          if (_selectedRole.isNotEmpty) ...[
                            const SizedBox(height: 32),
                            GlassCard(
                              child: Padding(
                                padding: const EdgeInsets.all(24.0),
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    const Text('Capabilities', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
                                    const SizedBox(height: 16),
                                    SwitchListTile(
                                      title: const Text('Reply to customer messages', style: TextStyle(fontFamily: 'Inter', color: Colors.white)),
                                      value: _capMessagingSend,
                                      onChanged: (val) => setState(() => _capMessagingSend = val),
                                      activeColor: Colors.blueAccent,
                                    ),
                                    SwitchListTile(
                                      title: const Text('Send order updates', style: TextStyle(fontFamily: 'Inter', color: Colors.white)),
                                      value: _capEmailRead,
                                      onChanged: (val) => setState(() => _capEmailRead = val),
                                      activeColor: Colors.blueAccent,
                                    ),
                                    SwitchListTile(
                                      title: const Text('Write product descriptions', style: TextStyle(fontFamily: 'Inter', color: Colors.white)),
                                      value: _capDataAccess,
                                      onChanged: (val) => setState(() => _capDataAccess = val),
                                      activeColor: Colors.blueAccent,
                                    ),
                                    const SizedBox(height: 24),
                                    const Text('Schedule', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
                                    const SizedBox(height: 8),
                                    const Text('How often should this agent work?', style: TextStyle(fontFamily: 'Inter', color: Colors.white70)),
                                    const SizedBox(height: 16),
                                    Slider(
                                      value: _scheduleSliderValue,
                                      min: 0,
                                      max: 3,
                                      divisions: 3,
                                      label: _scheduleSliderValue == 0 ? 'Weekly' : (_scheduleSliderValue == 1 ? 'Daily' : (_scheduleSliderValue == 2 ? 'Hourly' : 'Real-time')),
                                      activeColor: Colors.blueAccent,
                                      onChanged: (val) => setState(() {
                                        _scheduleSliderValue = val;
                                        _maxSessionsPerDay = val == 0 ? 10.0 : (val == 1 ? 50.0 : (val == 2 ? 100.0 : 200.0));
                                      }),
                                    ),
                                    Row(
                                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                                      children: const [
                                        Text('Weekly', style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 12)),
                                        Text('Daily', style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 12)),
                                        Text('Hourly', style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 12)),
                                        Text('Real-time', style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 12)),
                                      ],
                                    ),
                                    const SizedBox(height: 24),
                                    Row(
                                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                                      children: [
                                        const Text('Show Advanced Settings', style: TextStyle(fontFamily: 'Inter', color: Colors.white)),
                                        Switch(
                                          value: expertMode,
                                          onChanged: (val) {
                                            final notifier = ref.read(clientSettingsProvider.notifier);
                                            notifier.updateExpertMode(val);
                                          },
                                          activeColor: Colors.blueAccent,
                                        ),
                                      ],
                                    ),
                                    if (expertMode) ...[
                                      const SizedBox(height: 16),
                                      TextField(
                                        controller: _nameController,
                                        style: const TextStyle(color: Colors.white),
                                        decoration: const InputDecoration(labelText: 'Agent Name', labelStyle: TextStyle(color: Colors.white70)),
                                      ),
                                      const SizedBox(height: 16),
                                      DropdownButtonFormField<String>(
                                        value: _selectedProvider,
                                        dropdownColor: const Color(0xFF1A1A33),
                                        style: const TextStyle(color: Colors.white),
                                        decoration: const InputDecoration(labelText: 'Provider', labelStyle: TextStyle(color: Colors.white70)),
                                        items: _providers.map((p) => DropdownMenuItem(value: p.type, child: Text(p.label))).toList(),
                                        onChanged: (val) => setState(() => _selectedProvider = val!),
                                      ),
                                    ],
                                    const SizedBox(height: 32),
                                    const Text('Review & Activate', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
                                    const SizedBox(height: 16),
                                    Container(
                                      padding: const EdgeInsets.all(16),
                                      decoration: BoxDecoration(
                                        color: Colors.black.withValues(alpha: 0.2),
                                        borderRadius: BorderRadius.circular(12),
                                        border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
                                      ),
                                      child: Row(
                                        children: [
                                          CircleAvatar(
                                            backgroundColor: Colors.blueAccent.withValues(alpha: 0.2),
                                            child: Icon(_getRoleIcon(_selectedRole), color: Colors.blueAccent),
                                          ),
                                          const SizedBox(width: 16),
                                          Expanded(
                                            child: Column(
                                              crossAxisAlignment: CrossAxisAlignment.start,
                                              children: [
                                                Text(_nameController.text.isNotEmpty ? _nameController.text : '$_selectedRole Agent', style: const TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, color: Colors.white, fontSize: 16)),
                                                Text('Schedule: ${_maxSessionsPerDay <= 20 ? 'Weekly' : (_maxSessionsPerDay <= 50 ? 'Daily' : (_maxSessionsPerDay <= 100 ? 'Hourly' : 'Real-time'))}', style: const TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 14)),
                                              ],
                                            ),
                                          ),
                                          ElevatedButton(
                                            onPressed: _isDeploying ? null : _handleDeploy,
                                            style: ElevatedButton.styleFrom(
                                              backgroundColor: Colors.green,
                                              padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
                                            ),
                                            child: _isDeploying
                                                ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white))
                                                : const Text('Activate', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                                          ),
                                        ],
                                      ),
                                    ),
                                  ],
                                ),
                              ),
                            ),
                          ],
                        ],
                      ),
                    ),
                  ),
                ),
              ),
      ),
    );
  }
}
