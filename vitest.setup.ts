import '@testing-library/jest-dom';

const originalError = console.error;
console.error = (...args) => {
  if (typeof args[0] === 'string' && /Warning.*not wrapped in act/.test(args[0])) {
    return;
  }
  originalError.call(console, ...args);
};
