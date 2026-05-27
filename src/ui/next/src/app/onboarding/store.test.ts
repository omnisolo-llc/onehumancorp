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

    // The state is persisted in localStorage under 'onboarding-storage-v3'
    const storedState = JSON.parse(localStorage.getItem('onboarding-storage-v3') || '{}');
    expect(storedState.state.step).toBe(3);
    expect(storedState.state.businessDescription).toBe('Persisted Description');
    expect(storedState.state.businessName).toBe('Persisted Name');
  });

  it('should sync state to server asynchronously', async () => {
    const fetchMock = vi.fn().mockResolvedValue({ ok: true });
    global.fetch = fetchMock;

    // Simulate updating store, which will trigger debounce timeout
    useOnboardingStore.getState().setBusinessName('Sync Test');

    // Wait for the debounce to execute
    await new Promise((r) => setTimeout(r, 1100));

    expect(fetchMock).toHaveBeenCalledWith('/api/onboarding/state', expect.objectContaining({
      method: 'POST',
      headers: expect.objectContaining({ 'Content-Type': 'application/json' })
    }));

    // Test the syncStateToServer explicitly
    const { syncStateToServer } = await import('./store');

    const testState = useOnboardingStore.getState();
    await syncStateToServer(testState);

    expect(fetchMock).toHaveBeenCalledTimes(2);
  });
});