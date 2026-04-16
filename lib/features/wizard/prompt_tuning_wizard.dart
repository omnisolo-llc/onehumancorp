import 'dart:ui';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:http/http.dart' as http;

class PromptTuningWizard extends ConsumerStatefulWidget {
  const PromptTuningWizard({super.key});

  @override
  ConsumerState<PromptTuningWizard> createState() => _PromptTuningWizardState();
}

class _PromptTuningWizardState extends ConsumerState<PromptTuningWizard> {
  int _step = 0;
  bool _isLoading = false;

  String _selectedTone = 'Formal';

  final Map<String, bool> _domainFocus = {
    'Only discuss topics related to my business': false,
    'Avoid competitor mentions': false,
    'Always reply in Spanish': false,
  };

  final List<TextEditingController> _qControllers = List.generate(3, (_) => TextEditingController());
  final List<TextEditingController> _aControllers = List.generate(3, (_) => TextEditingController());

  // Chat Sandbox State
  final List<Map<String, String>> _messages = [];
  final TextEditingController _chatController = TextEditingController();
  bool _isChatLoading = false;

  // Use a proper baseUrl to support both web and mobile
  final String _baseUrl = const String.fromEnvironment('API_BASE_URL', defaultValue: 'http://127.0.0.1:8080');

  String get _generatedPrompt {
    String base = 'You are an AI assistant. ';
    base += 'Tone: $_selectedTone. ';
    List<String> activeDomains = _domainFocus.entries.where((e) => e.value).map((e) => e.key).toList();
    if (activeDomains.isNotEmpty) {
      base += 'Rules: ${activeDomains.join(', ')}. ';
    }

    int exampleCount = 0;
    for (int i=0; i<3; i++) {
       if (_qControllers[i].text.isNotEmpty && _aControllers[i].text.isNotEmpty) {
           if (exampleCount == 0) base += 'Examples: ';
           base += '\nQ: ${_qControllers[i].text}\nA: ${_aControllers[i].text}';
           exampleCount++;
       }
    }
    return base;
  }

  void _savePrompt() async {
    setState(() => _isLoading = true);

    List<Map<String, String>> examples = [];
    for (int i=0; i<3; i++) {
        if (_qControllers[i].text.isNotEmpty && _aControllers[i].text.isNotEmpty) {
            examples.add({'q': _qControllers[i].text, 'a': _aControllers[i].text});
        }
    }

    final payload = {
        'agent_id': 'default-agent',
        'tone': _selectedTone,
        'domain_focus': _domainFocus.entries.where((e) => e.value).map((e) => e.key).toList(),
        'examples': examples,
        'system_prompt': _generatedPrompt,
    };

    try {
        await http.post(
            Uri.parse('$_baseUrl/api/wizard/prompt/tune'),
            headers: {'Content-Type': 'application/json'},
            body: jsonEncode(payload)
        );
        if (mounted) {
             ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Your agent has been updated ✓')));
        }
    } catch(e) {
         if (mounted) {
             ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Error: $e')));
         }
    } finally {
        setState(() => _isLoading = false);
    }
  }

