import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import CheckoutPage from "./page";

const mockPush = vi.fn();
const mockUseSearchParams = vi.fn();

vi.mock("next/navigation", () => ({
  useRouter: () => ({ push: mockPush }),
  useSearchParams: () => mockUseSearchParams(),
}));

vi.mock("../../components/TooltipRegistry", () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>,
}));

vi.mock("../components/PoweredByOHC", () => ({ PoweredByOHC: () => null }));
vi.mock("../components/OneTapReferral", () => ({ OneTapReferral: () => null }));
vi.mock("../components/PostPurchaseShareWidget", () => ({ PostPurchaseShareWidget: () => null }));
vi.mock("../components/ShareAndSaveWidget", () => ({ ShareAndSaveWidget: () => null }));
vi.mock("../../hooks/useSyncGateway", () => ({ useSyncGateway: () => ({ lastMessage: null }) }));

const response = (body: unknown, ok = true, status = ok ? 200 : 500) => ({
  ok,
  status,
  json: async () => body,
});

describe("CheckoutPage", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUseSearchParams.mockReturnValue(new URLSearchParams("product_id=prod-real&quantity=2"));
    global.fetch = vi.fn(async (url) => {
      if (url === "/api/v1/catalog/products") {
        return response([{ id: "prod-real", title: "Seeded Product", price_cents: 1299 }]) as Response;
      }
      return response({}, false) as Response;
    });
  });

  it("loads the selected product and price from the authenticated catalog", async () => {
    render(<CheckoutPage />);

    expect(await screen.findByText("Seeded Product")).toBeDefined();
    expect(screen.getByText("$25.98")).toBeDefined();
    expect(global.fetch).toHaveBeenCalledWith("/api/v1/catalog/products");
    expect(document.body.textContent).not.toContain("Service Deposit");
    expect(document.body.textContent).not.toContain("20% discount");
  });

  it("fails closed without an explicit valid product", async () => {
    mockUseSearchParams.mockReturnValue(new URLSearchParams(""));
    render(<CheckoutPage />);

    expect(await screen.findByRole("alert")).toHaveTextContent("A valid product is required");
    expect(global.fetch).not.toHaveBeenCalledWith("/api/v1/catalog/products");
    expect(screen.queryByRole("button", { name: "Pay" })).toBeNull();
  });

  it("does not trust a success query flag without a verified paid order", async () => {
    mockUseSearchParams.mockReturnValue(new URLSearchParams("success=true&orderId=order-1"));
    global.fetch = vi.fn(async () => response([{ id: "order-1", status: "pending" }]) as Response);
    render(<CheckoutPage />);

    expect(await screen.findByRole("alert")).toHaveTextContent("Payment has not been confirmed");
    expect(screen.queryByText("Order Successful")).toBeNull();
  });

  it("renders success only for the exact order returned as paid", async () => {
    mockUseSearchParams.mockReturnValue(new URLSearchParams("success=true&orderId=order-1"));
    global.fetch = vi.fn(async () => response([{ id: "order-1", status: "paid" }]) as Response);
    render(<CheckoutPage />);

    expect(await screen.findByText("Order Successful")).toBeDefined();
    expect(screen.getByText("Order order-1 has a confirmed payment.")).toBeDefined();
    expect(global.fetch).toHaveBeenCalledWith("/api/v1/ui/orders");
  });

  it("refuses an untrusted checkout redirect", async () => {
    const assign = vi.fn();
    Object.defineProperty(window, "location", { configurable: true, value: { origin: "http://localhost", assign } });
    global.fetch = vi.fn(async (url) => {
      if (url === "/api/v1/catalog/products") {
        return response([{ id: "prod-real", title: "Seeded Product", price_cents: 1299 }]) as Response;
      }
      return response({ checkout_url: "https://attacker.example/collect" }) as Response;
    });
    render(<CheckoutPage />);

    fireEvent.click(await screen.findByRole("button", { name: "Pay" }));
    expect(await screen.findByRole("status")).toHaveTextContent("Checkout is temporarily unavailable");
    expect(assign).not.toHaveBeenCalled();
  });

  it("redirects to a trusted Stripe checkout returned by the backend", async () => {
    const assign = vi.fn();
    Object.defineProperty(window, "location", { configurable: true, value: { origin: "http://localhost", assign } });
    global.fetch = vi.fn(async (url) => {
      if (url === "/api/v1/catalog/products") {
        return response([{ id: "prod-real", title: "Seeded Product", price_cents: 1299 }]) as Response;
      }
      return response({ checkout_url: "https://checkout.stripe.com/c/pay/cs_live_real" }) as Response;
    });
    render(<CheckoutPage />);

    fireEvent.click(await screen.findByRole("button", { name: "Pay" }));
    await waitFor(() => expect(assign).toHaveBeenCalledWith("https://checkout.stripe.com/c/pay/cs_live_real"));
    const call = vi.mocked(global.fetch).mock.calls.find(([url]) => url === "/api/v1/billing/create-checkout-session");
    expect(JSON.parse(String(call?.[1]?.body))).toEqual({
      is_subscription: false,
      product_id: "prod-real",
      quantity: 2,
    });
  });
});
