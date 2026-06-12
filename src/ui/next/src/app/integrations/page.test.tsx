import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import Integrations from "./page";

const push = vi.fn();

vi.mock("next/navigation", () => ({
  useRouter: () => ({ push }),
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
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        authorization_url: "https://oauth.example/shippo",
      }),
    });

    render(<Integrations />);

    // Find the Connect button specifically for Shippo
    const shippoHeading = screen.getByText("Shippo");
    const shippoCard = shippoHeading.closest("div.flex.flex-col");
    const connectButton = shippoCard?.querySelector("button");
    if (connectButton) {
        fireEvent.click(connectButton);
    }

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith("/api/integrations/shippo/connect", {
        method: "POST",
      });
      expect(assign).toHaveBeenCalledWith("https://oauth.example/shippo");
    });
  });
});
