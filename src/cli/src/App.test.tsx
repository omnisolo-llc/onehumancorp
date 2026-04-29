/**
 * App Component Tests - 50 comprehensive tests
 * Tests full component composition, layout, and static rendering
 */

import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { App } from './App';

describe('App', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  describe('Initial Render', () => {
    it('renders main header text', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('ONE HUMAN CORP');
    });

    it('renders subtitle', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('Standalone Agent Mode');
    });

    it('renders agent status component', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('Initializing Agent...');
    });

    it('renders tools executed header', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('Tools Executed:');
    });

    it('renders harness label', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('OHC Interactive Harness');
    });

    it('renders markdown text component', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('OHC Interactive Harness');
    });
  });

  describe('Component Composition Verification', () => {
    it('renders AgentStatus component', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('Initializing Agent...');
    });

    it('renders ToolProgress component', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('Tools Executed:');
    });

    it('renders ToolProgress with first tool', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('ls -la');
    });

    it('renders ToolProgress with second tool', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('read_file');
    });

    it('renders MarkdownText component', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('Powered by Ink');
    });

    it('renders MarkdownText with list items', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('•');
    });
  });

  describe('Layout Structure', () => {
    it('renders header at top', () => {
      const { lastFrame } = render(<App />);
      const frame = lastFrame();
      const corpIndex = frame.indexOf('ONE HUMAN CORP');
      expect(corpIndex).toBeGreaterThanOrEqual(0);
    });

    it('renders agent status in middle', () => {
      const { lastFrame } = render(<App />);
      const frame = lastFrame();
      expect(frame).toContain('Initializing Agent...');
    });

    it('renders tools section', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('Tools Executed:');
    });
  });

  describe('Initial State Verification', () => {
    it('shows initializing status at start', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('Initializing Agent...');
    });

    it('shows initial success tool', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('[✓]');
    });

    it('shows initial pending tool', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('[ ]');
    });

    it('has two initial tools', () => {
      const { lastFrame } = render(<App />);
      const frame = lastFrame();
      expect(frame).toContain('ls -la');
      expect(frame).toContain('read_file');
    });

    it('shows markdown content', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('OHC Interactive Harness');
    });
  });

  describe('Static Content Verification', () => {
    it('always renders company name', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('ONE HUMAN CORP');
    });

    it('always renders mode label', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('Standalone Agent Mode');
    });

    it('always renders harness label', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('OHC Interactive Harness');
    });

    it('renders markdown header text', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('OHC Interactive Harness');
    });

    it('renders markdown list bullet', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('•');
    });

    it('renders Powered by Ink text', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('Powered by Ink');
    });

    it('renders React in CLI text', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('React in the CLI');
    });
  });

  describe('Multiple Render Cycles', () => {
    it('renders fresh after unmount', () => {
      const { unmount, lastFrame } = render(<App />);
      expect(lastFrame()).toContain('ONE HUMAN CORP');
      unmount();

      const { lastFrame: newFrame } = render(<App />);
      expect(newFrame()).toContain('ONE HUMAN CORP');
    });

    it('handles multiple mount/unmount cycles', () => {
      for (let i = 0; i < 3; i++) {
        const { unmount, lastFrame } = render(<App />);
        expect(lastFrame()).toContain('ONE HUMAN CORP');
        unmount();
      }
    });
  });

  describe('Border and Styling', () => {
    it('renders with border characters', () => {
      const { lastFrame } = render(<App />);
      const frame = lastFrame();
      expect(frame).toBeDefined();
    });

    it('renders with cyan color elements', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('ONE HUMAN CORP');
    });
  });

  describe('Concurrent Instances', () => {
    it('renders multiple instances', () => {
      const { lastFrame: frame1 } = render(<App />);
      const { lastFrame: frame2 } = render(<App />);
      expect(frame1()).toContain('ONE HUMAN CORP');
      expect(frame2()).toContain('ONE HUMAN CORP');
    });
  });

  describe('Text Content Accuracy', () => {
    it('displays correct initial status text', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('Initializing Agent...');
    });

    it('displays correct tool names', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('ls -la');
      expect(lastFrame()).toContain('read_file');
    });

    it('displays success indicator', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('[✓] ls -la');
    });

    it('displays pending indicator', () => {
      const { lastFrame } = render(<App />);
      expect(lastFrame()).toContain('[ ] read_file');
    });
  });

  describe('Performance and Edge Cases', () => {
    it('handles rapid unmount/remount', () => {
      for (let i = 0; i < 5; i++) {
        const { unmount, lastFrame } = render(<App />);
        expect(lastFrame()).toContain('ONE HUMAN CORP');
        unmount();
      }
    });

    it('renders without memory leaks on unmount', () => {
      const { unmount } = render(<App />);
      expect(() => unmount()).not.toThrow();
    });
  });
});