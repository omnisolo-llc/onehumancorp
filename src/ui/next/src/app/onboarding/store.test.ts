import { useOnboardingStore } from './store';
import { describe, it, expect, beforeEach, vi } from 'vitest';

// Mock fetch globally
global.fetch = vi.fn().mockResolvedValue({
  ok: true,
  json: async () => ({})
});

describe('useOnboardingStore', () => {
  beforeEach(() => {
    localStorage.clear();
    useOnboardingStore.setState({
      step: 1,
      businessType: '',
      businessName: '',
      businessCategory: '',
      isLoading: false,
      error: '',
      intakeData: null,
      startResult: null,
    });
    vi.clearAllMocks();
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

  it('should update step and sync to server', async () => {
    useOnboardingStore.getState().setStep(2);
    expect(useOnboardingStore.getState().step).toBe(2);

    // Fast-forward debounce
    await new Promise(r => setTimeout(r, 600));

    expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/state', expect.objectContaining({
      method: 'POST',
      body: expect.stringContaining('"step":2')
    }));
  });

  it('should update businessName and sync to server', async () => {
    useOnboardingStore.getState().setBusinessName('Test Name');
    expect(useOnboardingStore.getState().businessName).toBe('Test Name');

    await new Promise(r => setTimeout(r, 600));

    expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/state', expect.objectContaining({
      method: 'POST',
      body: expect.stringContaining('"businessName":"Test Name"')
    }));
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

  it('should persist state to localStorage', () => {
    useOnboardingStore.getState().setStep(3);
    useOnboardingStore.getState().setBusinessName('Persisted Name');

    // The state is persisted in localStorage under 'onboarding-storage'
    const storedState = JSON.parse(localStorage.getItem('onboarding-storage') || '{}');
    expect(storedState.state.step).toBe(3);
    expect(storedState.state.businessName).toBe('Persisted Name');
  });

  it('should load state from server correctly', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        step: 4,
        businessName: 'Server Name',
        businessType: 'Server Type'
      })
    });

    await useOnboardingStore.getState().loadFromServer();

    expect(useOnboardingStore.getState().step).toBe(4);
    expect(useOnboardingStore.getState().businessName).toBe('Server Name');
    expect(useOnboardingStore.getState().businessType).toBe('Server Type');
  });
});
