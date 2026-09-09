import "@testing-library/jest-dom";
import { act, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import { UnifiedAgentFeed } from "./UnifiedAgentFeed";

vi.mock("../utils/offlineQueue", () => ({
  getActions: vi.fn().mockResolvedValue([]),
  enqueueAction: vi.fn(),
  removeAction: vi.fn(),
}));

describe("UnifiedAgentFeed activity surfaces", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
    vi.useRealTimers();
  });

  it("renders activity entries as glass surfaces", async () => {
    render(
      <UnifiedAgentFeed
        initialData={{
          items: [
            {
              id: "activity-1",
              tenant_id: "tenant-1",
              event_source: "operations",
              context_payload: { description: "Completed prep" },
              proposed_action: { message: "Completed prep" },
              lifecycle_state: "APPROVED",
              created_at: "2026-08-10T00:00:00Z",
              updated_at: "2026-08-10T00:01:00Z",
            },
          ],
        }}
      />,
    );

    await userEvent.setup().click(screen.getByRole("button", { name: "Activity Feed" }));

    await waitFor(() => expect(screen.getByText("Completed prep")).toBeVisible());

    expect(screen.getByTestId("activity-feed-entry")).toHaveClass("glassmorphism");
  });

  it("refreshes through authenticated HTTP polling without opening a browser socket", async () => {
    vi.useFakeTimers();
    const fetchMock = vi.fn().mockResolvedValue(
      new Response(JSON.stringify({ items: [] }), { status: 200 }),
    );
    const webSocketMock = vi.fn();
    vi.stubGlobal("fetch", fetchMock);
    vi.stubGlobal("WebSocket", webSocketMock);

    render(
      <UnifiedAgentFeed
        initialData={{
          items: [],
          activity: [],
        }}
      />,
    );

    await act(async () => {
      await vi.advanceTimersByTimeAsync(5_000);
    });

    expect(fetchMock).toHaveBeenCalledWith("/api/v1/agent-feed");
    expect(webSocketMock).not.toHaveBeenCalled();
  });
});
