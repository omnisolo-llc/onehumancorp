import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { TooltipProvider } from "@/components/TooltipRegistry";
import Integrations from "./page";

const push = vi.fn();

vi.mock("next/navigation", () => ({
  useRouter: () => ({ push }),
  usePathname: () => "/integrations",
}));

const mockIntersectionObserver = vi.fn();
mockIntersectionObserver.mockReturnValue({
  observe: () => null,
  unobserve: () => null,
  disconnect: () => null
});
window.IntersectionObserver = mockIntersectionObserver as any;

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
      if (url === "/api/tooltips") return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
      return Promise.resolve({
        ok: true,
        json: async () => ({
          authorization_url: "https://oauth.example/shippo",
        }),
      });
    });

    render(
      <TooltipProvider>
        <Integrations />
      </TooltipProvider>
    );
    fireEvent.click(screen.getAllByRole("button", { name: "Connect" })[4]);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith("/api/integrations/shippo/connect", {
        method: "POST",
      });
      expect(assign).toHaveBeenCalledWith("https://oauth.example/shippo");
    });
  });
});
