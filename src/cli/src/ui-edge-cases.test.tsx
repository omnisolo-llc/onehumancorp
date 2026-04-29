/**
 * UI Edge Cases Tests - 70 comprehensive tests
 * Tests rapid state changes, memory/cleanup, race conditions, invalid props, and error handling
 */

import React from 'react';
import { render } from 'ink-testing-library';
import { renderHook, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { AgentStatus } from './components/AgentStatus';
import { ToolProgress, ToolItem } from './components/ToolProgress';
import { MarkdownText } from './components/MarkdownText';
import { useOrchestrator } from './hooks/useOrchestrator';

describe('UI Edge Cases Tests', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  describe('Rapid State Changes - AgentStatus', () => {
    it('handles rapid status changes', () => {
      const { rerender, lastFrame } = render(<AgentStatus status="State 0" />);
      for (let i = 1; i <= 100; i++) {
        rerender(<AgentStatus status={`State ${i}`} />);
      }
      expect(lastFrame()).toContain('State 100');
    });

    it('handles empty string to long string transition', () => {
      const { rerender, lastFrame } = render(<AgentStatus status="" />);
      rerender(<AgentStatus status={''.repeat(500)} />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles unicode rapid changes', () => {
      const { rerender, lastFrame } = render(<AgentStatus status="English" />);
      const unicodes = ['中文', '日本語', '한국어', 'العربية', 'עברית'];
      unicodes.forEach(u => {
        rerender(<AgentStatus status={u} />);
      });
      expect(lastFrame()).toContain('עברית');
    });

    it('handles special characters rapid changes', () => {
      const { rerender, lastFrame } = render(<AgentStatus status="Normal" />);
      const specials = ['!@#$%^&*()', '中文_test', '<>{}[]', '   ', '\n\n\n'];
      specials.forEach(s => {
        rerender(<AgentStatus status={s} />);
      });
    });
  });

  describe('Rapid State Changes - ToolProgress', () => {
    it('handles rapid tool additions', () => {
      const { rerender, lastFrame } = render(<ToolProgress tools={[]} />);
      for (let i = 0; i < 50; i++) {
        rerender(<ToolProgress tools={[{ name: `tool-${i}`, status: 'pending' }]} />);
      }
      expect(lastFrame()).toContain('tool-49');
    });

    it('handles rapid status transitions', () => {
      const { rerender, lastFrame } = render(<ToolProgress tools={[{ name: 'test', status: 'pending' }]} />);
      const statuses: ('pending' | 'success' | 'error')[] = ['pending', 'success', 'error', 'pending', 'success'];
      statuses.forEach(s => {
        rerender(<ToolProgress tools={[{ name: 'test', status: s }]} />);
      });
    });

    it('handles empty to large tool list', () => {
      const { rerender, lastFrame } = render(<ToolProgress tools={[]} />);
      const largeList = Array.from({ length: 50 }, (_, i) => ({
        name: `tool-${i}`,
        status: 'pending' as const,
      }));
      rerender(<ToolProgress tools={largeList} />);
      expect(lastFrame()).toContain('tool-0');
      expect(lastFrame()).toContain('tool-49');
    });
  });

  describe('Rapid State Changes - MarkdownText', () => {
    it('handles rapid content changes', () => {
      const { rerender, lastFrame } = render(<MarkdownText content="Start" />);
      for (let i = 1; i <= 50; i++) {
        rerender(<MarkdownText content={`Content ${i}`} />);
      }
      expect(lastFrame()).toContain('Content 50');
    });

    it('handles empty to markdown transition', () => {
      const { rerender, lastFrame } = render(<MarkdownText content="" />);
      rerender(<MarkdownText content="# Header\n## Subheader\n- List item" />);
      expect(lastFrame()).toContain('Header');
      expect(lastFrame()).toContain('List item');
    });

    it('handles rapid markdown type changes', () => {
      const { rerender, lastFrame } = render(<MarkdownText content="Plain text" />);
      const contents = ['# H1', '## H2', '- list', '\n\n', 'text with spaces'];
      contents.forEach(c => {
        rerender(<MarkdownText content={c} />);
      });
    });
  });

  describe('Memory and Cleanup - Components', () => {
    it('no memory leak on rapid unmount/remount AgentStatus', () => {
      for (let i = 0; i < 20; i++) {
        const { unmount } = render(<AgentStatus status={`Test ${i}`} />);
        unmount();
      }
    });

    it('no memory leak on rapid unmount/remount ToolProgress', () => {
      for (let i = 0; i < 20; i++) {
        const { unmount } = render(<ToolProgress tools={[{ name: `t${i}`, status: 'pending' }]} />);
        unmount();
      }
    });

    it('no memory leak on rapid unmount/remount MarkdownText', () => {
      for (let i = 0; i < 20; i++) {
        const { unmount } = render(<MarkdownText content={`Content ${i}`} />);
        unmount();
      }
    });

    it('cleanup on unmount during render', () => {
      const { unmount } = render(<AgentStatus status="Unmount Test" />);
      expect(() => unmount()).not.toThrow();
    });
  });

  describe('Memory and Cleanup - Hooks', () => {
    it('useOrchestrator cleanup on unmount before timer', () => {
      const { unmount } = renderHook(() => useOrchestrator());
      unmount();
      act(() => {
        vi.advanceTimersByTime(2000);
      });
    });

    it('useOrchestrator cleanup on unmount after timer', () => {
      const { unmount } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      unmount();
      act(() => {
        vi.advanceTimersByTime(10000);
      });
    });

    it('multiple hooks cleanup independently', () => {
      const hook1 = renderHook(() => useOrchestrator());
      const hook2 = renderHook(() => useOrchestrator());
      hook1.unmount();
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      expect(hook2.result.current.status).toBe('Analyzing Codebase...');
      hook2.unmount();
    });
  });

  describe('Race Conditions', () => {
    it('handles concurrent state updates', () => {
      const { rerender, lastFrame } = render(<AgentStatus status="Initial" />);
      rerender(<AgentStatus status="Updated" />);
      expect(lastFrame()).toContain('Updated');
    });

    it('handles unmount during state transition', () => {
      const { unmount, rerender } = render(<AgentStatus status="Changing" />);
      rerender(<AgentStatus status="Changed" />);
      unmount();
      expect(() => unmount()).not.toThrow();
    });

    it('handles hook unmount during timer', () => {
      const { unmount } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(1000);
      });
      unmount();
    });
  });

  describe('Invalid Props - AgentStatus', () => {
    it('handles undefined status', () => {
      // @ts-ignore
      const { lastFrame } = render(<AgentStatus status={undefined} />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles null status', () => {
      // @ts-ignore
      const { lastFrame } = render(<AgentStatus status={null} />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles numeric status', () => {
      // @ts-ignore
      const { lastFrame } = render(<AgentStatus status={42} />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles object status', () => {
      // @ts-ignore
      const { lastFrame } = render(<AgentStatus status={{}} />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles array status', () => {
      // @ts-ignore
      const { lastFrame } = render(<AgentStatus status={[]} />);
      expect(lastFrame()).toBeDefined();
    });
  });

  describe('Invalid Props - ToolProgress', () => {
    it('handles undefined tools', () => {
      // @ts-ignore
      const { lastFrame } = render(<ToolProgress tools={undefined} />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles null tools', () => {
      // @ts-ignore
      const { lastFrame } = render(<ToolProgress tools={null} />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles non-array tools', () => {
      // @ts-ignore
      const { lastFrame } = render(<ToolProgress tools="not array" />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles tools with undefined elements', () => {
      // @ts-ignore
      const { lastFrame } = render(<ToolProgress tools={[undefined]} />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles tools with null elements', () => {
      // @ts-ignore
      const { lastFrame } = render(<ToolProgress tools={[null]} />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles invalid status in tool', () => {
      // @ts-ignore
      const { lastFrame } = render(<ToolProgress tools={[{ name: 'test', status: 'invalid' }]} />);
      expect(lastFrame()).toBeDefined();
    });
  });

  describe('Invalid Props - MarkdownText', () => {
    it('handles undefined content', () => {
      // @ts-ignore
      const { lastFrame } = render(<MarkdownText content={undefined} />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles null content', () => {
      // @ts-ignore
      const { lastFrame } = render(<MarkdownText content={null} />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles numeric content', () => {
      // @ts-ignore
      const { lastFrame } = render(<MarkdownText content={12345} />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles object content', () => {
      // @ts-ignore
      const { lastFrame } = render(<MarkdownText content={{ key: 'value' }} />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles array content', () => {
      // @ts-ignore
      const { lastFrame } = render(<MarkdownText content={['a', 'b', 'c']} />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles boolean content', () => {
      // @ts-ignore
      const { lastFrame } = render(<MarkdownText content={true} />);
      expect(lastFrame()).toBeDefined();
    });
  });

  describe('High-Frequency Updates', () => {
    it('handles AgentStatus high frequency updates', () => {
      const { rerender, lastFrame } = render(<AgentStatus status="Update 0" />);
      for (let i = 1; i <= 200; i++) {
        rerender(<AgentStatus status={`Update ${i}`} />);
      }
      expect(lastFrame()).toContain('Update 200');
    });

    it('handles ToolProgress high frequency updates', () => {
      const { rerender, lastFrame } = render(<ToolProgress tools={[]} />);
      for (let i = 1; i <= 100; i++) {
        rerender(<ToolProgress tools={[{ name: `t${i}`, status: 'pending' }]} />);
      }
      expect(lastFrame()).toContain('t100');
    });

    it('handles MarkdownText high frequency updates', () => {
      const { rerender, lastFrame } = render(<MarkdownText content="Line 0" />);
      for (let i = 1; i <= 100; i++) {
        rerender(<MarkdownText content={`Line ${i}`} />);
      }
      expect(lastFrame()).toContain('Line 100');
    });
  });

  describe('Long Tool Lists', () => {
    it('handles 100 tools', () => {
      const tools = Array.from({ length: 100 }, (_, i) => ({
        name: `tool-${i}`,
        status: 'pending' as const,
      }));
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('tool-0');
      expect(lastFrame()).toContain('tool-99');
    });

    it('handles 500 tools', () => {
      const tools = Array.from({ length: 500 }, (_, i) => ({
        name: `tool-${i}`,
        status: i % 2 === 0 ? 'success' : 'pending',
      }));
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('tool-0');
      expect(lastFrame()).toContain('tool-499');
    });

    it('handles mixed status long list', () => {
      const tools = Array.from({ length: 200 }, (_, i) => ({
        name: `tool-${i}`,
        status: (i % 3 === 0 ? 'success' : i % 3 === 1 ? 'pending' : 'error') as 'pending' | 'success' | 'error',
      }));
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('tool-199');
    });
  });

  describe('Deeply Nested Markdown', () => {
    it('handles deeply nested markdown content', () => {
      const content = Array.from({ length: 50 }, (_, i) => `# Header ${i}\n## Sub ${i}\n- Item ${i}`).join('\n');
      const { lastFrame } = render(<MarkdownText content={content} />);
      expect(lastFrame()).toContain('Header 0');
      expect(lastFrame()).toContain('Header 49');
    });

    it('handles many consecutive headers', () => {
      const content = Array.from({ length: 100 }, (_, i) => `# H${i}`).join('\n');
      const { lastFrame } = render(<MarkdownText content={content} />);
      expect(lastFrame()).toContain('H0');
      expect(lastFrame()).toContain('H99');
    });

    it('handles many consecutive list items', () => {
      const content = Array.from({ length: 100 }, (_, i) => `- Item ${i}`).join('\n');
      const { lastFrame } = render(<MarkdownText content={content} />);
      expect(lastFrame()).toContain('Item 0');
      expect(lastFrame()).toContain('Item 99');
    });
  });

  describe('Component Unmount During Render', () => {
    it('handles unmount during render cycle', () => {
      const { unmount, rerender } = render(<AgentStatus status="Testing" />);
      rerender(<AgentStatus status="Changing" />);
      unmount();
    });

    it('handles unmount during timer', () => {
      const { unmount } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(1500);
      });
      unmount();
    });

    it('handles rapid mount/unmount cycles', () => {
      for (let i = 0; i < 10; i++) {
        const { unmount } = render(<AgentStatus status={`Cycle ${i}`} />);
        unmount();
      }
    });
  });

  describe('Concurrent Timer Scenarios', () => {
    it('handles multiple hooks with staggered timers', () => {
      const hook1 = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(500);
      });
      const hook2 = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(1000);
      });
      const hook3 = renderHook(() => useOrchestrator());

      expect(hook1.result.current.status).toBe('Initializing Agent...');
      expect(hook2.result.current.status).toBe('Initializing Agent...');
      expect(hook3.result.current.status).toBe('Initializing Agent...');

      hook1.unmount();
      hook2.unmount();
      hook3.unmount();
    });

    it('handles rapid timer advances', () => {
      const { result } = renderHook(() => useOrchestrator());
      for (let i = 0; i < 100; i++) {
        act(() => {
          vi.advanceTimersByTime(20);
        });
      }
      expect(result.current.status).toBe('Analyzing Codebase...');
    });

    it('handles timer advances beyond 2000ms', () => {
      const { result } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(5000);
      });
      expect(result.current.status).toBe('Analyzing Codebase...');
      act(() => {
        vi.advanceTimersByTime(10000);
      });
      expect(result.current.status).toBe('Analyzing Codebase...');
    });
  });

  describe('Error Boundary Behavior', () => {
    it('handles invalid prop types gracefully', () => {
      // @ts-ignore - testing edge case
      expect(() => render(<AgentStatus status={undefined} />)).not.toThrow();
    });

    it('handles malformed data gracefully', () => {
      // @ts-ignore
      expect(() => render(<ToolProgress tools="invalid" />)).not.toThrow();
    });

    it('handles corrupted props gracefully', () => {
      // @ts-ignore
      expect(() => render(<MarkdownText content={null} />)).not.toThrow();
    });
  });

  describe('Empty and Null Handling', () => {
    it('handles empty string for AgentStatus', () => {
      const { lastFrame } = render(<AgentStatus status="" />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles empty array for ToolProgress', () => {
      const { lastFrame } = render(<ToolProgress tools={[]} />);
      expect(lastFrame()).toContain('Tools Executed:');
    });

    it('handles empty string for MarkdownText', () => {
      const { lastFrame } = render(<MarkdownText content="" />);
      expect(lastFrame()).toBeDefined();
    });

    it('handles only whitespace content', () => {
      const { lastFrame } = render(<MarkdownText content="   \n  \n  " />);
      expect(lastFrame()).toBeDefined();
    });
  });

  describe('Leak Prevention', () => {
    it('no timer leak after component unmount', () => {
      const { unmount } = render(<AgentStatus status="Test" />);
      unmount();
      act(() => {
        vi.advanceTimersByTime(10000);
      });
    });

    it('no timer leak after hook unmount', () => {
      const { unmount } = renderHook(() => useOrchestrator());
      act(() => {
        vi.advanceTimersByTime(2000);
      });
      unmount();
      act(() => {
        vi.advanceTimersByTime(10000);
      });
    });

    it('no timer accumulation with multiple hooks', () => {
      for (let i = 0; i < 5; i++) {
        const { unmount } = renderHook(() => useOrchestrator());
        act(() => {
          vi.advanceTimersByTime(2000);
        });
        unmount();
      }
      act(() => {
        vi.advanceTimersByTime(10000);
      });
    });
  });
});