import '@testing-library/jest-dom'
import { vi } from 'vitest'

global.vi = vi

vi.mock('next/link', () => ({
  default: ({ children, href }: any) => {
    return children;
  },
}));
