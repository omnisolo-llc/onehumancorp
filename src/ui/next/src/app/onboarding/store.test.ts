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
    });
  });

  it('should initialize with default state', () => {
    const state = useOnboardingStore.getState();
    expect(state.step).toBe(1);
    expect(state.businessDescription).toBe('');
    expect(state.isLoading).toBe(false);
    expect(state.error).toBe('');
    expect(state.startResult).toBeNull();
  });

  it('should update step', () => {
    useOnboardingStore.getState().setStep(2);
    expect(useOnboardingStore.getState().step).toBe(2);
  });

  it('should update businessDescription', () => {
    useOnboardingStore.getState().setBusinessDescription('Test Description');
    expect(useOnboardingStore.getState().businessDescription).toBe('Test Description');
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

  it('should persist state to localStorage', () => {
    useOnboardingStore.getState().setStep(3);
    useOnboardingStore.getState().setBusinessDescription('Persisted Description');

    // The state is persisted in localStorage under 'onboarding-storage-v2'
    const storedState = JSON.parse(localStorage.getItem('onboarding-storage-v2') || '{}');
    expect(storedState.state.step).toBe(3);
    expect(storedState.state.businessDescription).toBe('Persisted Description');
  });
});