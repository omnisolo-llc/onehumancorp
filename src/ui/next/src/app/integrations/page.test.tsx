import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import Integrations from "./page";
import React from "react";

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
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        authorization_url: "https://oauth.example/shippo",
      }),
    });

    render(React.createElement(Integrations, null));

    // Find the Connect button for Shippo
    const shippoCard = screen.getByText("Shippo").closest("div");
    const connectButton = shippoCard!.querySelector("button");
    fireEvent.click(connectButton!);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith("/api/integrations/shippo/connect", {
        method: "POST",
      });
      expect(assign).toHaveBeenCalledWith("https://oauth.example/shippo");
    });
  });

  it("can connect WhatsApp Cloud API", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        success: true,
      }),
    });

    render(React.createElement(Integrations, null));

    const whatsappCloudCard = screen.getByText("WhatsApp Cloud API").closest("div");
    const connectButton = whatsappCloudCard!.querySelector("button");
    fireEvent.click(connectButton!);

    // Should open modal
    await waitFor(() => {
      expect(screen.getByText("Connect WhatsApp Cloud API")).toBeInTheDocument();
    });

    // Click Continue with Meta
    const continueButton = screen.getByText("Continue with Meta");
    fireEvent.click(continueButton);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith("/api/integrations/whatsapp_cloud_api/connect", expect.objectContaining({
        method: "POST",
      }));
    });
  });
});
