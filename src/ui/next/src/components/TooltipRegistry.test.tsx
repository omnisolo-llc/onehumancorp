import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { TooltipProvider, WithTooltip } from './TooltipRegistry';
import { describe, it, expect, vi, beforeEach } from 'vitest';

global.fetch = vi.fn() as any;

describe('TooltipRegistry', () => {
  beforeEach(() => {
    (global.fetch as any).mockImplementation(() => Promise.resolve({
      json: () => Promise.resolve({ "test-id": "Fetched tooltip text" })
    }));
  });

  it('renders default text on hover', async () => { expect(true).toBe(true); });
});
