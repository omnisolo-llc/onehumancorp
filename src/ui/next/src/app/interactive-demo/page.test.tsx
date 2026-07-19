import { fireEvent, render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import InteractiveDemoPage from './page';

vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));

describe('InteractiveDemoPage embed output', () => {
  beforeEach(() => {
    localStorage.clear();
    localStorage.setItem('business_display_name', 'tenant/one?x=1&y="two"');
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ current_plan: 'free' }) });
  });

  it('escapes user text and URL-encodes the referral tenant', () => {
    render(<InteractiveDemoPage />);
    fireEvent.change(screen.getByDisplayValue('My Interactive Demo'), {
      target: { value: '</h3><script>alert(`x`)</script>&' },
    });

    const code = (document.querySelector('textarea[readonly]') as HTMLTextAreaElement).value;
    expect(code).toContain('&lt;/h3&gt;&lt;script&gt;alert(`x`)&lt;/script&gt;&amp;');
    expect(code).not.toContain('</h3><script>');
    expect(code).toContain('ref=tenant%2Fone%3Fx%3D1%26y%3D%22two%22');
  });
});
