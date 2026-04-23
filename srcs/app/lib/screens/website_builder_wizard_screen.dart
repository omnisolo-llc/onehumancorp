import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/services/auth_service.dart';

class WebsiteBuilderWizardScreen extends ConsumerStatefulWidget {
  const WebsiteBuilderWizardScreen({super.key});

  @override
  ConsumerState<WebsiteBuilderWizardScreen> createState() => _WebsiteBuilderWizardScreenState();
}

class _WebsiteBuilderWizardScreenState extends ConsumerState<WebsiteBuilderWizardScreen> {
  int _step = 0;
  String? _selectedTemplate;
  String? _selectedPalette;
  final TextEditingController _productNameController = TextEditingController();
  final TextEditingController _productPriceController = TextEditingController();
  final TextEditingController _productDescController = TextEditingController();
  String? _domainOption;
  bool _isPublishing = false;
  bool _isGeneratingLogo = false;
  bool _isGeneratingDesc = false;
  String? _logoUrl;

  void _nextStep() {
    setState(() => _step++);
  }

  void _prevStep() {
    if (_step > 0) setState(() => _step--);
  }

  @override
  void dispose() {
    _productNameController.dispose();
    _productPriceController.dispose();
    _productDescController.dispose();
    super.dispose();
  }

  Future<void> _generateLogo() async {
    setState(() => _isGeneratingLogo = true);
    try {
      final user = ref.read(authStateProvider).valueOrNull;
      final baseUrl = ref.read(backendUrlProvider);
      final resp = await http.post(
        Uri.parse('$baseUrl/api/wizard/generate_logo'),
        headers: {
          'Authorization': 'Bearer ${user?.token ?? ''}',
          'Content-Type': 'application/json',
        },
      );
      if (resp.statusCode == 200) {
        final data = jsonDecode(resp.body);
        setState(() {
          _logoUrl = data['logo_url'];
        });
      }
    } finally {
      if (mounted) setState(() => _isGeneratingLogo = false);
    }
  }

  Future<void> _generateDescription() async {
    if (_productNameController.text.isEmpty) return;
    setState(() => _isGeneratingDesc = true);
    try {
      final user = ref.read(authStateProvider).valueOrNull;
      final baseUrl = ref.read(backendUrlProvider);
      final resp = await http.post(
        Uri.parse('$baseUrl/api/wizard/generate_desc'),
        headers: {
          'Authorization': 'Bearer ${user?.token ?? ''}',
          'Content-Type': 'application/json',
        },
        body: jsonEncode({'product_name': _productNameController.text}),
      );
      if (resp.statusCode == 200) {
        final data = jsonDecode(resp.body);
        setState(() {
          _productDescController.text = data['description'];
        });
      }
    } finally {
      if (mounted) setState(() => _isGeneratingDesc = false);
    }
  }

