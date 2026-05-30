import { describe, it, expect, beforeEach } from 'vitest';
import { useOnboardingStore } from './store';

describe('useOnboardingStore', () => {
  beforeEach(() => {
    localStorage.clear();
    useOnboardingStore.setState({
      step: 1,
      businessName: '',
      whatYouSell: '',
      style: '',
      firstProductName: '',
      firstProductPrice: '',
      isLoading: false,
      error: '',
      startResult: null,
    });
  });

  it('should initialize with default state', () => {
    const state = useOnboardingStore.getState();
    expect(state.step).toBe(1);
    expect(state.businessName).toBe('');
    expect(state.whatYouSell).toBe('');
    expect(state.style).toBe('');
    expect(state.isLoading).toBe(false);
  });

  it('should update step', () => {
    useOnboardingStore.getState().setStep(2);
    expect(useOnboardingStore.getState().step).toBe(2);
  });

  it('should update whatYouSell', () => {
    useOnboardingStore.getState().setWhatYouSell('Test Description');
    expect(useOnboardingStore.getState().whatYouSell).toBe('Test Description');
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
    const result = { success: true };
    useOnboardingStore.getState().setStartResult(result);
    expect(useOnboardingStore.getState().startResult).toEqual(result);
  });

  it('should update new state keys correctly', () => {
    useOnboardingStore.getState().setBusinessName('Test Business');
    useOnboardingStore.getState().setStyle('Elegant');
    useOnboardingStore.getState().setFirstProductName('Test Product');
    useOnboardingStore.getState().setFirstProductPrice('100');

    const state = useOnboardingStore.getState();
    expect(state.businessName).toBe('Test Business');
    expect(state.style).toBe('Elegant');
    expect(state.firstProductName).toBe('Test Product');
    expect(state.firstProductPrice).toBe('100');
  });

  it('should persist state to localStorage', () => {
    useOnboardingStore.getState().setStep(3);
    useOnboardingStore.getState().setWhatYouSell('Persisted Description');
    useOnboardingStore.getState().setBusinessName('Persisted Name');

    // The state is persisted in localStorage under 'onboarding-storage-v4'
    const storedState = JSON.parse(localStorage.getItem('onboarding-storage-v4') || '{}');

    expect(storedState.state.step).toBe(3);
    expect(storedState.state.whatYouSell).toBe('Persisted Description');
    expect(storedState.state.businessName).toBe('Persisted Name');
  });
});
