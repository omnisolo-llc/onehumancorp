import { describe, it, expect, vi } from 'vitest';
import { useOnboardingStore } from './store';

describe('useOnboardingStore', () => {
  it('initializes with default values', () => {
    const state = useOnboardingStore.getState();
    expect(state.step).toBe(1);
    expect(state.chatStep).toBe(1);
    expect(state.businessName).toBe('');
    expect(state.websiteTemplate).toBe('Modern');
    expect(state.aiAutoRespond).toBe(true);
  });

  it('updates simple fields', () => {
    const { setBusinessName, setStep } = useOnboardingStore.getState();
    setBusinessName('Maya Cakes');
    setStep(2);

    const state = useOnboardingStore.getState();
    expect(state.businessName).toBe('Maya Cakes');
    expect(state.step).toBe(2);
  });

  it('handles category updates', () => {
    const { setCategories } = useOnboardingStore.getState();
    setCategories(['food', 'baking']);

    const state = useOnboardingStore.getState();
    expect(state.categories).toEqual(['food', 'baking']);
  });

  it('handles AI agents selection', () => {
    const { setAiAgents } = useOnboardingStore.getState();
    setAiAgents(['Sales Agent', 'Support Agent']);

    const state = useOnboardingStore.getState();
    expect(state.aiAgents).toEqual(['Sales Agent', 'Support Agent']);
  });
});
