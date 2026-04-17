import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class PromptTuningWizard extends ConsumerStatefulWidget {
  const PromptTuningWizard({super.key});

  @override
  ConsumerState<PromptTuningWizard> createState() => _PromptTuningWizardState();
}

class _PromptTuningWizardState extends ConsumerState<PromptTuningWizard> {
  int _step = 0;
  bool _isSaving = false;

  String _personality = 'Friendly';
  final List<String> _domainFocus = [];
  final _exampleQController = TextEditingController();
  final _exampleAController = TextEditingController();
  final List<Map<String, String>> _examples = [];
  final _domainController = TextEditingController();

  final List<String> _personalities = ['Formal', 'Friendly', 'Concise', 'Detailed', 'Custom'];

  Widget _buildStep1Personality() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Step 1 — Personality & Tone', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        ..._personalities.map((p) => RadioListTile<String>(
          title: Text(p, style: const TextStyle(fontFamily: 'Inter')),
          value: p,
          groupValue: _personality,
          onChanged: (val) => setState(() => _personality = val!),
        )),
      ],
    );
  }

  Widget _buildStep2Domain() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Step 2 — Domain Focus', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        Row(
          children: [
            Expanded(
              child: TextField(
                controller: _domainController,
                decoration: const InputDecoration(labelText: 'Add a rule (e.g. Always reply in Spanish)'),
              ),
            ),
            IconButton(
              icon: const Icon(Icons.add),
              onPressed: () {
                if (_domainController.text.isNotEmpty) {
                  setState(() {
                    _domainFocus.add(_domainController.text);
                    _domainController.clear();
                  });
                }
              },
            )
          ],
        ),
        const SizedBox(height: 16),
        Wrap(
          spacing: 8,
          children: _domainFocus.map((d) => Chip(
            label: Text(d),
            onDeleted: () => setState(() => _domainFocus.remove(d)),
          )).toList(),
        )
      ],
    );
  }

  Widget _buildStep3Examples() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Step 3 — Example Interactions', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        TextField(controller: _exampleQController, decoration: const InputDecoration(labelText: 'User says...')),
        const SizedBox(height: 8),
        TextField(controller: _exampleAController, decoration: const InputDecoration(labelText: 'Agent replies...')),
        const SizedBox(height: 8),
        ElevatedButton(
          onPressed: () {
            if (_examples.length < 3 && _exampleQController.text.isNotEmpty && _exampleAController.text.isNotEmpty) {
              setState(() {
                _examples.add({'q': _exampleQController.text, 'a': _exampleAController.text});
                _exampleQController.clear();
                _exampleAController.clear();
              });
            }
          },
          child: const Text('Add Example'),
        ),
        const SizedBox(height: 16),
        ..._examples.map((e) => ListTile(
          title: Text('Q: ${e['q']}'),
          subtitle: Text('A: ${e['a']}'),
        ))
      ],
    );
  }

  Widget _buildStep4Preview() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Step 4 — Live Preview', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        ClipRRect(
          borderRadius: BorderRadius.circular(16),
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            child: Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.surface.withOpacity(0.1),
                border: Border.all(color: Theme.of(context).colorScheme.onSurface.withOpacity(0.2)),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('System Prompt Generated:', style: TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
                  const SizedBox(height: 8),
                  Text('You are a $_personality AI assistant.\nRules:\n${_domainFocus.join('\n')}\nExamples:\n${_examples.map((e) => "User: ${e['q']}\nAgent: ${e['a']}").join('\n')}', style: const TextStyle(fontFamily: 'Inter')),
                ],
              ),
            ),
          ),
        ),
      ],
    );
  }

  void _handleSave() async {
    setState(() => _isSaving = true);
    await Future.delayed(const Duration(seconds: 1));
    if (mounted) {
      setState(() => _isSaving = false);
      ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Your agent has been updated ✓')));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Stepper(
      currentStep: _step,
      onStepContinue: () {
        if (_step < 3) setState(() => _step += 1);
      },
      onStepCancel: () {
        if (_step > 0) setState(() => _step -= 1);
      },
      controlsBuilder: (context, details) {
        return Padding(
          padding: const EdgeInsets.only(top: 24.0),
          child: Row(
            children: [
              if (_step < 3)
                ElevatedButton(onPressed: details.onStepContinue, child: const Text('Next'))
              else
                ElevatedButton(
                  onPressed: _isSaving ? null : _handleSave,
                  child: _isSaving ? const CircularProgressIndicator() : const Text('Save Settings'),
                ),
              const SizedBox(width: 12),
              if (_step > 0)
                TextButton(onPressed: details.onStepCancel, child: const Text('Back')),
            ],
          ),
        );
      },
      steps: [
        Step(title: const Text('Personality'), isActive: _step >= 0, content: _buildStep1Personality()),
        Step(title: const Text('Domain'), isActive: _step >= 1, content: _buildStep2Domain()),
        Step(title: const Text('Examples'), isActive: _step >= 2, content: _buildStep3Examples()),
        Step(title: const Text('Preview'), isActive: _step >= 3, content: _buildStep4Preview()),
      ],
    );
  }
}