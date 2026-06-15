import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import Integrations from "./page";
import { TooltipProvider } from "../../components/TooltipRegistry";

const push = vi.fn();

vi.mock("next/navigation", () => ({
  usePathname: () => "/integrations",
  useRouter: () => vi.fn(() => ({ push })),
}));

describe("Integrations", () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.includes('/api/integrations/shippo/connect')) {
        return Promise.resolve({ ok: true, json: async () => ({ authorization_url: "https://oauth.example/shippo" }) });
      }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    }) as any;
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

    render(<TooltipProvider><Integrations /></TooltipProvider>);
    fireEvent.click(screen.getAllByRole("button", { name: "Connect" })[4]);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith("/api/integrations/shippo/connect", {
        method: "POST",
      });
      expect(assign).toHaveBeenCalledWith("https://oauth.example/shippo");
    });
  });
});