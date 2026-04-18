import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/models/ai_provider.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/widgets/glass_card.dart';

/// Common model-provider wizard.  Walks the user through a 5-step flow to
/// configure any AI provider (builtin agent, openclaw, hermes, etc.).
///
/// Step 1 – Select provider type
/// Step 2 – Enter / confirm base URL
/// Step 3 – Enter API key
/// Step 4 – Choose primary model
/// Step 5 – Review and save
class ModelProviderWizardScreen extends ConsumerStatefulWidget {
  /// The agent type this provider will be used by.
  /// Examples: 'builtin', 'openclaw', 'ironclaw'.
  final String agentType;

  const ModelProviderWizardScreen({
    super.key,
    this.agentType = '',
  });

  @override
  ConsumerState<ModelProviderWizardScreen> createState() =>
      _ModelProviderWizardScreenState();
}

class _ModelProviderWizardScreenState
    extends ConsumerState<ModelProviderWizardScreen> {
  int _step = 0;
  bool _loading = false;
  String? _error;
  String? _instruction;
  bool _complete = false;
  bool _obscureKey = true;

  // Provider config being built step-by-step.
  ProviderType _providerType = ProviderType.unspecified;
  final _nameCtrl = TextEditingController();
  final _urlCtrl = TextEditingController();
  final _keyCtrl = TextEditingController();
  final _modelCtrl = TextEditingController();

  static const List<ProviderType> _selectableProviders = [
    ProviderType.openai,
    ProviderType.anthropic,
    ProviderType.google,
    ProviderType.groq,
    ProviderType.ollama,
    ProviderType.openrouter,
    ProviderType.azure,
    ProviderType.amazonBedrock,
    ProviderType.minimax,
    ProviderType.custom,
  ];

  static const Map<ProviderType, String> _defaultBaseUrls = {
    ProviderType.openai: 'https://api.openai.com/v1',
    ProviderType.anthropic: 'https://api.anthropic.com/v1',
    ProviderType.google: 'https://generativelanguage.googleapis.com/v1beta',
    ProviderType.groq: 'https://api.groq.com/openai/v1',
    ProviderType.ollama: 'http://localhost:11434/v1',
    ProviderType.openrouter: 'https://openrouter.ai/api/v1',
    ProviderType.minimax: 'https://api.minimax.io/v1',
  };

  static const Map<ProviderType, List<String>> _defaultModels = {
    ProviderType.openai: ['gpt-4o-mini', 'gpt-4o', 'gpt-4-turbo'],
    ProviderType.anthropic: ['claude-sonnet-4-5', 'claude-haiku-4-5'],
    ProviderType.groq: ['llama-3.1-70b-versatile', 'mixtral-8x7b-32768'],
    ProviderType.ollama: ['llama3', 'mistral', 'phi3'],
    ProviderType.minimax: ['MiniMax-Text-01', 'abab6.5s-chat'],
    ProviderType.openrouter: ['openai/gpt-4o', 'anthropic/claude-3-5-sonnet'],
  };

  @override
  void dispose() {
    _nameCtrl.dispose();
    _urlCtrl.dispose();
    _keyCtrl.dispose();
    _modelCtrl.dispose();
    super.dispose();
  }

  void _applyProviderDefaults(ProviderType type) {
    _providerType = type;
    if (_nameCtrl.text.isEmpty || _nameCtrl.text == _providerType.displayName) {
      _nameCtrl.text = type.displayName;
    }
    final url = _defaultBaseUrls[type] ?? '';
    if (url.isNotEmpty) _urlCtrl.text = url;
    final models = _defaultModels[type];
    if (models != null && models.isNotEmpty && _modelCtrl.text.isEmpty) {
      _modelCtrl.text = models.first;
    }
  }

  Future<void> _advance() async {
    setState(() {
      _loading = true;
      _error = null;
    });

    final api = ref.read(apiServiceProvider);
    if (api == null) {
      setState(() {
        _loading = false;
        _error = 'No API connection available.';
      });
      return;
    }

    try {
      final result = await api.modelProviderWizardStep(
        step: _step,
        provider: {
          'provider_type': _providerType.value,
          'name': _nameCtrl.text.trim(),
          'base_url': _urlCtrl.text.trim(),
          'api_key': _keyCtrl.text.trim(),
          'model': _modelCtrl.text.trim(),
          'models': _modelCtrl.text.trim().isNotEmpty ? [_modelCtrl.text.trim()] : <String>[],
        },
        agentType: widget.agentType,
      );

      final errors = (result['validation_errors'] as List<dynamic>?)?.cast<String>() ?? [];
      if (errors.isNotEmpty) {
        setState(() {
          _loading = false;
          _error = errors.join(', ');
        });
        return;
      }

      final nextStep = (result['step'] as int?) ?? _step + 1;
      final instruction = result['instruction'] as String? ?? '';
      final complete = result['complete'] as bool? ?? false;

      setState(() {
        _step = nextStep;
        _instruction = instruction;
        _complete = complete;
        _loading = false;
      });

      if (complete) {
        await _saveProvider(api);
      }
    } catch (e) {
      setState(() {
        _loading = false;
        _error = 'Error: $e';
      });
    }
  }

  Future<void> _saveProvider(ApiService api) async {
    try {
      await api.addAiProvider(
        name: _nameCtrl.text.trim(),
        baseUrl: _urlCtrl.text.trim(),
        apiKey: _keyCtrl.text.trim(),
        models: _modelCtrl.text.trim().isNotEmpty ? [_modelCtrl.text.trim()] : [],
      );
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('${_nameCtrl.text} provider saved!')),
        );
        Navigator.of(context).pop(true);
      }
    } catch (e) {
      setState(() => _error = 'Failed to save provider: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Scaffold(
      appBar: AppBar(
        title: Text(
          widget.agentType.isNotEmpty
              ? 'Configure Model – ${widget.agentType}'
              : 'Configure AI Model Provider',
        ),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 560),
          child: GlassCard(
            child: Padding(
              padding: const EdgeInsets.all(24),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  // Step indicator.
                  Row(
                    children: List.generate(5, (i) {
                      final active = i < _step;
                      final current = i == _step - 1;
                      return Expanded(
                        child: Container(
                          margin: const EdgeInsets.symmetric(horizontal: 2),
                          height: 4,
                          decoration: BoxDecoration(
                            color: active
                                ? colors.primary
                                : current
                                    ? colors.primary.withOpacity(0.5)
                                    : colors.outline.withOpacity(0.3),
                            borderRadius: BorderRadius.circular(2),
                          ),
                        ),
                      );
                    }),
                  ),
                  const SizedBox(height: 20),

                  if (_error != null) ...[
                    Text(_error!, style: TextStyle(color: colors.error)),
                    const SizedBox(height: 12),
                  ],

                  if (_instruction != null) ...[
                    Text(_instruction!, style: const TextStyle(color: Colors.white70, fontSize: 14)),
                    const SizedBox(height: 16),
                  ],

                  // Step content.
                  _buildStepContent(colors),

                  const SizedBox(height: 24),

                  // Navigation buttons.
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      if (_step > 1)
                        TextButton(
                          onPressed: _loading ? null : () => setState(() => _step--),
                          child: const Text('Back'),
                        )
                      else
                        const SizedBox(),
                      ElevatedButton(
                        onPressed: _loading ? null : _advance,
                        child: _loading
                            ? const SizedBox(
                                width: 18,
                                height: 18,
                                child: CircularProgressIndicator(strokeWidth: 2),
                              )
                            : Text(_step >= 5 ? 'Save' : 'Next'),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildStepContent(ColorScheme colors) {
    switch (_step) {
      case 0:
      case 1:
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Select Provider',
              style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold, color: colors.onSurface),
            ),
            const SizedBox(height: 12),
            DropdownButtonFormField<ProviderType>(
              value: _providerType == ProviderType.unspecified ? null : _providerType,
              decoration: const InputDecoration(
                labelText: 'Provider',
                border: OutlineInputBorder(),
              ),
              items: _selectableProviders.map((pt) {
                return DropdownMenuItem(value: pt, child: Text(pt.displayName));
              }).toList(),
              onChanged: (pt) {
                if (pt != null) setState(() => _applyProviderDefaults(pt));
              },
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _nameCtrl,
              decoration: const InputDecoration(
                labelText: 'Display Name',
                border: OutlineInputBorder(),
              ),
            ),
          ],
        );

      case 2:
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Base URL',
              style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold, color: colors.onSurface),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _urlCtrl,
              decoration: const InputDecoration(
                labelText: 'Base URL',
                hintText: 'https://api.example.com/v1',
                border: OutlineInputBorder(),
              ),
            ),
          ],
        );

      case 3:
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'API Key',
              style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold, color: colors.onSurface),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _keyCtrl,
              obscureText: _obscureKey,
              decoration: InputDecoration(
                labelText: 'API Key (optional for local providers)',
                border: const OutlineInputBorder(),
                prefixIcon: const Icon(Icons.key),
                suffixIcon: IconButton(
                  icon: Icon(_obscureKey ? Icons.visibility : Icons.visibility_off),
                  onPressed: () => setState(() => _obscureKey = !_obscureKey),
                ),
              ),
            ),
          ],
        );

      case 4:
        final suggestions = _defaultModels[_providerType] ?? [];
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Select Model',
              style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold, color: colors.onSurface),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _modelCtrl,
              decoration: const InputDecoration(
                labelText: 'Model Name',
                border: OutlineInputBorder(),
              ),
            ),
            if (suggestions.isNotEmpty) ...[
              const SizedBox(height: 8),
              Wrap(
                spacing: 8,
                children: suggestions.map((m) => ActionChip(
                  label: Text(m, style: const TextStyle(fontSize: 12)),
                  onPressed: () => setState(() => _modelCtrl.text = m),
                )).toList(),
              ),
            ],
          ],
        );

      default:
        // Step 5: Review.
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Review & Save',
              style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold, color: colors.onSurface),
            ),
            const SizedBox(height: 12),
            _reviewRow('Provider', _nameCtrl.text),
            _reviewRow('Type', _providerType.displayName),
            _reviewRow('Base URL', _urlCtrl.text),
            _reviewRow('API Key', _keyCtrl.text.isEmpty ? '(none)' : '••••••••'),
            _reviewRow('Model', _modelCtrl.text),
          ],
        );
    }
  }

  Widget _reviewRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        children: [
          SizedBox(
            width: 90,
            child: Text(
              label,
              style: const TextStyle(fontFamily: 'Inter', color: Colors.white54, fontSize: 13),
            ),
          ),
          Expanded(
            child: Text(
              value,
              style: const TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 13),
            ),
          ),
        ],
      ),
    );
  }
}
