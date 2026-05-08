import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/api_service.dart';

class WizardState {
  final int currentStep;
  final String? companyName;
  final String? industry;
  final String? size;
  final List<String> goals;
  final String? selectedTemplate;
  final String? productName;
  final String? productDescription;
  final String? productPrice;
  final String? deploymentPreference;
  final String? adminName;
  final String? adminEmail;
  final String? adminPassword;
  final String? subdomain;
  final bool isLoading;

  WizardState({
    this.currentStep = 0,
    this.companyName,
    this.industry,
    this.size,
    this.goals = const [],
    this.selectedTemplate,
    this.productName,
    this.productDescription,
    this.productPrice,
    this.deploymentPreference,
    this.adminName,
    this.adminEmail,
    this.adminPassword,
    this.subdomain,
    this.isLoading = true,
  });

  WizardState copyWith({
    int? currentStep,
    String? companyName,
    String? industry,
    String? size,
    List<String>? goals,
    String? selectedTemplate,
    String? productName,
    String? productDescription,
    String? productPrice,
    String? deploymentPreference,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
    String? subdomain,
    bool? isLoading,
  }) {
    return WizardState(
      currentStep: currentStep ?? this.currentStep,
      companyName: companyName ?? this.companyName,
      industry: industry ?? this.industry,
      size: size ?? this.size,
      goals: goals ?? this.goals,
      selectedTemplate: selectedTemplate ?? this.selectedTemplate,
      productName: productName ?? this.productName,
      productDescription: productDescription ?? this.productDescription,
      productPrice: productPrice ?? this.productPrice,
      deploymentPreference: deploymentPreference ?? this.deploymentPreference,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
      subdomain: subdomain ?? this.subdomain,
      isLoading: isLoading ?? this.isLoading,
    );
  }

  Map<String, dynamic> toMap() {
    return {
      'currentStep': currentStep,
      'companyName': companyName,
      'industry': industry,
      'size': size,
      'goals': goals,
      'selectedTemplate': selectedTemplate,
      'productName': productName,
      'productDescription': productDescription,
      'productPrice': productPrice,
      'deploymentPreference': deploymentPreference,
      'adminName': adminName,
      'adminEmail': adminEmail,
      'subdomain': subdomain,
    };
  }

  factory WizardState.fromMap(Map<String, dynamic> map) {
    return WizardState(
      currentStep: map['currentStep'] ?? 0,
      companyName: map['companyName'],
      industry: map['industry'],
      size: map['size'],
      goals: List<String>.from(map['goals'] ?? []),
      selectedTemplate: map['selectedTemplate'],
      productName: map['productName'],
      productDescription: map['productDescription'],
      productPrice: map['productPrice'],
      deploymentPreference: map['deploymentPreference'],
      adminName: map['adminName'],
      adminEmail: map['adminEmail'],
      subdomain: map['subdomain'],
      isLoading: false,
    );
  }
}

class WizardNotifier extends Notifier<WizardState> {
  final ApiService _apiService = ApiService();

  @override
  WizardState build() {
    _loadState();
    return WizardState();
  }

  Future<void> _loadState() async {
    final savedState = await _apiService.getWizardState();
    if (savedState != null) {
      state = WizardState.fromMap(savedState);
    } else {
      state = state.copyWith(isLoading: false);
    }
  }

  Future<void> _saveState() async {
    await _apiService.saveWizardState(state.toMap());
  }

  void nextStep() {
    if (state.currentStep < 10) {
      state = state.copyWith(currentStep: state.currentStep + 1);
      _saveState();
    }
  }

  void prevStep() {
    if (state.currentStep > 0) {
      state = state.copyWith(currentStep: state.currentStep - 1);
      _saveState();
    }
  }

  void updateBusinessProfile({String? companyName, String? industry, String? size}) {
    state = state.copyWith(
      companyName: companyName ?? state.companyName,
      industry: industry ?? state.industry,
      size: size ?? state.size,
      subdomain: companyName != null ? '${companyName.toLowerCase().replaceAll(' ', '')}.ohc.app' : state.subdomain,
    );
    _saveState();
  }

  void toggleGoal(String goal) {
    final currentGoals = List<String>.from(state.goals);
    if (currentGoals.contains(goal)) {
      currentGoals.remove(goal);
    } else {
      currentGoals.add(goal);
    }
    state = state.copyWith(goals: currentGoals);
    _saveState();
  }

  void setTemplate(String template) {
    state = state.copyWith(selectedTemplate: template);
    _saveState();
  }

  void updateProduct({String? name, String? price, String? description}) {
    state = state.copyWith(
      productName: name ?? state.productName,
      productPrice: price ?? state.productPrice,
      productDescription: description ?? state.productDescription,
    );
    _saveState();
  }

  Future<void> generateAiDescription() async {
    if (state.productName == null || state.productName!.isEmpty) return;

    state = state.copyWith(productDescription: "Generating...");
    await Future.delayed(const Duration(seconds: 1));
    state = state.copyWith(productDescription: "Premium ${state.productName} crafted with care. Perfect for your daily needs and fully customizable.");
    _saveState();
  }

  void setDeploymentPreference(String preference) {
    state = state.copyWith(deploymentPreference: preference);
    _saveState();
  }

  void updateAdminAccount({String? name, String? email, String? password}) {
    state = state.copyWith(
      adminName: name ?? state.adminName,
      adminEmail: email ?? state.adminEmail,
      adminPassword: password ?? state.adminPassword,
    );
    _saveState();
  }

  Future<void> submitWizard() async {
    final data = state.toMap();
    data['adminPassword'] = state.adminPassword; // not saved in local storage, but sent to API

    await _apiService.submitBusinessData(data);

    // Proceed to the dashboard
    nextStep();
  }
}

final wizardProvider = NotifierProvider<WizardNotifier, WizardState>(WizardNotifier.new);
