import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/api_service.dart';

class WizardState {
  final int currentStep;
  final String? businessIdea;
  final String? generatedUrl;

  WizardState({
    this.currentStep = 0,
    this.businessIdea,
    this.generatedUrl,
  });

  WizardState copyWith({
    int? currentStep,
    String? businessIdea,
    String? generatedUrl,
  }) {
    return WizardState(
      currentStep: currentStep ?? this.currentStep,
      businessIdea: businessIdea ?? this.businessIdea,
      generatedUrl: generatedUrl ?? this.generatedUrl,
    );
  }
}

class WizardNotifier extends Notifier<WizardState> {
  final ApiService _apiService = ApiService();

  @override
  WizardState build() {
    return WizardState();
  }

  void nextStep() {
    if (state.currentStep < 4) {
      state = state.copyWith(currentStep: state.currentStep + 1);
    }
  }

  void prevStep() {
    if (state.currentStep > 0) {
      state = state.copyWith(currentStep: state.currentStep - 1);
    }
  }

  void setBusinessIdea(String idea) {
    state = state.copyWith(businessIdea: idea);
  }

  Future<void> submitWizard() async {
    final data = {
      'businessIdea': state.businessIdea,
    };
    try {
      await _apiService.submitBusinessData(data);
    } catch (e) {
      // Ignored for now
    }
    state = state.copyWith(generatedUrl: 'https://mybusiness.ohc.app');
    nextStep();
  }
}

final wizardProvider = NotifierProvider<WizardNotifier, WizardState>(WizardNotifier.new);
