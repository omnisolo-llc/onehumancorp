import { act } from "react";
import "@testing-library/jest-dom/vitest";
import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import DiagnosticsPage from './page';

vi.mock('../components/AppShell', () => ({ AppShell: ({ children }: { children: React.ReactNode }) => <div>{children}</div> }));

describe('DiagnosticsPage', () => {
  it('shows unknown booleans and no canned action success when health fields are missing', async () => {
    global.fetch = vi.fn().mockImplementation((url: string) => Promise.resolve({
      ok: true,
      json: async () => url === '/api/v1/health' ? { status: 'ok' } : { total_sales: 4 },
    }));
    act(() => { render(<DiagnosticsPage />); });
    expect(await screen.findByText('Operational Telemetry')).toBeDefined();
    expect(screen.getByText('Mesh Active:').parentElement).toHaveTextContent('Unknown');
    expect(screen.getByText('Hybrid Mode Ready:').parentElement).toHaveTextContent('Unknown');
    expect(screen.getByTestId('diagnostics-result')).toHaveTextContent('Diagnostics actions are unavailable.');
    expect(document.body.textContent).not.toContain('test result passed');
    expect(document.body.textContent).not.toContain('download ready');
  });
});
