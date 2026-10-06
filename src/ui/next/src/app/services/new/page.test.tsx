import { cleanup } from '@testing-library/react';
/* @vitest-environment jsdom */
import { render, screen, waitFor, act } from "@testing-library/react";
import NewServicePage from "./page";
import { beforeEach, describe, it, expect, vi, afterEach } from "vitest";
import "@testing-library/jest-dom/vitest";
import userEvent from "@testing-library/user-event";

const mockRouterPush = vi.hoisted(() => vi.fn());

vi.mock("next/navigation", () => ({
  usePathname: () => "/services/new",
  useRouter: () => ({
    push: mockRouterPush,
    replace: vi.fn(),
    prefetch: vi.fn(),
    back: vi.fn(),
    forward: vi.fn(),
    refresh: vi.fn(),
  }),
}));

describe("NewServicePage", () => {
    const renderNewServicePage = async () => {
    let view: ReturnType<typeof render>;
    await act(async () => {
      view = render(
          <NewServicePage />
      );
    });
    return view;
  };

  beforeEach(() => {
    localStorage.clear();
    vi.clearAllMocks();
    global.fetch = vi.fn().mockResolvedValue({
        ok: true,
        json: async () => ({}),
      });
  });

  afterEach(() => {
    cleanup();
    vi.restoreAllMocks();
  });

  it("renders correctly", async () => {
      await renderNewServicePage();
      expect(screen.getByText("Add Service")).toBeInTheDocument();
  });

  it("auto generates description", async () => {
    const user = userEvent.setup();
    await renderNewServicePage();
    const btn = screen.getByText("✨ Auto-draft");
    await user.click(btn);
    const textArea = screen.getByPlaceholderText("Describe the service...") as HTMLTextAreaElement;
    expect(textArea.value).toBe("A weekly music tutoring session focused on improving technique, music theory, and performance skills. Perfect for students of all levels.");
  });

  it("shows error when title is empty", async () => {
    const user = userEvent.setup();
    await renderNewServicePage();
    const saveBtn = screen.getByText("Save Service");
    await user.click(saveBtn);
    expect(screen.getByText("Enter a service title before saving.")).toBeInTheDocument();
  });

  it("shows billing frequency when recurring is checked", async () => {
      const user = userEvent.setup();
      await renderNewServicePage();
      const checkbox = screen.getByRole("checkbox", { name: "Recurring payment" });
      await user.click(checkbox);
      expect(screen.getByText("Billing Frequency")).toBeInTheDocument();
  });

  it("calls the save API and redirects", async () => {
      vi.useFakeTimers({ shouldAdvanceTime: true });
      const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
      await renderNewServicePage();
      const titleInput = screen.getByPlaceholderText("e.g. Weekly Music Tutoring");
      await user.type(titleInput, "Test Service");
      const saveBtn = screen.getByText("Save Service");
      await user.click(saveBtn);

      expect(global.fetch).toHaveBeenCalledWith("/api/v1/onboarding/state", expect.objectContaining({
          method: "POST",
          body: JSON.stringify({ services: [{ title: "Test Service", description: "", price: "", isRecurring: false, frequency: undefined }] })
      }));

      expect(await screen.findByText("Service Saved!")).toBeInTheDocument();

      await act(async () => {
         vi.advanceTimersByTime(1500);
      });

      expect(mockRouterPush).toHaveBeenCalledWith("/dashboard");
      vi.useRealTimers();
  });
  it("shows error when save fails", async () => {
      vi.useFakeTimers({ shouldAdvanceTime: true });
      global.fetch = vi.fn().mockResolvedValue({
        ok: false,
        status: 500
      });
      const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
      await renderNewServicePage();
      const titleInput = screen.getByPlaceholderText("e.g. Weekly Music Tutoring");
      await user.type(titleInput, "Test Service");
      const saveBtn = screen.getByText("Save Service");
      await user.click(saveBtn);

      expect(await screen.findByText("Failed to save service. Please try again.")).toBeInTheDocument();
      expect(screen.queryByText("Service Saved!")).not.toBeInTheDocument();
      vi.useRealTimers();
  });
});