  void _sendMessage() async {
    if (_chatController.text.isEmpty) return;

    setState(() {
      _messages.add({'role': 'user', 'content': _chatController.text});
      _isChatLoading = true;
    });

    final query = _chatController.text;
    _chatController.clear();

    try {
      final response = await http.post(
        Uri.parse('$_baseUrl/api/wizard/prompt/preview'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'system_prompt': _generatedPrompt,
          'message': query
        })
      );

      final data = jsonDecode(response.body);
      setState(() {
         _messages.add({'role': 'agent', 'content': data['reply'] ?? 'Mock reply based on tune.'});
      });
    } catch (e) {
      setState(() {
         _messages.add({'role': 'agent', 'content': 'Sandbox simulated response based on $_selectedTone tone.'});
      });
    } finally {
      setState(() => _isChatLoading = false);
    }
  }

  Widget _buildGlassmorphism({required Widget child}) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: Colors.white.withOpacity(0.03),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(color: Colors.white.withOpacity(0.1)),
          ),
          child: child,
        ),
      ),
    );
  }

  Widget _buildStep1Tone() {
     return _buildGlassmorphism(
         child: Column(
             children: ['Formal', 'Friendly', 'Concise', 'Detailed', 'Custom'].map((tone) => RadioListTile<String>(
                 title: Text(tone, style: const TextStyle(fontFamily: 'Inter')),
                 value: tone,
                 groupValue: _selectedTone,
                 onChanged: (v) => setState(() => _selectedTone = v!),
             )).toList()
         )
     );
  }

  Widget _buildStep2Domain() {
      return _buildGlassmorphism(
          child: Wrap(
             spacing: 8.0,
             runSpacing: 4.0,
             children: _domainFocus.keys.map((k) => FilterChip(
                 label: Text(k, style: const TextStyle(fontFamily: 'Inter')),
                 selected: _domainFocus[k]!,
                 onSelected: (bool value) => setState(() => _domainFocus[k] = value),
             )).toList()
          )
      );
  }

  Widget _buildStep3Examples() {
      return _buildGlassmorphism(
          child: Column(
             children: List.generate(3, (i) => Column(
                 crossAxisAlignment: CrossAxisAlignment.start,
                 children: [
                     Text('Example ${i+1}', style: const TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                     TextField(controller: _qControllers[i], decoration: const InputDecoration(labelText: 'Question')),
                     TextField(controller: _aControllers[i], decoration: const InputDecoration(labelText: 'Answer')),
                     const SizedBox(height: 16),
                 ]
             ))
          )
      );
  }

  Widget _buildStep4Preview(BuildContext context) {
      final isMobile = MediaQuery.of(context).size.width < 600;

      Widget promptWidget = _buildGlassmorphism(
          child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                  const Text('Generated Prompt', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                  const SizedBox(height: 8),
                  Container(
                      padding: const EdgeInsets.all(8),
                      decoration: BoxDecoration(
                          color: Colors.black.withOpacity(0.1),
                          borderRadius: BorderRadius.circular(8)
                      ),
                      child: Text(_generatedPrompt, style: const TextStyle(fontFamily: 'Inter', fontSize: 14)),
                  ),
                  const SizedBox(height: 16),
                  ElevatedButton(
                      onPressed: _isLoading ? null : _savePrompt,
                      child: _isLoading ? const CircularProgressIndicator() : const Text('Save Prompt', style: TextStyle(fontFamily: 'Inter'))
                  )
              ]
          )
      );

      Widget sandboxWidget = _buildGlassmorphism(
          child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                  const Text('Chat Sandbox', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                  const SizedBox(height: 8),
                  Container(
                      height: 200,
                      padding: const EdgeInsets.all(8),
                      decoration: BoxDecoration(
                          color: Colors.black.withOpacity(0.1),
                          borderRadius: BorderRadius.circular(8)
                      ),
                      child: ListView.builder(
                          itemCount: _messages.length,
                          itemBuilder: (context, index) {
                              final msg = _messages[index];
                              final isUser = msg['role'] == 'user';
                              return Align(
                                  alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
                                  child: Container(
                                      margin: const EdgeInsets.symmetric(vertical: 4),
                                      padding: const EdgeInsets.all(8),
                                      decoration: BoxDecoration(
                                          color: isUser ? Colors.blue.withOpacity(0.2) : Colors.green.withOpacity(0.2),
                                          borderRadius: BorderRadius.circular(8)
                                      ),
                                      child: Text(msg['content'] ?? '', style: const TextStyle(fontFamily: 'Inter', fontSize: 14)),
                                  ),
                              );
                          }
                      ),
                  ),
                  const SizedBox(height: 8),
                  Row(
                      children: [
                          Expanded(
                              child: TextField(
                                  controller: _chatController,
                                  decoration: const InputDecoration(hintText: 'Test your agent...', border: OutlineInputBorder()),
                                  onSubmitted: (_) => _sendMessage(),
                              )
                          ),
                          IconButton(
                              icon: _isChatLoading ? const CircularProgressIndicator() : const Icon(Icons.send),
                              onPressed: _isChatLoading ? null : _sendMessage,
                          )
                      ]
                  )
              ]
          )
      );

      if (isMobile) {
          return Column(
              children: [
                  promptWidget,
                  const SizedBox(height: 16),
                  sandboxWidget,
              ]
          );
      } else {
          return Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                  Expanded(flex: 1, child: promptWidget),
                  const SizedBox(width: 16),
                  Expanded(flex: 1, child: sandboxWidget),
              ]
          );
      }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        body: Stepper(
             currentStep: _step,
             onStepContinue: () {
                 if (_step < 3) setState(() => _step++);
             },
             onStepCancel: () {
                 if (_step > 0) setState(() => _step--);
             },
             steps: [
                 Step(title: const Text('Personality & Tone', style: TextStyle(fontFamily: 'Outfit')), content: _buildStep1Tone(), isActive: _step >= 0),
                 Step(title: const Text('Domain Focus', style: TextStyle(fontFamily: 'Outfit')), content: _buildStep2Domain(), isActive: _step >= 1),
                 Step(title: const Text('Example Interactions', style: TextStyle(fontFamily: 'Outfit')), content: _buildStep3Examples(), isActive: _step >= 2),
                 Step(title: const Text('Live Preview & Save', style: TextStyle(fontFamily: 'Outfit')), content: _buildStep4Preview(context), isActive: _step >= 3),
             ]
        )
    );
  }
}
