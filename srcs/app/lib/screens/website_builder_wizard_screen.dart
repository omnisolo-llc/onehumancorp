import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../widgets/glass_card.dart';

class WebsiteBuilderState {
  final int step;
  final String template;
  final String colorPalette;
  final String logoPath;
  final String firstProductName;
  final String firstProductPrice;
  final String firstProductDescription;
  final String domainChoice;
  final bool isLoading;
  final String? errorMessage;

  const WebsiteBuilderState({
    this.step = 0,
    this.template = '',
    this.colorPalette = '',
    this.logoPath = '',
    this.firstProductName = '',
    this.firstProductPrice = '',
    this.firstProductDescription = '',
    this.domainChoice = '',
    this.isLoading = false,
    this.errorMessage,
  });

  WebsiteBuilderState copyWith({
    int? step,
    String? template,
    String? colorPalette,
    String? logoPath,
    String? firstProductName,
    String? firstProductPrice,
    String? firstProductDescription,
    String? domainChoice,
    bool? isLoading,
    String? errorMessage,
  }) {
    return WebsiteBuilderState(
      step: step ?? this.step,
      template: template ?? this.template,
      colorPalette: colorPalette ?? this.colorPalette,
      logoPath: logoPath ?? this.logoPath,
      firstProductName: firstProductName ?? this.firstProductName,
      firstProductPrice: firstProductPrice ?? this.firstProductPrice,
      firstProductDescription: firstProductDescription ?? this.firstProductDescription,
      domainChoice: domainChoice ?? this.domainChoice,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage,
    );
  }
}

class WebsiteBuilderNotifier extends Notifier<WebsiteBuilderState> {
  @override
  WebsiteBuilderState build() => const WebsiteBuilderState();

  void nextStep() {
    if (state.step < 4) {
      state = state.copyWith(step: state.step + 1);
    }
  }

  void prevStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
    }
  }

  void selectTemplate(String template) {
    state = state.copyWith(template: template);
  }

  void selectColorPalette(String palette) {
    state = state.copyWith(colorPalette: palette);
  }

  void updateProductName(String name) {
    state = state.copyWith(firstProductName: name);
  }

  void updateProductPrice(String price) {
    state = state.copyWith(firstProductPrice: price);
  }

  void updateProductDescription(String desc) {
    state = state.copyWith(firstProductDescription: desc);
  }

  void selectDomain(String domain) {
    state = state.copyWith(domainChoice: domain);
  }

  Future<void> publish(BuildContext context) async {
    state = state.copyWith(isLoading: true, errorMessage: null);

    // Simulate API call
    await Future.delayed(const Duration(seconds: 2));

    state = state.copyWith(isLoading: false);

    if (context.mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Website published successfully! Link copied to clipboard.')),
      );
      context.go('/dashboard');
    }
  }
}

final websiteBuilderProvider = NotifierProvider<WebsiteBuilderNotifier, WebsiteBuilderState>(() {
  return WebsiteBuilderNotifier();
});

class WebsiteBuilderWizardScreen extends ConsumerWidget {
  const WebsiteBuilderWizardScreen({super.key});

