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
  final String? domainChoice;

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
    this.domainChoice,
  });

  Map<String, dynamic> toJson() {
    return {
      'currentStep': currentStep,
      'companyName': companyName,
      'industry': industry,
      'size': size,
      'goals': goals,
      'templateSelection': templateSelection,
      'deploymentPreference': deploymentPreference,
      'adminName': adminName,
      'adminEmail': adminEmail,
      'adminPassword': adminPassword,
      'productName': productName,
      'productDescription': productDescription,
      'productPrice': productPrice,
      'domainChoice': domainChoice,
    };
  }

  factory WizardState.fromJson(Map<String, dynamic> json) {
    return WizardState(
      currentStep: json['currentStep'] ?? 0,
      companyName: json['companyName'],
      industry: json['industry'],
      size: json['size'],
      goals: List<String>.from(json['goals'] ?? []),
      templateSelection: json['templateSelection'],
      deploymentPreference: json['deploymentPreference'],
      adminName: json['adminName'],
      adminEmail: json['adminEmail'],
      adminPassword: json['adminPassword'],
      productName: json['productName'],
      productDescription: json['productDescription'],
      productPrice: json['productPrice'],
      domainChoice: json['domainChoice'],
    );
  }

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
    String? domainChoice,
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
      domainChoice: domainChoice ?? this.domainChoice,
    );
  }
}

class WizardNotifier extends Notifier<WizardState> {
  final ApiService _apiService = ApiService();

  @override
  WizardState build() {
    return WizardState();
  }

  Future<String> generateProductDescription(String name) async {
    return await _apiService.generateDescription(name);
  }

  void goToProductStep() {
    state = state.copyWith(currentStep: 7);
    _apiService.saveState(state.toJson());
  }

  void goToInstagramStep() {
    state = state.copyWith(currentStep: 3);
    _apiService.saveState(state.toJson());
  }

  void nextStep() {
    if (state.currentStep < 11) {
      state = state.copyWith(currentStep: state.currentStep + 1);
      _apiService.saveState(state.toJson());
    }
  }

  void prevStep() {
    if (state.currentStep > 0) {
      state = state.copyWith(currentStep: state.currentStep - 1);
      _apiService.saveState(state.toJson());
    }
  }

  void updateBusinessProfile({String? companyName, String? industry, String? size}) {
    state = state.copyWith(
      companyName: companyName ?? state.companyName,
      industry: industry ?? state.industry,
      size: size ?? state.size,
    );
    _apiService.saveState(state.toJson());
  }

  void toggleGoal(String goal) {
    final currentGoals = List<String>.from(state.goals);
    if (currentGoals.contains(goal)) {
      currentGoals.remove(goal);
    } else {
      currentGoals.add(goal);
    }
    state = state.copyWith(goals: currentGoals);
    _apiService.saveState(state.toJson());
  }

  void setTemplateSelection(String template) {
    state = state.copyWith(templateSelection: template);
    _apiService.saveState(state.toJson());
  }

  void setDeploymentPreference(String preference) {
    state = state.copyWith(deploymentPreference: preference);
    _apiService.saveState(state.toJson());
  }

  void updateAdminAccount({String? name, String? email, String? password}) {
    state = state.copyWith(
      adminName: name ?? state.adminName,
      adminEmail: email ?? state.adminEmail,
      adminPassword: password ?? state.adminPassword,
    );
    _apiService.saveState(state.toJson());
  }

  void updateProductDetails({String? name, String? description, String? price}) {
    state = state.copyWith(
      productName: name ?? state.productName,
      productDescription: description ?? state.productDescription,
      productPrice: price ?? state.productPrice,
    );
    _apiService.saveState(state.toJson());
  }

  void setDomainChoice(String? domain) {
    state = state.copyWith(domainChoice: domain);
    _apiService.saveState(state.toJson());
  }

  Future<void> submitWizard() async {
    final data = state.toJson();
    await _apiService.submitBusinessData(data);
    _apiService.saveState(state.toJson());

    // Proceed to the dashboard
    nextStep();
  }
}

final wizardProvider = NotifierProvider<WizardNotifier, WizardState>(WizardNotifier.new);
