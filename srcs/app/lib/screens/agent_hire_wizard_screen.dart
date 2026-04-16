import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import '../widgets/glass_card.dart';

class AgentHireWizardScreen extends StatefulWidget {
  @override
  _AgentHireWizardScreenState createState() => _AgentHireWizardScreenState();
}

class _AgentHireWizardScreenState extends State<AgentHireWizardScreen> {
  int _currentStep = 0;
  String _selectedRole = '';
  double _budget = 1000.0;
  bool _isLoading = false;

  final List<String> _roles = [
    'UX Researcher',
    'Data Scientist',
    'Security Engineer',
    'Growth Hacker',
  ];

  void _nextStep() {
    setState(() {
      if (_currentStep < 2) {
        _currentStep++;
      }
    });
  }

  void _previousStep() {
    setState(() {
      if (_currentStep > 0) {
        _currentStep--;
      }
    });
  }

  Future<void> _deployAgent() async {
    setState(() {
      _isLoading = true;
    });

    // Simulate API call
    await Future.delayed(Duration(seconds: 2));

    setState(() {
      _isLoading = false;
    });

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('Agent $_selectedRole successfully deployed!'),
        backgroundColor: Colors.green,
      ),
    );

    // Go back to dashboard after a delay
    Future.delayed(Duration(seconds: 1), () {
      Navigator.of(context).pop();
    });
  }

  Widget _buildStepContent() {
    switch (_currentStep) {
      case 0:
        return _buildRoleSelection();
      case 1:
        return _buildBudgetConfiguration();
      case 2:
        return _buildConfirmation();
      default:
        return Container();
    }
  }

  Widget _buildRoleSelection() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Select Agent Role',
          style: Theme.of(context).textTheme.headlineMedium?.copyWith(
            color: Colors.white,
            fontWeight: FontWeight.bold,
          ),
        ),
        SizedBox(height: 24),
        ..._roles.map((role) => Padding(
          padding: const EdgeInsets.only(bottom: 12.0),
          child: InkWell(
            onTap: () {
              setState(() {
                _selectedRole = role;
              });
            },
            child: Container(
              padding: EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: _selectedRole == role
                    ? Colors.white.withOpacity(0.2)
                    : Colors.white.withOpacity(0.05),
                borderRadius: BorderRadius.circular(12),
                border: Border.all(
                  color: _selectedRole == role
                      ? Colors.white.withOpacity(0.5)
                      : Colors.transparent,
                ),
              ),
              child: Row(
                children: [
                  Icon(
                    Icons.psychology,
                    color: _selectedRole == role ? Colors.white : Colors.white54,
                  ),
                  SizedBox(width: 16),
                  Text(
                    role,
                    style: TextStyle(
                      color: _selectedRole == role ? Colors.white : Colors.white70,
                      fontSize: 16,
                      fontWeight: _selectedRole == role ? FontWeight.bold : FontWeight.normal,
                    ),
                  ),
                ],
              ),
            ),
          ),
        )).toList(),
      ],
    );
  }

  Widget _buildBudgetConfiguration() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Configure Autonomous Budget',
          style: Theme.of(context).textTheme.headlineMedium?.copyWith(
            color: Colors.white,
            fontWeight: FontWeight.bold,
          ),
        ),
        SizedBox(height: 16),
        Text(
          'Set the monthly token allocation for this agent.',
          style: TextStyle(color: Colors.white70),
        ),
        SizedBox(height: 40),
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(
              '\$500',
              style: TextStyle(color: Colors.white54),
            ),
            Text(
              '\$${_budget.toInt()}',
              style: TextStyle(
                color: Colors.white,
                fontSize: 24,
                fontWeight: FontWeight.bold,
              ),
            ),
            Text(
              '\$5000',
              style: TextStyle(color: Colors.white54),
            ),
          ],
        ),
        Slider(
          value: _budget,
          min: 500,
          max: 5000,
          divisions: 45,
          activeColor: Colors.white,
          inactiveColor: Colors.white24,
          onChanged: (value) {
            setState(() {
              _budget = value;
            });
          },
        ),
      ],
    );
  }

  Widget _buildConfirmation() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Confirm Deployment',
          style: Theme.of(context).textTheme.headlineMedium?.copyWith(
            color: Colors.white,
            fontWeight: FontWeight.bold,
          ),
        ),
        SizedBox(height: 32),
        GlassCard(
          child: Padding(
            padding: const EdgeInsets.all(24.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.rocket_launch, color: Colors.white, size: 28),
                    SizedBox(width: 16),
                    Text(
                      'Mission Summary',
                      style: TextStyle(
                        color: Colors.white,
                        fontSize: 20,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ],
                ),
                SizedBox(height: 24),
                _buildSummaryRow('Role', _selectedRole),
                SizedBox(height: 12),
                _buildSummaryRow('Budget', '\$${_budget.toInt()}/month'),
                SizedBox(height: 12),
                _buildSummaryRow('Access Level', 'Autonomous'),
                SizedBox(height: 12),
                _buildSummaryRow('Reporting', 'Weekly Diagnostics'),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildSummaryRow(String label, String value) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Text(
          label,
          style: TextStyle(color: Colors.white70, fontSize: 16),
        ),
        Text(
          value,
          style: TextStyle(color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold),
        ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Color(0xFF1A1A2E), // Deep space background
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        title: Text('New Agent Provisioning', style: TextStyle(color: Colors.white)),
        iconTheme: IconThemeData(color: Colors.white),
      ),
      body: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            children: [
              // Progress indicators
              Row(
                children: List.generate(3, (index) {
                  return Expanded(
                    child: Container(
                      height: 4,
                      margin: EdgeInsets.symmetric(horizontal: 4),
                      decoration: BoxDecoration(
                        color: index <= _currentStep ? Colors.white : Colors.white24,
                        borderRadius: BorderRadius.circular(2),
                      ),
                    ),
                  );
                }),
              ),
              SizedBox(height: 40),

              // Main content area
              Expanded(
                child: _buildStepContent(),
              ),

              // Bottom navigation
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  if (_currentStep > 0)
                    TextButton(
                      onPressed: _previousStep,
                      child: Text('Back', style: TextStyle(color: Colors.white70)),
                    )
                  else
                    SizedBox.shrink(),

                  ElevatedButton(
                    onPressed: _currentStep == 0 && _selectedRole.isEmpty
                        ? null
                        : _currentStep == 2
                            ? _deployAgent
                            : _nextStep,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.white,
                      foregroundColor: Color(0xFF1A1A2E),
                      padding: EdgeInsets.symmetric(horizontal: 32, vertical: 16),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(30),
                      ),
                    ),
                    child: _isLoading
                        ? SizedBox(
                            width: 20,
                            height: 20,
                            child: CircularProgressIndicator(
                              strokeWidth: 2,
                              valueColor: AlwaysStoppedAnimation<Color>(Color(0xFF1A1A2E)),
                            ),
                          )
                        : Text(
                            _currentStep == 2 ? 'Deploy Swarm' : 'Next',
                            style: TextStyle(fontWeight: FontWeight.bold),
                          ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
