import { render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, test, vi } from "vitest";
import SubscriptionsPage from "./page";

const fetchMock = vi.fn<typeof fetch>();

describe("SubscriptionsPage", () => {
  beforeEach(() => {
    fetchMock.mockReset();
    vi.stubGlobal("fetch", fetchMock);
  });

  test("renders the typed subscription overview", async () => {
    fetchMock.mockResolvedValue(
      Response.json({
        plans: [
          {
            id: "plan-a",
            name: "Monthly essentials",
            description: "Everyday supplies",
            amount: 2500,
            interval: "month",
            active: true,
          },
        ],
        subscribers: [
          {
            id: "subscriber-a",
            customer_id: "customer-a",
            status: "ACTIVE",
            health_score: 90,
          },
        ],
        batches: [
          {
            id: "batch-a",
            fulfillment_date: "2026-08-01",
            status: "PENDING",
            subscriber_count: 3,
          },
        ],
      }),
    );

    render(<SubscriptionsPage />);

    expect(await screen.findByText("$25.00/month")).toBeInTheDocument();
    expect(screen.getByText("Customer #custom")).toBeInTheDocument();
    expect(screen.getByText("Ship on 2026-08-01")).toBeInTheDocument();
    expect(screen.getByText("3 boxes")).toBeInTheDocument();
  });

  test("shows an accessible error for a non-OK response", async () => {
    fetchMock.mockResolvedValue(
      Response.json({ error: "database unavailable" }, { status: 503 }),
    );

    render(<SubscriptionsPage />);

    expect(
      await screen.findByRole("alert", {
        name: "Unable to load subscriptions",
      }),
    ).toHaveTextContent("database unavailable");
  });
});
