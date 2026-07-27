import { act } from "react";
import "@testing-library/jest-dom/vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import Integrations from "./page";

const push = vi.fn();

vi.mock("next/navigation", () => ({
  useRouter: () => ({ push }),
  usePathname: () => "/integrations",
}));

vi.mock('../../components/TooltipRegistry', () => ({
  TooltipProvider: ({ children }: any) => children,
  WithTooltip: ({ children }: any) => children,
}));

describe("Integrations", () => {
  beforeEach(() => {
    global.fetch = vi.fn();
    push.mockClear();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("does not call an unimplemented OAuth contract or mark it connected", async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/v1/integrations') return Promise.resolve({ ok: true, json: async () => ({ success: true, integrations: [] }) });
      return Promise.resolve({ ok: false, json: async () => ({}) });
    });

    act(() => { render(<Integrations />); });
    await waitFor(() => expect(global.fetch).toHaveBeenCalledWith('/api/v1/integrations'));
    fireEvent.click(screen.getAllByRole('button', { name: 'Connect' })[0]);

    expect(await screen.findByText('Ayrshare connection is unavailable until secure provider verification is configured.')).toBeDefined();
    expect(global.fetch).not.toHaveBeenCalledWith('/api/v1/integrations/ayrshare/connect', expect.anything());
    expect(screen.getByText('Ayrshare').closest('div')).not.toHaveTextContent('Connected');
  });

  it('requires Twilio credentials and explicit backend connection confirmation', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/v1/integrations') return Promise.resolve({ ok: true, json: async () => ({ success: true, integrations: [] }) });
      if (url === '/api/v1/integrations/twilio/connect') return Promise.resolve({ ok: true, json: async () => ({ success: true, status: 'pending' }) });
      return Promise.resolve({ ok: false, json: async () => ({}) });
    });
    act(() => { render(<Integrations />); });
    await waitFor(() => expect(global.fetch).toHaveBeenCalledWith('/api/v1/integrations'));
    fireEvent.click(screen.getAllByRole('button', { name: 'Connect' })[6]);

    const connect = screen.getByRole('button', { name: 'Connect Twilio' });
    expect(connect).toBeDisabled();
    fireEvent.change(screen.getByLabelText('Twilio Account SID'), { target: { value: 'AC123' } });
    fireEvent.change(screen.getByLabelText('Twilio Auth Token'), { target: { value: 'secret' } });
    fireEvent.click(connect);

    await waitFor(() => expect(global.fetch).toHaveBeenCalledWith('/api/v1/integrations/twilio/connect', expect.objectContaining({
      method: 'POST',
      body: expect.stringContaining('"bot_token":"AC123"'),
    })));
    expect(await screen.findByText('Twilio Conversations connection could not be confirmed.')).toBeDefined();
    expect(screen.getByText('Twilio Conversations').closest('div')).not.toHaveTextContent('Connected');
  });

  it('marks Twilio connected only when the backend confirms it is usable', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/v1/integrations') return Promise.resolve({ ok: true, json: async () => ({ success: true, integrations: [] }) });
      if (url === '/api/v1/integrations/twilio/connect') return Promise.resolve({ ok: true, json: async () => ({ success: true, status: 'connected', usable: true }) });
      return Promise.resolve({ ok: false, json: async () => ({}) });
    });
    act(() => { render(<Integrations />); });
    await waitFor(() => expect(global.fetch).toHaveBeenCalledWith('/api/v1/integrations'));
    fireEvent.click(screen.getAllByRole('button', { name: 'Connect' })[6]);
    fireEvent.change(screen.getByLabelText('Twilio Account SID'), { target: { value: 'AC123' } });
    fireEvent.change(screen.getByLabelText('Twilio Auth Token'), { target: { value: 'secret' } });
    fireEvent.click(screen.getByRole('button', { name: 'Connect Twilio' }));

    expect(await screen.findByText('Twilio Conversations connected.')).toBeDefined();
    expect(push).toHaveBeenCalledWith('/inbox');
  });
});
