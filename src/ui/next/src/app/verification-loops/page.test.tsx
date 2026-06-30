import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import VerificationLoopsPage from "./page";
import { vi, describe, it, expect } from "vitest";

global.fetch = vi.fn();

describe("VerificationLoopsPage", () => {
  it("renders correctly", () => {
    render(<VerificationLoopsPage />);
    expect(screen.getByText("Verification Loops")).toBeInTheDocument();
  });

  it("handles valid execution", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ result: { message: "Verification passed successfully." } }),
    });

    render(<VerificationLoopsPage />);

    // Set text to enable the button
    const textareas = screen.getAllByRole("textbox");
    // the output text area is the second one
    fireEvent.change(textareas[1], {
      target: { value: 'print("hello")' }
    });

    // Click button
    const buttons = screen.getAllByRole("button");
    const runBtn = buttons.find(b => b.textContent?.includes("Run Computational Guide"));
    if (runBtn) {
        fireEvent.click(runBtn);
    }

    await waitFor(() => {
      expect(screen.getByText(/Verification passed successfully\./i)).toBeInTheDocument();
    });
  });

  it("handles validation error execution", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      json: async () => ({ error: "Verification Loop Failed" }),
    });

    render(<VerificationLoopsPage />);

    // Set text to enable the button
    const textareas = screen.getAllByRole("textbox");
    fireEvent.change(textareas[1], {
      target: { value: 'print("hello")' }
    });

    // Click button
    const buttons = screen.getAllByRole("button");
    const runBtn = buttons.find(b => b.textContent?.includes("Run Computational Guide"));

    if (runBtn) {
        fireEvent.click(runBtn);
    }

    await waitFor(() => {
      expect(screen.getByText(/Verification Failed/i)).toBeInTheDocument();
      expect(screen.getByText(/Verification Loop Failed/i)).toBeInTheDocument();
    });
  });
});
