import { render, screen } from "@testing-library/react";
import AutoQuoteBookPage from "./page";
import { describe, expect, it } from "vitest";

describe("AutoQuoteBookPage", () => {
  it("renders the auto quote and book dashboard", () => {
    // we should provide a mock for any next components or icons if needed
    render(<AutoQuoteBookPage />);
    expect(
      screen.getByText("Auto-Quote & Book Dashboard")
    ).toBeInTheDocument();
  });
});
