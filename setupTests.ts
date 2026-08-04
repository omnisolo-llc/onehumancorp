import '@testing-library/jest-dom';
import '@testing-library/jest-dom/vitest';

if (typeof window !== "undefined") { window.HTMLElement.prototype.scrollIntoView = function () {}; }