  Widget _buildStepZero(WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Choose a Template', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white, fontSize: 18)),
        const SizedBox(height: 16),
        SizedBox(
          height: 300,
          child: GridView.count(
            crossAxisCount: 2,
            crossAxisSpacing: 16,
            mainAxisSpacing: 16,
            childAspectRatio: 0.8,
            children: ['Modern Minimal', 'Vibrant Store', 'Elegant Service', 'Bold Portfolio'].map((t) {
              final isSelected = state.template == t;
              return GestureDetector(
                onTap: () => notifier.selectTemplate(t),
                child: Container(
                  decoration: BoxDecoration(
                    color: isSelected ? Colors.blueAccent.withValues(alpha: 0.2) : Colors.white.withValues(alpha: 0.05),
                    borderRadius: BorderRadius.circular(12),
                    border: Border.all(color: isSelected ? Colors.blueAccent : Colors.transparent, width: 2),
                  ),
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(Icons.web, size: 48, color: isSelected ? Colors.blueAccent : Colors.white70),
                      const SizedBox(height: 16),
                      Text(t, style: TextStyle(color: isSelected ? Colors.white : Colors.white70, fontWeight: isSelected ? FontWeight.bold : FontWeight.normal)),
                    ],
                  ),
                ),
              );
            }).toList(),
          ),
        ),
      ],
    );
  }

  Widget _buildStepOne(WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Brand Colors & Logo', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white, fontSize: 18)),
        const SizedBox(height: 16),
        const Text('Suggested Palettes', style: TextStyle(color: Colors.white70)),
        const SizedBox(height: 8),
        Wrap(
          spacing: 16,
          children: ['Ocean Blue', 'Forest Green', 'Sunset Orange'].map((p) {
            final isSelected = state.colorPalette == p;
            return ChoiceChip(
              label: Text(p),
              selected: isSelected,
              onSelected: (selected) {
                if (selected) notifier.selectColorPalette(p);
              },
              selectedColor: Colors.blueAccent,
              backgroundColor: const Color(0xFF1A1A33),
              labelStyle: TextStyle(color: isSelected ? Colors.white : Colors.white70),
            );
          }).toList(),
        ),
        const SizedBox(height: 24),
        OutlinedButton.icon(
          onPressed: () {},
          icon: const Icon(Icons.upload),
          label: const Text('Upload Logo'),
        ),
        const SizedBox(height: 8),
        TextButton(
          onPressed: () {},
          child: const Text('Generate a logo with AI'),
        ),
      ],
    );
  }

  Widget _buildStepTwo(WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Add your first item', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white, fontSize: 18)),
        const SizedBox(height: 16),
        TextField(
          decoration: const InputDecoration(labelText: 'Product/Service Name', labelStyle: TextStyle(color: Colors.white70)),
          onChanged: notifier.updateProductName,
          style: const TextStyle(color: Colors.white),
        ),
        const SizedBox(height: 16),
        TextField(
          decoration: const InputDecoration(labelText: 'Price', labelStyle: TextStyle(color: Colors.white70)),
          keyboardType: TextInputType.number,
          onChanged: notifier.updateProductPrice,
          style: const TextStyle(color: Colors.white),
        ),
        const SizedBox(height: 16),
        TextField(
          decoration: const InputDecoration(labelText: 'Short Description', labelStyle: TextStyle(color: Colors.white70)),
          maxLines: 3,
          onChanged: notifier.updateProductDescription,
          style: const TextStyle(color: Colors.white),
        ),
        const SizedBox(height: 8),
        TextButton.icon(
          onPressed: () {
            notifier.updateProductDescription("A premium ${state.firstProductName} perfect for your needs.");
          },
          icon: const Icon(Icons.auto_awesome),
          label: const Text('AI Auto-write'),
        ),
      ],
    );
  }

  Widget _buildStepThree(WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Connect a domain', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white, fontSize: 18)),
        const SizedBox(height: 16),
        ...['Use free OHC subdomain (mybusiness.ohc.app)', 'Use my own domain', 'Buy a domain'].map((d) {
          return RadioListTile<String>(
            title: Text(d, style: const TextStyle(color: Colors.white)),
            value: d,
            groupValue: state.domainChoice,
            activeColor: Colors.blueAccent,
            onChanged: (String? value) {
              if (value != null) notifier.selectDomain(value);
            },
          );
        }),
      ],
    );
  }

  Widget _buildStepFour(WebsiteBuilderState state) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text('Ready to Go Live!', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white, fontSize: 24)),
        const SizedBox(height: 24),
        GlassCard(
          color: Colors.white.withValues(alpha: 0.05),
          child: Padding(
            padding: const EdgeInsets.all(24.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('Template: ${state.template.isEmpty ? "Not selected" : state.template}', style: const TextStyle(color: Colors.white)),
                const SizedBox(height: 8),
                Text('Palette: ${state.colorPalette.isEmpty ? "Default" : state.colorPalette}', style: const TextStyle(color: Colors.white70)),
                const SizedBox(height: 8),
                Text('First Item: ${state.firstProductName} (\$${state.firstProductPrice})', style: const TextStyle(color: Colors.white70)),
                const SizedBox(height: 8),
                Text('Domain: ${state.domainChoice.isEmpty ? "Not selected" : state.domainChoice}', style: const TextStyle(color: Colors.white70)),
              ],
            ),
          ),
        ),
      ],
    );
  }

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(websiteBuilderProvider);
    final notifier = ref.read(websiteBuilderProvider.notifier);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Website Builder'),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      extendBodyBehindAppBar: true,
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF0D0D1A), Color(0xFF1A1A33)],
          ),
        ),
        child: Center(
          child: SingleChildScrollView(
            padding: const EdgeInsets.all(24),
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 800),
              child: GlassCard(
                child: Padding(
                  padding: const EdgeInsets.all(32.0),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Row(
                        children: [
                          for (int i = 0; i < 5; i++)
                            Expanded(
                              child: Container(
                                height: 4,
                                margin: const EdgeInsets.symmetric(horizontal: 4),
                                decoration: BoxDecoration(
                                  color: i <= state.step ? Colors.blueAccent : Colors.white.withValues(alpha: 0.1),
                                  borderRadius: BorderRadius.circular(2),
                                ),
                              ),
                            ),
                        ],
                      ),
                      const SizedBox(height: 32),
                      AnimatedSwitcher(
                        duration: const Duration(milliseconds: 300),
                        child: Container(
                          key: ValueKey<int>(state.step),
                          child: () {
                            switch (state.step) {
                              case 0: return _buildStepZero(state, notifier);
                              case 1: return _buildStepOne(state, notifier);
                              case 2: return _buildStepTwo(state, notifier);
                              case 3: return _buildStepThree(state, notifier);
                              case 4: return _buildStepFour(state);
                              default: return const SizedBox();
                            }
                          }(),
                        ),
                      ),
                      const SizedBox(height: 32),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          if (state.step > 0)
                            TextButton(
                              onPressed: state.isLoading ? null : notifier.prevStep,
                              child: const Text('Back', style: TextStyle(fontFamily: 'Inter')),
                            )
                          else
                            const SizedBox(),
                          ElevatedButton(
                            onPressed: state.isLoading ? null : () {
                              if (state.step < 4) {
                                notifier.nextStep();
                              } else {
                                notifier.publish(context);
                              }
                            },
                            style: ElevatedButton.styleFrom(
                              padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 16),
                            ),
                            child: state.isLoading
                                ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                                : Text(state.step == 4 ? 'Publish →' : 'Continue', style: const TextStyle(fontFamily: 'Inter', fontSize: 16)),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
