import { describe, it, expect, beforeEach } from 'vitest';
import { useOnboardingStore } from './store';

describe('useOnboardingStore', () => {
  beforeEach(() => {
    useOnboardingStore.setState({
      step: 1,
      businessType: '',
      uploadedPhotos: [],
      businessName: '',
      isLoading: false,
      error: '',
      startResult: null
    });
  });

  it('should initialize with default state', () => {
    const state = useOnboardingStore.getState();
    expect(state.step).toBe(1);
    expect(state.businessType).toBe('');
    expect(state.uploadedPhotos).toEqual([]);
    expect(state.businessName).toBe('');
    expect(state.isLoading).toBe(false);
    expect(state.error).toBe('');
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

  it('should update businessType', () => {
    useOnboardingStore.getState().setBusinessType('Cafe');
    expect(useOnboardingStore.getState().businessType).toBe('Cafe');
  });

  it('should update uploadedPhotos', () => {
    useOnboardingStore.getState().setUploadedPhotos(['photo1.jpg']);
    expect(useOnboardingStore.getState().uploadedPhotos).toEqual(['photo1.jpg']);
  });

  it('should update isLoading', () => {
    useOnboardingStore.getState().setIsLoading(true);
    expect(useOnboardingStore.getState().isLoading).toBe(true);
  });

  it('should update error', () => {
    useOnboardingStore.getState().setError('test error');
    expect(useOnboardingStore.getState().error).toBe('test error');
  });

  it('should update startResult', () => {
    useOnboardingStore.getState().setStartResult({ result: 'test' });
    expect(useOnboardingStore.getState().startResult).toEqual({ result: 'test' });
  });
});