  Future<void> _publish() async {
    setState(() => _isPublishing = true);
    try {
      final user = ref.read(authStateProvider).valueOrNull;
      final baseUrl = ref.read(backendUrlProvider);
      final resp = await http.post(
        Uri.parse('$baseUrl/api/wizard/website_publish'),
        headers: {
          'Authorization': 'Bearer ${user?.token ?? ''}',
          'Content-Type': 'application/json',
        },
        body: jsonEncode({
          'template': _selectedTemplate,
          'palette': _selectedPalette,
          'product_name': _productNameController.text,
          'product_price': _productPriceController.text,
          'product_desc': _productDescController.text,
          'domain_option': _domainOption,
        }),
      );
      if (resp.statusCode == 200 && mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Site published! Link copied to clipboard.')),
        );
        context.go('/dashboard');
      } else if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Failed to publish: ${resp.body}')),
        );
      }
    } finally {
      if (mounted) setState(() => _isPublishing = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Website Builder Onboarding'),
        leading: _step > 0 ? IconButton(icon: const Icon(Icons.arrow_back), onPressed: _prevStep) : null,
      ),
      body: Center(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(24),
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600),
            child: GlassCard(
              child: Padding(
                padding: const EdgeInsets.all(32),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Text(
                      'Build My Website',
                      style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(height: 24),
                    _buildCurrentStep(),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildCurrentStep() {
    switch (_step) {
      case 0: return _buildTemplateGallery();
      case 1: return _buildBrandColorsAndLogo();
      case 2: return _buildAddProduct();
      case 3: return _buildConnectDomain();
      case 4: return _buildGoLive();
      default: return const SizedBox.shrink();
    }
  }

  Widget _buildTemplateGallery() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text('Step 1: Choose a Template', style: TextStyle(fontFamily: 'Inter', fontSize: 18, fontWeight: FontWeight.bold)),
        const SizedBox(height: 16),
        Wrap(
          spacing: 16,
          runSpacing: 16,
          children: ['Minimalist', 'Modern', 'Classic'].map((template) {
            final isSelected = _selectedTemplate == template;
            return GestureDetector(
              onTap: () => setState(() => _selectedTemplate = template),
              child: Container(
                width: 150,
                height: 150,
                decoration: BoxDecoration(
                  border: Border.all(color: isSelected ? Colors.green : Colors.grey),
                  borderRadius: BorderRadius.circular(8),
                  color: isSelected ? Colors.green.withOpacity(0.1) : Colors.transparent,
                ),
                child: Center(child: Text(template, style: const TextStyle(fontFamily: 'Inter'))),
              ),
            );
          }).toList(),
        ),
        const SizedBox(height: 24),
        FilledButton(
          onPressed: _selectedTemplate != null ? _nextStep : null,
          style: FilledButton.styleFrom(
            backgroundColor: _selectedTemplate != null ? Colors.green : null,
          ),
          child: Text(_selectedTemplate != null ? 'Use this template →' : 'Select a template'),
        ),
      ],
    );
  }

  Widget _buildBrandColorsAndLogo() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text('Step 2: Brand Colors & Logo', style: TextStyle(fontFamily: 'Inter', fontSize: 18, fontWeight: FontWeight.bold)),
        const SizedBox(height: 16),
        Wrap(
          spacing: 16,
          children: ['Ocean Blue', 'Forest Green', 'Sunset Orange'].map((palette) {
            final isSelected = _selectedPalette == palette;
            return ChoiceChip(
              label: Text(palette),
              selected: isSelected,
              onSelected: (val) {
                if (val) setState(() => _selectedPalette = palette);
              },
            );
          }).toList(),
        ),
        const SizedBox(height: 16),
        if (_logoUrl != null)
          Padding(
            padding: const EdgeInsets.only(bottom: 16.0),
            child: Row(
              children: [
                const Icon(Icons.image, size: 40, color: Colors.blue),
                const SizedBox(width: 8),
                Text('Logo generated', style: TextStyle(color: Colors.green.shade700)),
              ],
            ),
          ),
        _isGeneratingLogo
            ? const Center(child: CircularProgressIndicator())
            : OutlinedButton.icon(
                onPressed: _generateLogo,
                icon: const Icon(Icons.auto_awesome),
                label: const Text('Generate a logo for me'),
              ),
        const SizedBox(height: 24),
        FilledButton(
          onPressed: _selectedPalette != null ? _nextStep : null,
          child: const Text('Next Step →'),
        ),
      ],
    );
  }

  Widget _buildAddProduct() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text('Step 3: Add your first product or service', style: TextStyle(fontFamily: 'Inter', fontSize: 18, fontWeight: FontWeight.bold)),
        const SizedBox(height: 16),
        TextField(controller: _productNameController, decoration: const InputDecoration(labelText: 'Name', border: OutlineInputBorder())),
        const SizedBox(height: 16),
        TextField(controller: _productPriceController, decoration: const InputDecoration(labelText: 'Price', border: OutlineInputBorder()), keyboardType: TextInputType.number),
        const SizedBox(height: 16),
        Row(
          children: [
            Expanded(child: TextField(controller: _productDescController, decoration: const InputDecoration(labelText: 'Short Description', border: OutlineInputBorder()), maxLines: 3)),
            const SizedBox(width: 8),
            _isGeneratingDesc
                ? const CircularProgressIndicator()
                : IconButton(
                    icon: const Icon(Icons.auto_awesome, color: Colors.purple),
                    onPressed: _generateDescription,
                    tooltip: 'AI generate description',
                  ),
          ],
        ),
        const SizedBox(height: 24),
        FilledButton(
          onPressed: _nextStep,
          child: const Text('Next Step →'),
        ),
      ],
    );
  }

  Widget _buildConnectDomain() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text('Step 4: Connect a domain', style: TextStyle(fontFamily: 'Inter', fontSize: 18, fontWeight: FontWeight.bold)),
        const SizedBox(height: 16),
        RadioListTile<String>(
          title: const Text('Use a free OHC subdomain (mybusiness.ohc.app)'),
          value: 'free',
          groupValue: _domainOption,
          onChanged: (val) {
            setState(() => _domainOption = val);
            if (val == 'free') {
              _nextStep(); // One-tap completion for free subdomain
            }
          },
        ),
        RadioListTile<String>(
          title: const Text('Use my own domain'),
          value: 'own',
          groupValue: _domainOption,
          onChanged: (val) => setState(() => _domainOption = val),
        ),
        RadioListTile<String>(
          title: const Text('Buy a domain'),
          value: 'buy',
          groupValue: _domainOption,
          onChanged: (val) => setState(() => _domainOption = val),
        ),
        const SizedBox(height: 24),
        FilledButton(
          onPressed: _domainOption != null ? _nextStep : null,
          child: const Text('Next Step →'),
        ),
      ],
    );
  }

  Widget _buildGoLive() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text('Step 5: Go Live', style: TextStyle(fontFamily: 'Inter', fontSize: 18, fontWeight: FontWeight.bold)),
        const SizedBox(height: 16),
        const Text('Your site is ready to be published!', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
        const SizedBox(height: 24),
        _isPublishing
            ? const Center(child: CircularProgressIndicator())
            : FilledButton(
                onPressed: _publish,
                child: const Text('Publish'),
              ),
      ],
    );
  }
}
