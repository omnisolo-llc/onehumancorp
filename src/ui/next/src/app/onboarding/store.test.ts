import { describe, it, expect, beforeEach } from 'vitest';
import { useOnboardingStore } from './store';

describe('useOnboardingStore', () => {
  beforeEach(() => {
    // Reset store before each test
    useOnboardingStore.setState({
      step: 0,
      bio: '',
      businessDescription: '',
      businessGoal: '',
      businessName: '',
      businessType: 'Online Store',
      categories: [],
      websiteTemplate: 'Modern',
      domainChoice: 'subdomain',
      firstProductName: '',
      firstProductPrice: '',
      adminName: '',
      adminEmail: '',
      adminPassword: '',
      aiAgents: [],
      aiAutoRespond: true,
      isLoading: false,
      error: '',
      startResult: null,
    });
  });

  it('should initialize with default values', () => {
    const state = useOnboardingStore.getState();
    expect(state.step).toBe(0);
    expect(state.bio).toBe('');
    expect(state.businessType).toBe('Online Store');
    expect(state.websiteTemplate).toBe('Modern');
    expect(state.domainChoice).toBe('subdomain');
    expect(state.aiAutoRespond).toBe(true);
    expect(state.isLoading).toBe(false);
  });

  it('should update step', () => {
    useOnboardingStore.getState().setStep(1);
    expect(useOnboardingStore.getState().step).toBe(1);
  });

  it('should update bio', () => {
    useOnboardingStore.getState().setBio('I am a baker');
    expect(useOnboardingStore.getState().bio).toBe('I am a baker');
  });

  it('should update business name', () => {
    useOnboardingStore.getState().setBusinessName('Maya Bakery');
    expect(useOnboardingStore.getState().businessName).toBe('Maya Bakery');
  });

  it('should update categories', () => {
    useOnboardingStore.getState().setCategories(['Food', 'Bakery']);
    expect(useOnboardingStore.getState().categories).toEqual(['Food', 'Bakery']);
  });

  it('should update admin details', () => {
    const state = useOnboardingStore.getState();
    state.setAdminName('Admin');
    state.setAdminEmail('admin@test.com');
    state.setAdminPassword('password123');

    const updatedState = useOnboardingStore.getState();
    expect(updatedState.adminName).toBe('Admin');
    expect(updatedState.adminEmail).toBe('admin@test.com');
    expect(updatedState.adminPassword).toBe('password123');
  });

  it('should update loading and error state', () => {
    const state = useOnboardingStore.getState();
    state.setIsLoading(true);
    state.setError('Network error');

    const updatedState = useOnboardingStore.getState();
    expect(updatedState.isLoading).toBe(true);
    expect(updatedState.error).toBe('Network error');
  });

  it('should update start result', () => {
    const mockResult = { organization_id: 'org_123', message: 'Success' };
    useOnboardingStore.getState().setStartResult(mockResult);
    expect(useOnboardingStore.getState().startResult).toEqual(mockResult);
  });
});
