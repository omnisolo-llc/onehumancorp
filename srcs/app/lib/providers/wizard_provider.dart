import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/api_service.dart';

class WizardState {
  final int currentStep;
  final String? companyName;
  final String? industry;
  final String? size;
  final List<String> goals;
  final String? deploymentPreference;
  final String? adminName;
  final String? adminEmail;
  final String? adminPassword;
  final String? websiteTemplate;
  final String? productName;
  final String? productPrice;
  final String? domain;
  final int checklistProgress;
  final String currency;
  final bool isLive;

  WizardState({
    this.currentStep = 0,
    this.companyName,
    this.industry,
    this.size,
    this.goals = const [],
    this.deploymentPreference,
    this.adminName,
    this.adminEmail,
    this.adminPassword,
    this.websiteTemplate,
    this.productName,
    this.productPrice,
    this.domain,
    this.checklistProgress = 0,
    this.currency = 'USD',
    this.isLive = false,
  });

  WizardState copyWith({
    int? currentStep,
    String? companyName,
    String? industry,
    String? size,
    List<String>? goals,
    String? deploymentPreference,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
    String? websiteTemplate,
    String? productName,
    String? productPrice,
    String? domain,
    int? checklistProgress,
    String? currency,
    bool? isLive,
  }) {
    return WizardState(
      currentStep: currentStep ?? this.currentStep,
      companyName: companyName ?? this.companyName,
      industry: industry ?? this.industry,
      size: size ?? this.size,
      goals: goals ?? this.goals,
      deploymentPreference: deploymentPreference ?? this.deploymentPreference,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
      websiteTemplate: websiteTemplate ?? this.websiteTemplate,
      productName: productName ?? this.productName,
      productPrice: productPrice ?? this.productPrice,
      domain: domain ?? this.domain,
      checklistProgress: checklistProgress ?? this.checklistProgress,
      currency: currency ?? this.currency,
      isLive: isLive ?? this.isLive,
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
    if (state.currentStep < 10) {
      state = state.copyWith(currentStep: state.currentStep + 1);
    }
  }

  void prevStep() {
    if (state.currentStep > 0) {
      state = state.copyWith(currentStep: state.currentStep - 1);
    }
  }

  void updateBusinessProfile({String? companyName, String? industry, String? size}) {
    state = state.copyWith(
      companyName: companyName ?? state.companyName,
      industry: industry ?? state.industry,
      size: size ?? state.size,
    );
  }

  void toggleGoal(String goal) {
    final currentGoals = List<String>.from(state.goals);
    if (currentGoals.contains(goal)) {
      currentGoals.remove(goal);
    } else {
      currentGoals.add(goal);
    }
    state = state.copyWith(goals: currentGoals);
  }

  void setDeploymentPreference(String preference) {
    state = state.copyWith(deploymentPreference: preference);
  }

  void updateAdminAccount({String? name, String? email, String? password}) {
    state = state.copyWith(
      adminName: name ?? state.adminName,
      adminEmail: email ?? state.adminEmail,
      adminPassword: password ?? state.adminPassword,
    );
  }


  void setWebsiteTemplate(String template) {
    state = state.copyWith(websiteTemplate: template);
  }

  void updateProduct({String? name, String? price}) {
    state = state.copyWith(
      productName: name ?? state.productName,
      productPrice: price ?? state.productPrice,
    );
  }

  void setLive(bool live) {
    state = state.copyWith(isLive: live);
  }

  void setDomain(String domain) {
    state = state.copyWith(domain: domain);
  }

  void updateChecklist(int progress) {
    state = state.copyWith(checklistProgress: progress);
  }

  Future<void> submitWizard() async {
    final data = {
      'companyName': state.companyName,
      'industry': state.industry,
      'size': state.size,
      'goals': state.goals,
      'deploymentPreference': state.deploymentPreference,
      'adminName': state.adminName,
      'adminEmail': state.adminEmail,
      'adminPassword': state.adminPassword,
      'websiteTemplate': state.websiteTemplate,
      'productName': state.productName,
      'productPrice': state.productPrice,
      'domain': state.domain,
    };

    await _apiService.submitBusinessData(data);

    // Proceed to the dashboard
    nextStep();
  }
}

final wizardProvider = NotifierProvider<WizardNotifier, WizardState>(WizardNotifier.new);
