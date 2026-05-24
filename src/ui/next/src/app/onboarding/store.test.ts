import { describe, it, expect, beforeEach } from 'vitest';
import { useOnboardingStore } from './store';

describe('Onboarding Store', () => {
  beforeEach(() => {
    useOnboardingStore.getState().reset();
  });

  it('initializes with default values', () => {
    const state = useOnboardingStore.getState();
    expect(state.step).toBe(1);
    expect(state.businessType).toBe('');
    expect(state.businessName).toBe('');
    expect(state.businessCategory).toBe('');
    expect(state.intakeData).toBeNull();
    expect(state.startResult).toBeNull();
  });

  it('updates step correctly', () => {
    useOnboardingStore.getState().setStep(2);
    expect(useOnboardingStore.getState().step).toBe(2);
  });

  it('updates business details', () => {
    useOnboardingStore.getState().setBusinessType('Retail');
    useOnboardingStore.getState().setBusinessName('My Shop');
    useOnboardingStore.getState().setBusinessCategory('Clothing');

    const state = useOnboardingStore.getState();
    expect(state.businessType).toBe('Retail');
    expect(state.businessName).toBe('My Shop');
    expect(state.businessCategory).toBe('Clothing');
  });

  it('resets state correctly', () => {
    useOnboardingStore.getState().setStep(3);
    useOnboardingStore.getState().setBusinessName('Test');
    useOnboardingStore.getState().reset();

    const state = useOnboardingStore.getState();
    expect(state.step).toBe(1);
    expect(state.businessName).toBe('');
  });
});
