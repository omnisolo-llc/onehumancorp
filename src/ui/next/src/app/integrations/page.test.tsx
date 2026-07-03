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

  it("uses a backend-generated OAuth URL when connecting an integration", async () => {
    const assign = vi.fn();
    Object.defineProperty(window, "location", {
      configurable: true,
      value: { assign },
    });
    (global.fetch as any).mockImplementation((url: string) => {
        if (url === '/api/integrations') {
            return Promise.resolve({
                ok: true,
                json: async () => ({
                    success: true,
                    integrations: []
                })
            });
        }
        if (url === '/api/integrations/shippo/connect') {
            return Promise.resolve({
                ok: true,
                json: async () => ({
                    authorization_url: "https://oauth.example/shippo",
                })
            });
        }
        return Promise.resolve({ ok: false });
    });

    render(<Integrations />);

    // Wait for initial load fetch to complete
    await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith("/api/integrations");
    });

    const connectButtons = screen.getAllByRole("button", { name: "Connect" });
    // Ayrshare is index 0, Cal is index 1, MailerLite is index 2, Mercado is 3, Shippo is 4
    fireEvent.click(connectButtons[4]);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith("/api/integrations/shippo/connect", {
        method: "POST",
      });
      expect(assign).toHaveBeenCalledWith("https://oauth.example/shippo");
    });
  });
});
