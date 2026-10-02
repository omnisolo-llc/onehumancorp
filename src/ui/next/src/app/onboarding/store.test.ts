import { initializeOnboardingDraft } from './store';
import { ownedOnboardingKey } from './draftSession';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
/* @vitest-environment jsdom */
import { useOnboardingStore } from './store';
import { describe, it, expect, beforeEach, vi } from 'vitest';

describe('useOnboardingStore', () => {
  beforeEach(async () => {
    localStorage.clear(); notifyQueueIdentityChange(); await initializeOnboardingDraft();
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
    useOnboardingStore.getState().setStartResult({ message: 'test' });
    expect(useOnboardingStore.getState().startResult).toEqual({ message: 'test' });
  });

  it('should update new state keys correctly', () => {
    useOnboardingStore.getState().setBusinessName('Test Business');
    useOnboardingStore.getState().setBusinessType('Cafe');
    useOnboardingStore.getState().setCategories(['food', 'drinks']);
    useOnboardingStore.getState().setWebsiteTemplate('Classic');
    useOnboardingStore.getState().setFirstProductName('Coffee');
    useOnboardingStore.getState().setFirstProductPrice('5.00');
    useOnboardingStore.getState().setDomainChoice('custom');

    const state = useOnboardingStore.getState();
    expect(state.businessName).toBe('Test Business');
    expect(state.businessType).toBe('Cafe');
    expect(state.categories).toEqual(['food', 'drinks']);
    expect(state.websiteTemplate).toBe('Classic');
    expect(state.firstProductName).toBe('Coffee');
    expect(state.firstProductPrice).toBe('5.00');
    expect(state.domainChoice).toBe('custom');
  });

  it('should support updating multiple state slice with updateState', () => {
    useOnboardingStore.getState().updateState({
      step: 4,
      businessDescription: 'Bulk Description',
      businessName: 'Bulk Name',
    });

    const state = useOnboardingStore.getState();
    expect(state.step).toBe(4);
    expect(state.businessDescription).toBe('Bulk Description');
    expect(state.businessName).toBe('Bulk Name');
  });

  it('should persist state to localStorage', () => {
    useOnboardingStore.getState().setStep(3);
    useOnboardingStore.getState().setBusinessDescription('Persisted Description');
    useOnboardingStore.getState().setBusinessName('Persisted Name');

    // The state is persisted in localStorage under ownedOnboardingKey('draft')!
    const storedState = JSON.parse(localStorage.getItem(ownedOnboardingKey('draft')!) || '{}');
    expect(storedState.state.step).toBe(3);
    expect(storedState.state.businessDescription).toBe('Persisted Description');
    expect(storedState.state.businessName).toBe('Persisted Name');
  });

  it('never persists the administrator password', () => {
    useOnboardingStore.getState().setStep(2);

    const storedState = JSON.parse(localStorage.getItem(ownedOnboardingKey('draft')!) || '{}');
    expect(storedState.state.adminPassword).toBeUndefined();
    expect(JSON.stringify(storedState)).not.toContain('NeverPersist123');
  });

  it('purges administrator passwords from old-schema owner-bound state', async () => {
    localStorage.setItem(
      ownedOnboardingKey('draft')!,
      JSON.stringify({
        state: { step: 2, adminPassword: 'LegacySecret123' },
        version: 4,
      }),
    );

    await useOnboardingStore.persist.rehydrate();

    expect(useOnboardingStore.getState()).not.toHaveProperty('adminPassword');
    expect(localStorage.getItem(ownedOnboardingKey('draft')!)).not.toContain(
      'LegacySecret123',
    );
  });
});

it('does not persist in-flight, error, or completion claims with a business draft', () => {
 useOnboardingStore.setState({ step: 5, isLoading: true, error: 'Old failure', startResult: { message: 'Old success' }, businessName: 'Keep business' });
 const state = JSON.parse(localStorage.getItem(ownedOnboardingKey('draft')!)!).state;
 expect(state.isLoading).toBeUndefined(); expect(state.error).toBeUndefined(); expect(state.startResult).toBeUndefined();
 expect(state.step).toBe(3); expect(state.businessName).toBe('Keep business');
});
it('purges stale runtime fields and credentials from the current owner-bound version', async () => {
 localStorage.setItem(ownedOnboardingKey('draft')!, JSON.stringify({ version: 6, state: { step: 5, isLoading: true, startResult: { status: 'launched' }, adminPassword: 'stale-secret', businessName: 'Kept' } }));
 await useOnboardingStore.persist.rehydrate();
 const state = useOnboardingStore.getState();
 expect(state.step).toBe(3); expect(state.isLoading).toBe(false); expect(state.startResult).toBeNull(); expect(state).not.toHaveProperty('adminPassword'); expect(state.businessName).toBe('Kept');
});

vi.mock('@/lib/sync/queueIdentity', async importOriginal => ({ ...await importOriginal<typeof import('@/lib/sync/queueIdentity')>(), readQueueOwner: vi.fn(async () => ({ userId: 'user-1', tenantId: 'tenant-1' })) }));
