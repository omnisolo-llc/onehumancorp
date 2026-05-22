import { useOnboardingStore } from './store';
import { describe, it, expect } from 'vitest';

describe('useOnboardingStore', () => {
  it('should initialize with default state', () => {
    const state = useOnboardingStore.getState();
    expect(state.step).toBe(1);
    expect(state.bio).toBe('');
        expect(state.isLoading).toBe(false);
    expect(state.error).toBe('');
    expect(state.intakeData).toBeNull();
    expect(state.startResult).toBeNull();
  });

  it('should update step', () => {
    useOnboardingStore.getState().setStep(2);
    expect(useOnboardingStore.getState().step).toBe(2);
  });

  it('should update bio', () => {
    useOnboardingStore.getState().setBio('Test Bio');
    expect(useOnboardingStore.getState().bio).toBe('Test Bio');
  });


  it('should update isLoading', () => {
    useOnboardingStore.getState().setIsLoading(true);
    expect(useOnboardingStore.getState().isLoading).toBe(true);
  });

  it('should update error', () => {
    useOnboardingStore.getState().setError('Test Error');
    expect(useOnboardingStore.getState().error).toBe('Test Error');
  });

  it('should update intakeData', () => {
    useOnboardingStore.getState().setIntakeData({ data: 'test' });
    expect(useOnboardingStore.getState().intakeData).toEqual({ data: 'test' });
  });

  it('should update startResult', () => {
    useOnboardingStore.getState().setStartResult({ result: 'test' });
    expect(useOnboardingStore.getState().startResult).toEqual({ result: 'test' });
  });
});
