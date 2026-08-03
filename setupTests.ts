import '@testing-library/jest-dom';
import '@testing-library/jest-dom/vitest';
import { expect } from 'vitest';
import * as matchers from '@testing-library/jest-dom/matchers';
expect.extend(matchers);

if (typeof window !== "undefined") {
  window.HTMLElement.prototype.scrollIntoView = function () {};
}
