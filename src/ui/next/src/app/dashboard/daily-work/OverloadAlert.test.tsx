import { render, screen, fireEvent } from "@testing-library/react";
import { OverloadAlert } from "./OverloadAlert";
import { expect, test, vi } from "vitest";

test("OverloadAlert shows when percentage > 100", () => {
  const onMitigate = vi.fn();
  render(<OverloadAlert percentage={120} timeSlot="12PM" onMitigate={onMitigate} />);

  expect(screen.getByText("Capacity Overload: 120%")).toBeDefined();

  const btn = screen.getByText("Mitigate Load");
  fireEvent.click(btn);
  expect(onMitigate).toHaveBeenCalled();
});

test("OverloadAlert hidden when percentage < 100", () => {
  const { container } = render(<OverloadAlert percentage={80} timeSlot="12PM" onMitigate={() => {}} />);
  expect(container.firstChild).toBeNull();
});
