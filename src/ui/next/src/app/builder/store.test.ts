import { describe, expect, it } from 'vitest';
import { useBuilderStore } from './store';

describe('useBuilderStore', () => {
  it('should initialize with correct default state', () => {
    const state = useBuilderStore.getState();
    expect(state.bio).toBe('');
    expect(state.businessName).toBe('');
    expect(state.businessCategory).toBe('');
    expect(state.vibe).toBe('');
    expect(state.wizardStep).toBe(1);
    expect(state.blocks).toEqual([]);
    expect(state.drafts).toEqual([]);
    expect(state.status).toBe('onboarding');
    expect(state.businessGoal).toBeNull();
    expect(state.liveUrl).toBe('');
  });

  it('should update simple string fields correctly', () => {
    useBuilderStore.getState().setBio('New bio');
    expect(useBuilderStore.getState().bio).toBe('New bio');

    useBuilderStore.getState().setBusinessName('New business name');
    expect(useBuilderStore.getState().businessName).toBe('New business name');

    useBuilderStore.getState().setBusinessCategory('Bakery');
    expect(useBuilderStore.getState().businessCategory).toBe('Bakery');

    useBuilderStore.getState().setVibe('Modern');
    expect(useBuilderStore.getState().vibe).toBe('Modern');

    useBuilderStore.getState().setLiveUrl('https://example.com');
    expect(useBuilderStore.getState().liveUrl).toBe('https://example.com');
  });

  it('should update complex fields correctly', () => {
    useBuilderStore.getState().setWizardStep(2);
    expect(useBuilderStore.getState().wizardStep).toBe(2);

    const newBlocks = [{ type: 'hero' }];
    useBuilderStore.getState().setBlocks(newBlocks);
    expect(useBuilderStore.getState().blocks).toEqual(newBlocks);

    const newDrafts = [[{ type: 'hero' }]];
    useBuilderStore.getState().setDrafts(newDrafts);
    expect(useBuilderStore.getState().drafts).toEqual(newDrafts);

    useBuilderStore.getState().setStatus('live');
    expect(useBuilderStore.getState().status).toBe('live');

    useBuilderStore.getState().setBusinessGoal('services');
    expect(useBuilderStore.getState().businessGoal).toBe('services');
  });
});
