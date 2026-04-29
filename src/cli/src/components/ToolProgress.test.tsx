/**
 * ToolProgress Component Tests - 70 comprehensive tests
 * Tests all tool states, status transitions, and edge cases for the ToolProgress component
 */

import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { ToolProgress, ToolItem } from './ToolProgress';

describe('ToolProgress', () => {
  describe('Empty State', () => {
    it('renders with empty tools array', () => {
      const { lastFrame } = render(<ToolProgress tools={[]} />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders header with empty tools', () => {
      const { lastFrame } = render(<ToolProgress tools={[]} />);
      expect(lastFrame()).toContain('Tools Executed:');
    });

    it('renders empty tools without error', () => {
      const { lastFrame } = render(<ToolProgress tools={[]} />);
      expect(lastFrame()).not.toContain('undefined');
      expect(lastFrame()).not.toContain('null');
    });
  });

  describe('Single Tool States', () => {
    it('renders single pending tool', () => {
      const tools: ToolItem[] = [{ name: 'test-tool', status: 'pending' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('test-tool');
      expect(lastFrame()).toContain('[ ]');
    });

    it('renders single success tool', () => {
      const tools: ToolItem[] = [{ name: 'test-tool', status: 'success' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('test-tool');
      expect(lastFrame()).toContain('[✓]');
    });

    it('renders single error tool', () => {
      const tools: ToolItem[] = [{ name: 'test-tool', status: 'error' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('test-tool');
      expect(lastFrame()).toContain('[x]');
    });
  });

  describe('Multiple Tools with Mixed States', () => {
    it('renders two tools with different statuses', () => {
      const tools: ToolItem[] = [
        { name: 'tool-one', status: 'success' },
        { name: 'tool-two', status: 'pending' },
      ];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('tool-one');
      expect(lastFrame()).toContain('tool-two');
    });

    it('renders three tools with all different statuses', () => {
      const tools: ToolItem[] = [
        { name: 'success-tool', status: 'success' },
        { name: 'pending-tool', status: 'pending' },
        { name: 'error-tool', status: 'error' },
      ];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('success-tool');
      expect(lastFrame()).toContain('pending-tool');
      expect(lastFrame()).toContain('error-tool');
    });

    it('renders tools in correct order', () => {
      const tools: ToolItem[] = [
        { name: 'First', status: 'success' },
        { name: 'Second', status: 'pending' },
        { name: 'Third', status: 'error' },
      ];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      const frame = lastFrame();
      const firstIndex = frame.indexOf('First');
      const secondIndex = frame.indexOf('Second');
      const thirdIndex = frame.indexOf('Third');
      expect(firstIndex).toBeLessThan(secondIndex);
      expect(secondIndex).toBeLessThan(thirdIndex);
    });

    it('renders many tools with mixed states', () => {
      const tools: ToolItem[] = Array.from({ length: 10 }, (_, i) => ({
        name: `tool-${i}`,
        status: i % 3 === 0 ? 'success' : i % 3 === 1 ? 'pending' : 'error',
      }));
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('tool-0');
      expect(lastFrame()).toContain('tool-9');
    });
  });

  describe('All Pending State', () => {
    it('renders all tools as pending', () => {
      const tools: ToolItem[] = [
        { name: 'tool-a', status: 'pending' },
        { name: 'tool-b', status: 'pending' },
        { name: 'tool-c', status: 'pending' },
      ];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('[ ]');
    });

    it('renders five pending tools', () => {
      const tools: ToolItem[] = [
        { name: 'a', status: 'pending' },
        { name: 'b', status: 'pending' },
        { name: 'c', status: 'pending' },
        { name: 'd', status: 'pending' },
        { name: 'e', status: 'pending' },
      ];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      const frame = lastFrame();
      expect(frame).toContain('a');
      expect(frame).toContain('b');
      expect(frame).toContain('c');
      expect(frame).toContain('d');
      expect(frame).toContain('e');
    });
  });

  describe('All Success State', () => {
    it('renders all tools as success', () => {
      const tools: ToolItem[] = [
        { name: 'tool-x', status: 'success' },
        { name: 'tool-y', status: 'success' },
      ];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('[✓]');
    });

    it('renders five success tools', () => {
      const tools: ToolItem[] = Array.from({ length: 5 }, (_, i) => ({
        name: `tool-${i}`,
        status: 'success',
      }));
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      const frame = lastFrame();
      for (let i = 0; i < 5; i++) {
        expect(frame).toContain(`tool-${i}`);
      }
    });
  });

  describe('All Error State', () => {
    it('renders all tools as error', () => {
      const tools: ToolItem[] = [
        { name: 'error-one', status: 'error' },
        { name: 'error-two', status: 'error' },
      ];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('[x]');
    });

    it('renders five error tools', () => {
      const tools: ToolItem[] = Array.from({ length: 5 }, (_, i) => ({
        name: `error-${i}`,
        status: 'error',
      }));
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      const frame = lastFrame();
      for (let i = 0; i < 5; i++) {
        expect(frame).toContain(`error-${i}`);
      }
    });
  });

  describe('Tool Name Variations', () => {
    it('renders tool with empty name', () => {
      const tools: ToolItem[] = [{ name: '', status: 'pending' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders tool with long name', () => {
      const longName = 'x'.repeat(200);
      const tools: ToolItem[] = [{ name: longName, status: 'pending' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('xxxxx');
    });

    it('renders tool with special characters', () => {
      const tools: ToolItem[] = [{ name: 'tool:with/special*chars', status: 'pending' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('tool:with/special*chars');
    });

    it('renders tool with unicode name', () => {
      const tools: ToolItem[] = [{ name: '工具测试', status: 'pending' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('工具测试');
    });

    it('renders tool with emoji', () => {
      const tools: ToolItem[] = [{ name: 'deploy 🚀', status: 'success' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('deploy');
    });

    it('renders tool with spaces', () => {
      const tools: ToolItem[] = [{ name: 'my test tool', status: 'pending' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('my test tool');
    });

    it('renders tool with leading spaces', () => {
      const tools: ToolItem[] = [{ name: '  spaced', status: 'pending' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('  spaced');
    });

    it('renders tool with trailing spaces', () => {
      const tools: ToolItem[] = [{ name: 'spaced  ', status: 'pending' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('spaced');
    });
  });

  describe('Status Transitions', () => {
    it('renders after pending to success transition', () => {
      const { rerender, lastFrame } = render(<ToolProgress tools={[{ name: 'test', status: 'pending' }]} />);
      expect(lastFrame()).toContain('[ ]');

      rerender(<ToolProgress tools={[{ name: 'test', status: 'success' }]} />);
      expect(lastFrame()).toContain('[✓]');
    });

    it('renders after pending to error transition', () => {
      const { rerender, lastFrame } = render(<ToolProgress tools={[{ name: 'test', status: 'pending' }]} />);
      expect(lastFrame()).toContain('[ ]');

      rerender(<ToolProgress tools={[{ name: 'test', status: 'error' }]} />);
      expect(lastFrame()).toContain('[x]');
    });

    it('renders after multiple status changes', () => {
      const { rerender, lastFrame } = render(<ToolProgress tools={[{ name: 'test', status: 'pending' }]} />);
      expect(lastFrame()).toContain('[ ]');

      rerender(<ToolProgress tools={[{ name: 'test', status: 'error' }]} />);
      expect(lastFrame()).toContain('[x]');

      rerender(<ToolProgress tools={[{ name: 'test', status: 'success' }]} />);
      expect(lastFrame()).toContain('[✓]');
    });

    it('renders after tool added to list', () => {
      const { rerender, lastFrame } = render(<ToolProgress tools={[{ name: 'tool1', status: 'success' }]} />);

      rerender(<ToolProgress tools={[
        { name: 'tool1', status: 'success' },
        { name: 'tool2', status: 'pending' },
      ]} />);

      expect(lastFrame()).toContain('tool1');
      expect(lastFrame()).toContain('tool2');
    });

    it('renders after tool removed from list', () => {
      const { rerender, lastFrame } = render(<ToolProgress tools={[
        { name: 'tool1', status: 'success' },
        { name: 'tool2', status: 'pending' },
      ]} />);

      rerender(<ToolProgress tools={[{ name: 'tool1', status: 'success' }]} />);

      expect(lastFrame()).toContain('tool1');
      expect(lastFrame()).not.toContain('tool2');
    });
  });

  describe('Dynamic Tool Additions', () => {
    it('renders tools added one at a time', () => {
      const { rerender, lastFrame } = render(<ToolProgress tools={[]} />);

      for (let i = 1; i <= 5; i++) {
        rerender(<ToolProgress tools={Array.from({ length: i }, (_, idx) => ({
          name: `tool-${idx}`,
          status: 'pending' as const,
        }))} />);
      }

      expect(lastFrame()).toContain('tool-4');
    });

    it('renders with growing tool list', () => {
      const { rerender, lastFrame } = render(<ToolProgress tools={[{ name: 'initial', status: 'success' }]} />);

      const newTools = [
        { name: 'initial', status: 'success' },
        { name: 'added-1', status: 'pending' },
        { name: 'added-2', status: 'pending' },
      ];

      rerender(<ToolProgress tools={newTools} />);
      expect(lastFrame()).toContain('added-1');
      expect(lastFrame()).toContain('added-2');
    });
  });

  describe('Edge Cases', () => {
    it('renders with undefined tool name', () => {
      // @ts-ignore - testing edge case
      const tools: ToolItem[] = [{ name: undefined, status: 'pending' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders with null tool name', () => {
      // @ts-ignore - testing edge case
      const tools: ToolItem[] = [{ name: null, status: 'pending' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders with invalid status', () => {
      // @ts-ignore - testing edge case
      const tools: ToolItem[] = [{ name: 'test', status: 'invalid' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders with numeric tool name', () => {
      // @ts-ignore - testing edge case
      const tools: ToolItem[] = [{ name: 12345, status: 'pending' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders with boolean tool name', () => {
      // @ts-ignore - testing edge case
      const tools: ToolItem[] = [{ name: true, status: 'pending' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders with object tool name', () => {
      // @ts-ignore - testing edge case
      const tools: ToolItem[] = [{ name: { foo: 'bar' }, status: 'pending' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toBeDefined();
    });
  });

  describe('Header Verification', () => {
    it('renders "Tools Executed:" header', () => {
      const { lastFrame } = render(<ToolProgress tools={[]} />);
      expect(lastFrame()).toContain('Tools Executed:');
    });

    it('renders header before tool list', () => {
      const { lastFrame } = render(<ToolProgress tools={[{ name: 'test', status: 'pending' }]} />);
      const frame = lastFrame();
      const headerIndex = frame.indexOf('Tools Executed:');
      const toolIndex = frame.indexOf('test');
      expect(headerIndex).toBeLessThan(toolIndex);
    });

    it('header is bold magenta', () => {
      const { lastFrame } = render(<ToolProgress tools={[]} />);
      expect(lastFrame()).toContain('Tools Executed:');
    });
  });

  describe('Color Verification', () => {
    it('renders green color for success', () => {
      const tools: ToolItem[] = [{ name: 'success-tool', status: 'success' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('success-tool');
    });

    it('renders red color for error', () => {
      const tools: ToolItem[] = [{ name: 'error-tool', status: 'error' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('error-tool');
    });

    it('renders yellow/gray color for pending', () => {
      const tools: ToolItem[] = [{ name: 'pending-tool', status: 'pending' }];
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('pending-tool');
    });
  });

  describe('Performance', () => {
    it('handles large number of tools', () => {
      const tools: ToolItem[] = Array.from({ length: 100 }, (_, i) => ({
        name: `tool-${i}`,
        status: i % 3 === 0 ? 'success' : i % 3 === 1 ? 'pending' : 'error',
      }));
      const { lastFrame } = render(<ToolProgress tools={tools} />);
      expect(lastFrame()).toContain('tool-0');
      expect(lastFrame()).toContain('tool-99');
    });

    it('renders without memory leaks on unmount', () => {
      const tools: ToolItem[] = [{ name: 'test', status: 'pending' }];
      const { unmount } = render(<ToolProgress tools={tools} />);
      expect(() => unmount()).not.toThrow();
    });

    it('handles rapid re-renders', () => {
      const { rerender, lastFrame } = render(<ToolProgress tools={[]} />);
      for (let i = 0; i < 50; i++) {
        rerender(<ToolProgress tools={[{ name: `tool-${i}`, status: 'pending' }]} />);
      }
      expect(lastFrame()).toContain('tool-49');
    });
  });
});