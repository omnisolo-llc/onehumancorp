import '@testing-library/jest-dom/vitest';
import { vi } from "vitest";

window.HTMLElement.prototype.scrollIntoView = vi.fn();
global.vi = vi;

vi.mock("next/link", () => ({
  default: ({ children }: any) => children,
}));

vi.mock("next/server", () => ({
  NextResponse: {
    json: (body: any, init?: ResponseInit) => {
      return new Response(JSON.stringify(body), {
        status: init?.status ?? 200,
        headers: {
          "Content-Type": "application/json",
          ...init?.headers,
        },
      });
    },
  },
}));
