import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/services/api_service.dart';

class PromptTuningWizardScreen extends ConsumerStatefulWidget {
  final String agentId;
  const PromptTuningWizardScreen({super.key, required this.agentId});

  @override
  ConsumerState<PromptTuningWizardScreen> createState() => _PromptTuningWizardScreenState();
}

class _PromptTuningWizardScreenState extends ConsumerState<PromptTuningWizardScreen> {
  String _personality = 'Formal';
  final List<String> _domainFocus = [];
  final _domainController = TextEditingController();
  final List<Map<String, String>> _examples = [{}, {}, {}];
  bool _isSaving = false;

  String get _generatedPrompt {
    return "Act as a $_personality AI assistant.\n\nDomain focus: ${_domainFocus.join(', ')}\n\nExamples:\n${_examples.where((e) => e.isNotEmpty).map((e) => "Q: ${e['q']}\nA: ${e['a']}").join('\n\n')}";
  }

  Future<void> _save() async {
    setState(() => _isSaving = true);
    try {
      final api = ref.read(apiServiceProvider);
      if (api != null) {
        await api.updateAgentPrompt(widget.agentId, _generatedPrompt);
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Your agent has been updated ✓')));
          context.pop();
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Error: $e')));
      }
    } finally {
      if (mounted) {
        setState(() => _isSaving = false);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    return Scaffold(
      appBar: AppBar(title: Text('Tune Agent ${widget.agentId}', style: const TextStyle(fontFamily: 'Outfit'))),
      body: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Row(
          children: [
            Expanded(
              flex: 1,
              child: ClipRRect(
                borderRadius: BorderRadius.circular(16),
                child: BackdropFilter(
                  filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
                  child: Container(
                    padding: const EdgeInsets.all(24),
                    decoration: BoxDecoration(
                      color: colorScheme.surface.withValues(alpha: 0.03),
                      borderRadius: BorderRadius.circular(16),
                      border: Border.all(color: colorScheme.outlineVariant.withValues(alpha: 0.2)),
                    ),
                    child: SingleChildScrollView(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          const Text('Personality & Tone', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold)),
                          const SizedBox(height: 12),
                          Wrap(
                            spacing: 8,
                            children: ['Formal', 'Friendly', 'Concise', 'Detailed', 'Custom'].map((p) => ChoiceChip(
                              label: Text(p),
                              selected: _personality == p,
                              onSelected: (s) => setState(() => _personality = s ? p : _personality),
                            )).toList(),
                          ),
                          const SizedBox(height: 24),
                          const Text('Domain Focus', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold)),
                          const SizedBox(height: 12),
                          Row(
                            children: [
                              Expanded(child: TextField(controller: _domainController, decoration: const InputDecoration(hintText: 'Add domain (e.g. Sales)'))),
                              IconButton(icon: const Icon(Icons.add), onPressed: () {
                                if (_domainController.text.isNotEmpty) {
                                  setState(() { _domainFocus.add(_domainController.text); _domainController.clear(); });
                                }
                              }),
                            ],
                          ),
                          Wrap(
                            spacing: 8,
                            children: _domainFocus.map((d) => Chip(label: Text(d), onDeleted: () => setState(() => _domainFocus.remove(d)))).toList(),
                          ),
                          const SizedBox(height: 24),
                          const Text('Example Interactions', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold)),
                          const SizedBox(height: 12),
                          for (int i = 0; i < 3; i++)
                            Padding(
                              padding: const EdgeInsets.only(bottom: 12.0),
                              child: Column(
                                children: [
                                  TextField(decoration: InputDecoration(hintText: 'User Q ${i+1}'), onChanged: (v) => setState(() => _examples[i]['q'] = v)),
                                  const SizedBox(height: 8),
                                  TextField(decoration: InputDecoration(hintText: 'Agent A ${i+1}'), onChanged: (v) => setState(() => _examples[i]['a'] = v)),
                                ],
                              ),
                            ),
                          const SizedBox(height: 24),
                          SizedBox(
                            width: double.infinity,
                            child: FilledButton(
                              onPressed: _isSaving ? null : _save,
                              child: _isSaving ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator()) : const Text('Save Prompt'),
                            ),
                          )
                        ],
                      ),
                    ),
                  ),
                ),
              ),
            ),
            const SizedBox(width: 24),
            Expanded(
              flex: 1,
              child: ClipRRect(
                borderRadius: BorderRadius.circular(16),
                child: BackdropFilter(
                  filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
                  child: Container(
                    padding: const EdgeInsets.all(24),
                    decoration: BoxDecoration(
                      color: colorScheme.surface.withValues(alpha: 0.03),
                      borderRadius: BorderRadius.circular(16),
                      border: Border.all(color: colorScheme.outlineVariant.withValues(alpha: 0.2)),
                    ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text('Live Preview', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold)),
                        const SizedBox(height: 16),
                        Expanded(
                          child: Container(
                            width: double.infinity,
                            padding: const EdgeInsets.all(16),
                            decoration: BoxDecoration(color: Colors.black12, borderRadius: BorderRadius.circular(8)),
                            child: SingleChildScrollView(child: Text(_generatedPrompt, style: const TextStyle(fontFamily: 'Inter', fontSize: 14))),
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
