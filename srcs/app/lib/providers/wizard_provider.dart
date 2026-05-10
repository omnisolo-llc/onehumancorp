import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/api_service.dart';

class WizardState {
  final int currentStep;
  final String? companyName;
  final String? industry;
  final String? size;
  final List<String> goals;
  final String? templateSelection;
  final String? deploymentPreference;
  final String? adminName;
  final String? adminEmail;
  final String? adminPassword;
  final String? productName;
  final String? productDescription;
  final String? productPrice;
  final String? domainName;

  WizardState({
    this.currentStep = 0,
    this.companyName,
    this.industry,
    this.size,
    this.goals = const [],
    this.templateSelection,
    this.deploymentPreference,
    this.adminName,
    this.adminEmail,
    this.adminPassword,
    this.productName,
    this.productDescription,
    this.productPrice,
    this.domainName,
  });

  WizardState copyWith({
    int? currentStep,
    String? companyName,
    String? industry,
    String? size,
    List<String>? goals,
    String? templateSelection,
    String? deploymentPreference,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
    String? productName,
    String? productDescription,
    String? productPrice,
    String? domainName,
  }) {
    return WizardState(
      currentStep: currentStep ?? this.currentStep,
      companyName: companyName ?? this.companyName,
      industry: industry ?? this.industry,
      size: size ?? this.size,
      goals: goals ?? this.goals,
      templateSelection: templateSelection ?? this.templateSelection,
      deploymentPreference: deploymentPreference ?? this.deploymentPreference,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
      productName: productName ?? this.productName,
      productDescription: productDescription ?? this.productDescription,
      productPrice: productPrice ?? this.productPrice,
      domainName: domainName ?? this.domainName,
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

  void setTemplateSelection(String template) {
    state = state.copyWith(templateSelection: template);
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

  void updateFirstProduct({String? name, String? description, String? price}) {
    state = state.copyWith(
      productName: name ?? state.productName,
      productDescription: description ?? state.productDescription,
      productPrice: price ?? state.productPrice,
    );
  }

  void setDomainName(String domain) {
    state = state.copyWith(domainName: domain);
  }

  Future<void> submitWizard() async {
    final data = {
      'companyName': state.companyName,
      'industry': state.industry,
      'size': state.size,
      'goals': state.goals,
      'templateSelection': state.templateSelection,
      'deploymentPreference': state.deploymentPreference,
      'adminName': state.adminName,
      'adminEmail': state.adminEmail,
      'adminPassword': state.adminPassword,
      'productName': state.productName,
      'productDescription': state.productDescription,
      'productPrice': state.productPrice,
      'domainName': state.domainName,
    };

    await _apiService.submitBusinessData(data);

    // Proceed to the dashboard
    nextStep();
  }
}

final wizardProvider = NotifierProvider<WizardNotifier, WizardState>(WizardNotifier.new);
