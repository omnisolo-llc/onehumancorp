import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../widgets/glass_card.dart';

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  ConsumerState<BusinessSetupWizardScreen> createState() => _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState extends ConsumerState<BusinessSetupWizardScreen> {
  int _step = 0;

  // State
  String _companyName = '';
  String _industry = '';
  String _size = '';
  final Set<String> _goals = {};
  String _deployment = '';
  String _adminName = '';
  String _adminEmail = '';
  String _adminPassword = '';

  void _nextStep() {
    if (_step < 4) {
      setState(() => _step++);
    } else {
      _launch();
    }
  }

  void _prevStep() {
    if (_step > 0) {
      setState(() => _step--);
    }
  }

  void _launch() {
    // Send data to backend
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black, // Dark background to make glassmorphism pop
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: Padding(
            padding: const EdgeInsets.all(24.0),
            child: GlassCard(
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  const Text(
                    'Welcome to OHC Agentic OS',
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 28,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                    ),
                    textAlign: TextAlign.center,
                  ),
                  const SizedBox(height: 32),
                  _buildStepContent(),
                  const SizedBox(height: 32),
                  _buildNavigation(),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildStepContent() {
    switch (_step) {
      case 0:
        return _buildBusinessProfileStep();
      case 1:
        return _buildGoalSelectionStep();
      case 2:
        return _buildDeploymentStep();
      case 3:
        return _buildAdminStep();
      case 4:
        return _buildReviewStep();
      default:
        return const SizedBox.shrink();
    }
  }

  Widget _buildBusinessProfileStep() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text("Business Profile", style: TextStyle(color: Colors.white, fontSize: 20, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        TextField(
          decoration: InputDecoration(hintText: "Company Name", filled: true, fillColor: Colors.white.withValues(alpha: 0.1)),
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
          onChanged: (val) => setState(() => _companyName = val),
        ),
        const SizedBox(height: 16),
        TextField(
          decoration: InputDecoration(hintText: "Industry", filled: true, fillColor: Colors.white.withValues(alpha: 0.1)),
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
          onChanged: (val) => setState(() => _industry = val),
        ),
        const SizedBox(height: 16),
        TextField(
          decoration: InputDecoration(hintText: "Size", filled: true, fillColor: Colors.white.withValues(alpha: 0.1)),
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
          onChanged: (val) => setState(() => _size = val),
        ),
      ],
    );
  }

  Widget _buildGoalSelectionStep() {
    final options = ['Support', 'Build software', 'Marketing', 'Data', 'Custom'];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text("Goal Selection", style: TextStyle(color: Colors.white, fontSize: 20, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        Wrap(
          spacing: 8,
          children: options.map((o) {
            final isSelected = _goals.contains(o);
            return ChoiceChip(
              label: Text(o, style: const TextStyle(fontFamily: 'Inter')),
              selected: isSelected,
              onSelected: (_) {
                setState(() {
                  if (isSelected) {
                    _goals.remove(o);
                  } else {
                    _goals.add(o);
                  }
                });
              },
              selectedColor: Colors.blueAccent,
            );
          }).toList(),
        )
      ],
    );
  }

  Widget _buildDeploymentStep() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text("Deployment Preference", style: TextStyle(color: Colors.white, fontSize: 20, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        DropdownButton<String>(
          value: _deployment.isEmpty ? null : _deployment,
          hint: const Text("Select Deployment", style: TextStyle(color: Colors.white54, fontFamily: 'Inter')),
          dropdownColor: Colors.grey[900],
          items: ['Cloud', 'Desktop', 'Mobile-only'].map((String value) {
            return DropdownMenuItem<String>(
              value: value,
              child: Text(value, style: const TextStyle(color: Colors.white, fontFamily: 'Inter')),
            );
          }).toList(),
          onChanged: (val) {
            if (val != null) setState(() => _deployment = val);
          },
        ),
      ],
    );
  }

  Widget _buildAdminStep() {
     return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text("Administrator Account", style: TextStyle(color: Colors.white, fontSize: 20, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        TextField(
          decoration: InputDecoration(hintText: "Name", filled: true, fillColor: Colors.white.withValues(alpha: 0.1)),
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
          onChanged: (val) => setState(() => _adminName = val),
        ),
        const SizedBox(height: 16),
        TextField(
          decoration: InputDecoration(hintText: "Email", filled: true, fillColor: Colors.white.withValues(alpha: 0.1)),
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
          onChanged: (val) => setState(() => _adminEmail = val),
        ),
        const SizedBox(height: 16),
        TextField(
          obscureText: true,
          decoration: InputDecoration(hintText: "Password", filled: true, fillColor: Colors.white.withValues(alpha: 0.1)),
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
          onChanged: (val) => setState(() => _adminPassword = val),
        ),
      ],
    );
  }

  Widget _buildReviewStep() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text("Review & Launch", style: TextStyle(color: Colors.white, fontSize: 20, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        Text("Company: $_companyName", style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
        Text("Industry: $_industry", style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
        Text("Size: $_size", style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
        Text("Goals: ${_goals.join(', ')}", style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
        Text("Deployment: $_deployment", style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
        Text("Admin: $_adminName ($_adminEmail)", style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
      ],
    );
  }

  Widget _buildNavigation() {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        if (_step > 0)
          TextButton(
            onPressed: _prevStep,
            child: const Text('Back', style: TextStyle(color: Colors.white54, fontFamily: 'Inter')),
          )
        else
          const SizedBox.shrink(),

        if (_step < 4)
          ElevatedButton(
            onPressed: _nextStep,
            child: const Text('Next', style: TextStyle(fontFamily: 'Inter')),
          )
        else
          ElevatedButton(
            onPressed: _launch,
            style: ElevatedButton.styleFrom(backgroundColor: Colors.blueAccent),
            child: const Text('Launch My AI Team', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
          ),
      ],
    );
  }
}
