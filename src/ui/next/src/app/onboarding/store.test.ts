import { useOnboardingStore } from './store';
import { describe, it, expect, beforeEach } from 'vitest';

describe('useOnboardingStore', () => {
  beforeEach(() => {
    // Clear localStorage mock
    const localStorageMock = (() => {
      let store: Record<string, string> = {};
      return {
        getItem: (key: string) => store[key] || null,
        setItem: (key: string, value: string) => {
          store[key] = value.toString();
        },
        removeItem: (key: string) => {
          delete store[key];
        },
        clear: () => {
          store = {};
        },
      };
    })();

    if (typeof window !== 'undefined') {
      Object.defineProperty(window, 'localStorage', {
        value: localStorageMock,
      });
      window.localStorage.clear();
    }
  });

  it('should initialize with default state', () => {
    const state = useOnboardingStore.getState();
    expect(state.step).toBe(1);
    expect(state.businessName).toBe('');
    expect(state.businessCategory).toBe('');
    expect(state.isLoading).toBe(false);
    expect(state.error).toBe('');
    expect(state.intakeData).toBeNull();
    expect(state.startResult).toBeNull();
  });

  it('should update step', () => {
    useOnboardingStore.getState().setStep(2);
    expect(useOnboardingStore.getState().step).toBe(2);
  });

  it('should update businessName', () => {
    useOnboardingStore.getState().setBusinessName('Test Name');
    expect(useOnboardingStore.getState().businessName).toBe('Test Name');
  });

  it('should update businessCategory', () => {
    useOnboardingStore.getState().setBusinessCategory('Test Category');
    expect(useOnboardingStore.getState().businessCategory).toBe('Test Category');
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
