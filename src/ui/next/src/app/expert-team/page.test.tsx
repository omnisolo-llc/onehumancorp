import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import ExpertTeamPage from "./page";
import { vi, describe, it, expect } from "vitest";

global.fetch = vi.fn();

describe("ExpertTeamPage", () => {
  it("renders correctly", () => {
    render(<ExpertTeamPage />);
    expect(screen.getByText("Collaborative Expert Team")).toBeInTheDocument();
  });

  it("handles valid execution", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ result: "Final Expert Synthesis Output" }),
    });

    render(<ExpertTeamPage />);

    // Set text to enable the button
    const textareas = screen.getAllByRole("textbox");
    fireEvent.change(textareas[0], {
      target: { value: 'Analyze new trends' }
    });

    // Click button
    const buttons = screen.getAllByRole("button");
    const runBtn = buttons.find(b => b.textContent?.includes("Execute Task via Expert Team"));
    if (runBtn) {
        fireEvent.click(runBtn);
    }

    await waitFor(() => {
      expect(screen.getByText(/Final Delivered Output/i)).toBeInTheDocument();
    });
  });

  it("handles validation error execution", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      json: async () => ({ error: "Pre-flight failed" }),
    });

    render(<ExpertTeamPage />);

    // Set text to enable the button
    const textareas = screen.getAllByRole("textbox");
    fireEvent.change(textareas[0], {
      target: { value: 'Analyze new trends' }
    });

    // Click button
    const buttons = screen.getAllByRole("button");
    const runBtn = buttons.find(b => b.textContent?.includes("Execute Task via Expert Team"));

    if (runBtn) {
        fireEvent.click(runBtn);
    }

    await waitFor(() => {
      expect(screen.getByText(/Pre-flight failed/i)).toBeInTheDocument();
    });
  });
});
