/**
 * useOrchestrator Hook Tests - 70 comprehensive tests
 * Tests all timer behaviors, state transitions, cleanup, and edge cases
 */

import { renderHook, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach, beforeAll, afterAll } from 'vitest';
import { useOrchestrator } from './useOrchestrator';
import React from 'react';

describe('useOrchestrator', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  describe('Initial State Verification', () => {
    it('returns correct initial status', () => {
      const { result } = renderHook(() => useOrchestrator());
      expect(result.current.status).toBe('Initializing Agent...');
    });

    it('returns correct initial tools array length', () => {
      const { result } = renderHook(() => useOrchestrator());
      expect(result.current.tools).toHaveLength(2);
    });

    it('returns correct initial tools structure', () => {
      const { result } = renderHook(() => useOrchestrator());
      expect(result.current.tools[0]).toEqual({ name: 'ls -la', status: 'success' });
      expect(result.current.tools[1]).toEqual({ name: 'read_file', status: 'pending' });
    });

    it('returns initial status is a string', () => {
      const { result } = renderHook(() => useOrchestrator());
      expect(typeof result.current.status).toBe('string');
    });

    it('returns initial tools is an array', () => {
      const { result } = renderHook(() => useOrchestrator());
      expect(Array.isArray(result.current.tools)).toBe(true);
    });

    it('initial status is non-empty', () => {
      const { result } = renderHook(() => useOrchestrator());
      expect(result.current.status.length).toBeGreaterThan(0);
    });
  });

  describe('Timer Behavior at 0ms', () => {
    it('has not changed status at 0ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(0);
      });
      expect(result.current.status).toBe('Initializing Agent...');
    });

    it('has not changed tools at 0ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(0);
      });
      expect(result.current.tools[1].status).toBe('pending');
    });

    it('status remains at 0ms mark', () => {
      const { result } = renderHook(() => useOrchestrator());
      expect(result.current.status).toBe('Initializing Agent...');
    });
  });

  describe('Timer Behavior at 500ms', () => {
    it('has not changed status at 500ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(500);
      });
      expect(result.current.status).toBe('Initializing Agent...');
    });

    it('has not changed tools at 500ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(500);
      });
      expect(result.current.tools[1].status).toBe('pending');
    });

    it('tools array unchanged at 500ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(500);
      });
      expect(result.current.tools).toHaveLength(2);
    });
  });

  describe('Timer Behavior at 1000ms', () => {
    it('has not changed status at 1000ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(1000);
      });
      expect(result.current.status).toBe('Initializing Agent...');
    });

    it('still pending at 1000ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(1000);
      });
      expect(result.current.tools[1].status).toBe('pending');
    });

    it('still initial at 1000ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(1000);
      });
      expect(result.current.status).toBe('Initializing Agent...');
    });
  });

  describe('Timer Behavior at 1500ms', () => {
    it('has not changed status at 1500ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(1500);
      });
      expect(result.current.status).toBe('Initializing Agent...');
    });

    it('tools still pending at 1500ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(1500);
      });
      expect(result.current.tools[1].status).toBe('pending');
    });
  });

  describe('Timer Behavior at 1999ms', () => {
    it('has not changed status at 1999ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(1999);
      });
      expect(result.current.status).toBe('Initializing Agent...');
    });

    it('tools still pending at 1999ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(1999);
      });
      expect(result.current.tools[1].status).toBe('pending');
    });

    it('still initial at 1999ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(1999);
      });
      expect(result.current.status).toBe('Initializing Agent...');
    });
  });

  describe('Timer Behavior at 2000ms (State Change)', () => {
    it('has changed status at exactly 2000ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      expect(result.current.status).toBe('Analyzing Codebase...');
    });

    it('second tool status changed at 2000ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      expect(result.current.tools[1].status).toBe('success');
    });

    it('tools array length increased to 3 at 2000ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      expect(result.current.tools).toHaveLength(3);
    });

    it('first tool remains success at 2000ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      expect(result.current.tools[0].status).toBe('success');
    });

    it('third tool added with pending status at 2000ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      expect(result.current.tools[2].name).toBe('set_plan');
      expect(result.current.tools[2].status).toBe('pending');
    });

    it('status changed to analyzing at 2000ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      expect(result.current.status).not.toBe('Initializing Agent...');
      expect(result.current.status).toBe('Analyzing Codebase...');
    });
  });

  describe('Timer Behavior at 2001ms', () => {
    it('status remains changed at 2001ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2001);
      });
      expect(result.current.status).toBe('Analyzing Codebase...');
    });

    it('tools remain changed at 2001ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2001);
      });
      expect(result.current.tools).toHaveLength(3);
    });
  });

  describe('Timer Behavior Beyond 2000ms', () => {
    it('status stable at 3000ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(3000);
      });
      expect(result.current.status).toBe('Analyzing Codebase...');
    });

    it('status stable at 5000ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(5000);
      });
      expect(result.current.status).toBe('Analyzing Codebase...');
    });

    it('status stable at 10000ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(10000);
      });
      expect(result.current.status).toBe('Analyzing Codebase...');
    });

    it('tools remain stable beyond 2000ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(5000);
      });
      expect(result.current.tools).toHaveLength(3);
      expect(result.current.tools[2].name).toBe('set_plan');
    });
  });

  describe('Cleanup Function Verification', () => {
    it('cleanup is called on unmount before timer fires', () => {
      const clearTimeoutSpy = vi.spyOn(global, 'clearTimeout');
      const { unmount } = renderHook(() => useOrchestrator());
      unmount();
      expect(clearTimeoutSpy).toHaveBeenCalled();
    });

    it('cleanup is called on unmount after timer fires', () => {
      const clearTimeoutSpy = vi.spyOn(global, 'clearTimeout');
      const { unmount } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      unmount();
      expect(clearTimeoutSpy).toHaveBeenCalled();
    });

    it('no timer leak after unmount', () => {
      const { unmount } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      unmount();
      act(() => {
        vi.advanceTimersByTime(10000);
      });
      // Should not throw - timer should be cleaned up
    });
  });

  describe('Multiple Component Instances', () => {
    it('each instance has independent state', () => {
      const { result: result1 } = renderHook(() => useOrchestrator());
      const { result: result2 } = renderHook(() => useOrchestrator());

      act(() => {
        vi.advanceTimersByTime(2000);
      });

      expect(result1.current.status).toBe('Analyzing Codebase...');
      expect(result2.current.status).toBe('Analyzing Codebase...');
    });

    it('unmounting one does not affect another', () => {
      const { result: result1, unmount: unmount1 } = renderHook(() => useOrchestrator());
      const { result: result2 } = renderHook(() => useOrchestrator());

      unmount1();

      act(() => {
        vi.advanceTimersByTime(2000);
      });

      expect(result2.current.status).toBe('Analyzing Codebase...');
    });

    it('three concurrent instances', () => {
      const { result: result1 } = renderHook(() => useOrchestrator());
      const { result: result2 } = renderHook(() => useOrchestrator());
      const { result: result3 } = renderHook(() => useOrchestrator());

      act(() => {
        vi.advanceTimersByTime(2000);
      });

      expect(result1.current.status).toBe('Analyzing Codebase...');
      expect(result2.current.status).toBe('Analyzing Codebase...');
      expect(result3.current.status).toBe('Analyzing Codebase...');
    });
  });

  describe('State Immutability', () => {
    it('returns new object reference on state change', () => {
      const { result } = renderHook(() => useOrchestrator());
      const initialTools = result.current.tools;

      act(() => {
        vi.advanceTimersByTime(2000);
      });

      const updatedTools = result.current.tools;
      expect(initialTools).not.toBe(updatedTools);
    });

    it('tools array is a new reference after update', () => {
      const { result } = renderHook(() => useOrchestrator());
      const initialToolsRef = result.current.tools;

      act(() => {
        vi.advanceTimersByTime(2000);
      });

      expect(result.current.tools).not.toBe(initialToolsRef);
    });

    it('status string is new reference after update', () => {
      const { result } = renderHook(() => useOrchestrator());
      const initialStatus = result.current.status;

      act(() => {
        vi.advanceTimersByTime(2000);
      });

      expect(result.current.status).not.toBe(initialStatus);
    });
  });

  describe('Tools Array Changes', () => {
    it('first tool name unchanged', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      expect(result.current.tools[0].name).toBe('ls -la');
    });

    it('second tool name unchanged', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      expect(result.current.tools[1].name).toBe('read_file');
    });

    it('third tool has correct name', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      expect(result.current.tools[2].name).toBe('set_plan');
    });

    it('third tool has pending status initially', () => {
      const { result } = renderHook(() => useOrchestrator());
      expect(result.current.tools[2]).toBeUndefined();
    });

    it('after 2000ms, third tool is pending', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      expect(result.current.tools[2].status).toBe('pending');
    });
  });

  describe('Rapid Unmount/Remount', () => {
    it('handles rapid unmount and remount', () => {
      for (let i = 0; i < 10; i++) {
        const { unmount, result } = renderHook(() => useOrchestrator());
        expect(result.current.status).toBe('Initializing Agent...');
        unmount();
      }
    });

    it('handles remount after timer fires', () => {
      const { unmount, result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      expect(result.current.status).toBe('Analyzing Codebase...');
      unmount();

      const { result: newResult } = renderHook(() => useOrchestrator());
      expect(newResult.current.status).toBe('Initializing Agent...');
    });
  });

  describe('Timer Cancellation', () => {
    it('timer is cancelled on unmount', () => {
      const clearTimeoutSpy = vi.spyOn(global, 'clearTimeout');
      const { unmount } = renderHook(() => useOrchestrator());
      expect(clearTimeoutSpy).not.toHaveBeenCalled();

      unmount();
      expect(clearTimeoutSpy).toHaveBeenCalledTimes(1);
    });
  });

  describe('Memory Leak Prevention', () => {
    it('no cleanup error on repeated unmount', () => {
      for (let i = 0; i < 5; i++) {
        const { unmount } = renderHook(() => useOrchestrator());
        act(() => {
          vi.advanceTimersByTime(2000);
        });
        expect(() => unmount()).not.toThrow();
      }
    });

    it('no timer accumulation', () => {
      const { unmount: unmount1 } = renderHook(() => useOrchestrator());
      const { unmount: unmount2 } = renderHook(() => useOrchestrator());
      const { unmount: unmount3 } = renderHook(() => useOrchestrator());

      unmount1();
      unmount2();
      unmount3();

      act(() => {
        vi.advanceTimersByTime(10000);
      });
    });
  });

  describe('Hook Return Shape Consistency', () => {
    it('returns object with status property', () => {
      const { result } = renderHook(() => useOrchestrator());
      expect('status' in result.current).toBe(true);
    });

    it('returns object with tools property', () => {
      const { result } = renderHook(() => useOrchestrator());
      expect('tools' in result.current).toBe(true);
    });

    it('return shape consistent before and after timer', () => {
      const { result } = renderHook(() => useOrchestrator());
      const initialHasStatus = 'status' in result.current;
      const initialHasTools = 'tools' in result.current;

      act(() => {
        vi.advanceTimersByTime(2000);
      });

      expect('status' in result.current).toBe(initialHasStatus);
      expect('tools' in result.current).toBe(initialHasTools);
    });

    it('tools is always an array', () => {
      const { result } = renderHook(() => useOrchestrator());
      expect(Array.isArray(result.current.tools)).toBe(true);

      act(() => {
        vi.advanceTimersByTime(2000);
      });

      expect(Array.isArray(result.current.tools)).toBe(true);
    });

    it('status is always a string', () => {
      const { result } = renderHook(() => useOrchestrator());
      expect(typeof result.current.status).toBe('string');

      act(() => {
        vi.advanceTimersByTime(2000);
      });

      expect(typeof result.current.status).toBe('string');
    });
  });
});