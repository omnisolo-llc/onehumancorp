import "@testing-library/jest-dom";
import React from "react";
import { act, render, screen, waitFor } from "@testing-library/react";
vi.mock("next/link", () => ({
  default: (props: import("react").AnchorHTMLAttributes<HTMLAnchorElement>) =>
    React.createElement("a", { href: props.href }, props.children),
}));
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import HelpCenterPage from "./page";
import type { Page } from "@playwright/test";
import { createAuditNavigation } from "../../../../../e2e/support/ui_audit_navigation";
import { AppShell } from "../components/AppShell";
import { TooltipProvider } from "../../components/TooltipRegistry";
import userEvent from "@testing-library/user-event";

describe("HelpCenterPage", () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === "/api/v1/tooltips") {
        return Promise.resolve(
          Response.json({ "test-id": "Tooltip text" }, { status: 200 }),
        );
      }
      if (url === "/api/v1/help") {
        return Promise.resolve(
          Response.json(
            [
              {
                title: "Getting Started",
                desc: "Learn how to easily set up your store and accept your first payment.",
                link: "/help/getting-started-1",
                category: "General",
              },
              {
                title: "Adding Products",
                desc: "Add products, track what's in stock, and change how your store looks.",
                link: "/help/my-store",
                category: "General",
              },
              {
                title: "API Documentation",
                desc: "Advanced.",
                link: "/api-docs",
                category: "Advanced",
              },
            ],
            { status: 200 },
          ),
        );
      }
      if (typeof url === "string" && url.includes("/api/v1/help/search")) {
        const urlObj = new URL("http://localhost" + url);
        const q = urlObj.searchParams.get("q")?.toLowerCase() || "";
        const allArticles = [
          {
            title: "Getting Started",
            desc: "Learn how to easily set up your store and accept your first payment.",
            link: "/help/getting-started-1",
          },
          {
            title: "Adding Products",
            desc: "Add products, track what's in stock, and change how your store looks.",
            link: "/help/my-store",
          },
        ];
        const results = allArticles.filter(
          (a) =>
            a.title.toLowerCase().includes(q) ||
            a.desc.toLowerCase().includes(q),
        );
        return Promise.resolve(Response.json(results, { status: 200 }));
      }
      if (url === "/api/v1/videos") {
        return Promise.resolve(
          Response.json(
            [
              {
                id: 1,
                title: "How to set up your first store easily",
                duration: "1:20",
              },
              {
                id: 2,
                title: "Linking your own website name",
                duration: "0:45",
              },
            ],
            { status: 200 },
          ),
        );
      }
      return Promise.resolve(Response.json([], { status: 200 }));
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it("keeps one page-level heading when rendered inside the product shell", async () => {
    render(
      <TooltipProvider>
        <AppShell title="In-App Help Center"><HelpCenterPage /></AppShell>
      </TooltipProvider>,
    );

    await screen.findByText("Getting Started");
    expect(screen.getAllByRole("heading", { level: 1, name: "In-App Help Center" })).toHaveLength(1);
  });

  it("read navigation preserves Advanced instead of triggering an unrelated search", async () => {
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);
    await screen.findByRole("button", { name: "Advanced" });
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue({ width: 100, height: 40 } as DOMRect);
    const page = { context: () => ({ on: vi.fn() }), goto: async () => ({ status: () => 200 }),
      url: () => "https://fixture.test/help", waitForLoadState: async () => undefined, waitForFunction: async () => ({ dispose: async () => undefined }),
      waitForTimeout: async () => undefined, evaluate: async (callback: () => unknown) => callback() } as unknown as Page;
    try {
      await act(async () => { await createAuditNavigation("https://fixture.test", async () => undefined)(page, "/help"); });
      await act(async () => { await new Promise(resolve => setTimeout(resolve, 400)); });
      expect(screen.getByTestId("help-search-input")).toHaveValue("");
      expect(screen.getByRole("button", { name: "Advanced" })).toBeVisible();
      expect(vi.mocked(fetch).mock.calls.some(([url]) => String(url).includes("/help/search"))).toBe(false);
    } finally { bounds.mockRestore(); }
  });

  it("renders articles loaded from API", async () => {
    render(
      <TooltipProvider>
        <HelpCenterPage />
      </TooltipProvider>,
    );

    expect(screen.getByText("In-App Help Center")).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText("Getting Started")).toBeInTheDocument();
      expect(screen.getByText("Adding Products")).toBeInTheDocument();
    });
  });

  it("filters articles based on search query", async () => {
    const user = userEvent.setup();
    render(
      <TooltipProvider>
        <HelpCenterPage />
      </TooltipProvider>,
    );

    await waitFor(() => {
      expect(screen.getByText("Getting Started")).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText(
      "Search for help articles and videos...",
    );
    await user.type(searchInput, "products");

    await waitFor(() => {
      expect(screen.queryByText("Getting Started")).not.toBeInTheDocument();
      expect(screen.getByText("Adding Products")).toBeInTheDocument();
    });
  });

  it.each([200, 503])("keeps current search results when the initial article response arrives late with HTTP %i", async status => {
    const fetchImmediately = global.fetch;
    const initialResponse = status === 200
      ? await fetchImmediately("/api/v1/help")
      : Response.json({ error: "Unavailable" }, { status });
    let releaseInitial!: (response: Response) => void;
    global.fetch = vi.fn((...args: Parameters<typeof fetch>) => args[0] === "/api/v1/help"
      ? new Promise<Response>(resolve => { releaseInitial = resolve; })
      : fetchImmediately(...args));
    const user = userEvent.setup();
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);
    const search = screen.getByTestId("help-search-input");
    await user.type(search, "products");
    await screen.findByText("Adding Products");
    expect(screen.queryByText("Getting Started")).not.toBeInTheDocument();

    await act(async () => { releaseInitial(initialResponse); });
    expect(search).toHaveValue("products");
    expect(screen.getByText("Adding Products")).toBeInTheDocument();
    expect(screen.queryByText("Getting Started")).not.toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Advanced" })).not.toBeInTheDocument();
    expect(screen.queryByText("Something went wrong loading the help center.")).not.toBeInTheDocument();
  });

  it("still displays an error for the active article request", async () => {
    const fetchImmediately = global.fetch;
    global.fetch = vi.fn((...args: Parameters<typeof fetch>) => args[0] === "/api/v1/help"
      ? Promise.resolve(Response.json({ error: "Unavailable" }, { status: 503 }))
      : fetchImmediately(...args));
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);
    expect(await screen.findByText("Something went wrong loading the help center.")).toBeInTheDocument();
  });

  it("displays no matching articles message when search fails", async () => {
    const user = userEvent.setup();
    render(
      <TooltipProvider>
        <HelpCenterPage />
      </TooltipProvider>,
    );

    await waitFor(() => {
      expect(screen.getByText("Getting Started")).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText(
      "Search for help articles and videos...",
    );
    await user.type(searchInput, "nonexistentxyz123");

    await waitFor(() => {
      expect(screen.getByText(/No results found matching/)).toBeInTheDocument();
      expect(screen.getByText(/"nonexistentxyz123"/)).toBeInTheDocument();
    });
  });

  it("renders video tutorials loaded from API", async () => {
    render(
      <TooltipProvider>
        <HelpCenterPage />
      </TooltipProvider>,
    );

    await waitFor(() => {
      expect(
        screen.getByText("How to set up your first store easily"),
      ).toBeInTheDocument();
      expect(
        screen.getByText("Linking your own website name"),
      ).toBeInTheDocument();
    });
  });

  it("opens and closes the video modal", async () => {
    const user = userEvent.setup();
    render(
      <TooltipProvider>
        <HelpCenterPage />
      </TooltipProvider>,
    );
    await waitFor(() => {
      expect(
        screen.getByText("How to set up your first store easily"),
      ).toBeInTheDocument();
    });
    // The video card container has changed in VideoTutorialList
    const videoTitle = screen.getByText(
      "How to set up your first store easily",
    );
    const videoCard = videoTitle.parentElement?.parentElement;
    if (videoCard) {
      await user.click(videoCard);
    }
    await waitFor(() => {
      expect(screen.getByLabelText("Close video")).toBeInTheDocument();
    });
    const closeBtn = screen.getByLabelText("Close video");
    await user.click(closeBtn);
    await waitFor(() => {
      expect(screen.queryByLabelText("Close video")).not.toBeInTheDocument();
    });
  });

  it("renders correctly when there are no matching results at all", async () => {
    const user = userEvent.setup();
    render(
      <TooltipProvider>
        <HelpCenterPage />
      </TooltipProvider>,
    );
    await waitFor(() => {
      expect(screen.getByText("Getting Started")).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText(
      "Search for help articles and videos...",
    );
    await user.type(searchInput, "nonexistentxyz123");

    await waitFor(() => {
      expect(screen.getByText(/No results found matching/)).toBeInTheDocument();
      expect(screen.queryByText("Video Tutorials")).not.toBeInTheDocument();
    });
  });
});
