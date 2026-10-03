import { act, render, screen, fireEvent, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import ExitIntentBuilder from "./page";
import { vi } from "vitest";
import { invalidateQueueOwner } from '@/lib/sync/queueIdentity';

describe("ExitIntentBuilder", () => {
  beforeEach(() => {
    localStorage.clear(); act(() => invalidateQueueOwner());
    global.fetch = vi.fn(async url => url === '/api/v1/auth/session-identity'
      ? Response.json({ userId: 'exit-owner', tenantId: 'exit-tenant', expiresAt: Date.now() + 60_000 })
      : Response.json({ current_plan: 'Free' }));
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn(),
      },
    });
  });

  it('encodes hostile editor text as inert JavaScript strings without innerHTML', () => {
    render(<ExitIntentBuilder />);
    fireEvent.change(screen.getByPlaceholderText('Wait! Before you go...'), { target: { value: '</script><img src=x onerror=alert(1)>` ${evil}' } });

    const code = screen.getByText((_, element) => element?.tagName === 'CODE').textContent ?? '';
    expect(code).not.toContain('</script><img');
    expect(code).not.toContain('innerHTML');
    expect(code).toContain('\\u003c/script\\u003e');
    expect(code).toContain('textContent');
  });

  it("renders the builder and preview properly", () => {
    render(<ExitIntentBuilder />);

    expect(screen.getByText("Exit-Intent Pop-up Builder")).toBeInTheDocument();
    expect(screen.getByDisplayValue("Wait! Before you go...")).toBeInTheDocument();

    // Check preview updates
    const headlineInput = screen.getByDisplayValue("Wait! Before you go...");
    fireEvent.change(headlineInput, { target: { value: "Don't leave yet!" } });

    // The preview should reflect this change (it shows up twice: input and preview)
    const texts = screen.getAllByText("Don't leave yet!");
    expect(texts.length).toBeGreaterThan(0);
  });

  it("routes to pricing without granting branding removal locally", async () => {
    render(<ExitIntentBuilder />);

    const toggleButton = screen.getByRole("switch");
    await waitFor(() => expect(toggleButton).toBeEnabled());
    await userEvent.click(toggleButton);

    expect(screen.getAllByText("Remove OmniSolo Branding").length).toBeGreaterThan(0);
    expect(screen.getByRole('link', { name: 'Review plans' })).toHaveAttribute('href', '/pricing');

    await userEvent.click(screen.getByRole('button', { name: 'Cancel' }));

    expect(screen.queryByRole('link', { name: 'Review plans' })).not.toBeInTheDocument();
    expect(toggleButton).toHaveAttribute("aria-checked", "false");
    expect(localStorage.getItem('has_pro')).toBeNull();
  });
});
