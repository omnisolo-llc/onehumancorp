import { useOnboardingStore } from './store';
import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';

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
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.clearAllMocks();
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

  it('should persist state to backend API', async () => {
    (global.fetch as any).mockResolvedValue({
        ok: true,
        json: async () => ({})
    });

    useOnboardingStore.getState().setStep(3);
    useOnboardingStore.getState().setBusinessDescription('Persisted Description');
    useOnboardingStore.getState().setBusinessName('Persisted Name');

    // wait for async fetch
    await new Promise(resolve => setTimeout(resolve, 0));

    expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/state', expect.objectContaining({
        method: 'POST',
        headers: expect.objectContaining({'Content-Type': 'application/json'})
    }));
  });
});