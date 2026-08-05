import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import SnapReceiptPage from "./page";

// Mock next/navigation
vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe("SnapReceiptPage", () => {
  it("initializes amount and vendor with empty values", () => {
    render(<SnapReceiptPage />);
    const amountInput = screen.getByTestId("receipt-amount-input");
    const vendorInput = screen.getByTestId("receipt-vendor-input");

    expect((amountInput as HTMLInputElement).value).toBe("");
    expect((vendorInput as HTMLInputElement).value).toBe("");
  });
});
