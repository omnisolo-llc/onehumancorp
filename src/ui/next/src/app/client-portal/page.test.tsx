import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, test, vi } from "vitest";
import ClientPortalPage from "./page";

function response(body: unknown, status = 200): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "content-type": "application/json" },
  });
}

describe("ClientPortalPage", () => {
  afterEach(() => vi.restoreAllMocks());

  test("renders data returned by the real billing and subscription API contracts", async () => {
    vi.spyOn(globalThis, "fetch").mockImplementation(async (input) => {
      const url = String(input);
      if (url.endsWith("/api/v1/billing/my-plan")) {
        return response({
          current_plan: "Starter",
          ai_actions_used: 27,
          ai_actions_limit: 1000,
          next_bill_estimated: 2900,
        });
      }
      if (url.endsWith("/api/v1/subscriptions")) {
        return response({
          plans: [
            { id: "plan-1", name: "Monthly delivery", active: true },
            { id: "plan-2", name: "Paused delivery", active: false },
          ],
          subscribers: [],
          batches: [],
        });
      }
      return response({}, 404);
    });

    const { container } = render(<ClientPortalPage />);

    expect(await screen.findByText("Starter")).toBeDefined();
    expect(container.querySelector('[data-client-portal-state="settled"]')).toBeTruthy();
    expect(screen.getByText("27")).toBeDefined();
    expect(screen.getByText("$29.00")).toBeDefined();
    expect(screen.getByText("Quotes and proposals").closest("a")).toHaveAttribute(
      "href",
      "/quoting",
    );
  });

  test("fails honestly when both backend resources are unavailable and can retry", async () => {
    const fetch = vi.spyOn(globalThis, "fetch").mockResolvedValue(response({}, 503));
    render(<ClientPortalPage />);

    expect(await screen.findByRole("alert")).toHaveTextContent(
      "Client data is currently unavailable.",
    );
    const callsBeforeRefresh = fetch.mock.calls.length;
    fireEvent.click(screen.getByRole("button", { name: "Refresh" }));
    await waitFor(() =>
      expect(fetch.mock.calls.length).toBeGreaterThanOrEqual(callsBeforeRefresh + 2),
    );
  });
});
