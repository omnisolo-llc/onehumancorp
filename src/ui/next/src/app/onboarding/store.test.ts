import { useOnboardingStore } from './store';
import { describe, it, expect, beforeEach } from 'vitest';

describe('useOnboardingStore', () => {
  beforeEach(() => {
    localStorage.clear();
    useOnboardingStore.setState({
      step: 1,
      businessDescription: '',
      isLoading: false,
      error: '',
      startResult: null,
      aiAgents: ['Inventory', 'Customer Support'],
      aiTone: 'Professional',
    });
  });

  it('should initialize with default state', () => {
    const state = useOnboardingStore.getState();
    expect(state.step).toBe(1);
    expect(state.businessDescription).toBe('');
    expect(state.isLoading).toBe(false);
    expect(state.error).toBe('');
    expect(state.startResult).toBeNull();
    expect(state.aiAgents).toEqual(['Inventory', 'Customer Support']);
    expect(state.aiTone).toBe('Professional');
  });

  it('should update step', () => {
    useOnboardingStore.getState().setStep(2);
    expect(useOnboardingStore.getState().step).toBe(2);
  });

  it('should update businessDescription', () => {
    useOnboardingStore.getState().setBusinessDescription('Test Description');
    expect(useOnboardingStore.getState().businessDescription).toBe('Test Description');
  });

  it('should update aiAgents', () => {
    useOnboardingStore.getState().setAiAgents(['Marketing']);
    expect(useOnboardingStore.getState().aiAgents).toEqual(['Marketing']);
  });

  it('should update aiTone', () => {
    useOnboardingStore.getState().setAiTone('Playful');
    expect(useOnboardingStore.getState().aiTone).toBe('Playful');
  });

  it('should update isLoading', () => {
    useOnboardingStore.getState().setIsLoading(true);
    expect(useOnboardingStore.getState().isLoading).toBe(true);
  });

  it('should update error', () => {
    useOnboardingStore.getState().setError('Test Error');
    expect(useOnboardingStore.getState().error).toBe('Test Error');
  });

  it('should update startResult', () => {
    useOnboardingStore.getState().setStartResult({ result: 'test' });
    expect(useOnboardingStore.getState().startResult).toEqual({ result: 'test' });
  });

  it('should update new state keys correctly', () => {
    useOnboardingStore.getState().setBusinessName('Test Business');
    useOnboardingStore.getState().setBusinessType('Cafe');
    useOnboardingStore.getState().setCategories(['food', 'drinks']);
    useOnboardingStore.getState().setWebsiteTemplate('Classic');
    useOnboardingStore.getState().setFirstProductName('Coffee');
    useOnboardingStore.getState().setFirstProductPrice('5.00');

    const state = useOnboardingStore.getState();
    expect(state.businessName).toBe('Test Business');
    expect(state.businessType).toBe('Cafe');
    expect(state.categories).toEqual(['food', 'drinks']);
    expect(state.websiteTemplate).toBe('Classic');
    expect(state.firstProductName).toBe('Coffee');
    expect(state.firstProductPrice).toBe('5.00');
  });

  it('should persist state to localStorage', () => {
    useOnboardingStore.getState().setStep(3);
    useOnboardingStore.getState().setBusinessDescription('Persisted Description');
    useOnboardingStore.getState().setBusinessName('Persisted Name');
    useOnboardingStore.getState().setAiAgents(['Inventory']);
    useOnboardingStore.getState().setAiTone('Playful');

    // The state is persisted in localStorage under 'onboarding-storage-v4'
    const storedState = JSON.parse(localStorage.getItem('onboarding-storage-v4') || '{}');
    expect(storedState.state.step).toBe(3);
    expect(storedState.state.businessDescription).toBe('Persisted Description');
    expect(storedState.state.businessName).toBe('Persisted Name');
    expect(storedState.state.aiAgents).toEqual(['Inventory']);
    expect(storedState.state.aiTone).toBe('Playful');
  });
});
