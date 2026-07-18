import { fireEvent, render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import DiscountCodeGeneratorPage from './page';

vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));
vi.mock('../components/PoweredByOHC', () => ({ PoweredByOHC: () => null }));
vi.mock('../components/useProPlan', () => ({
  useProPlan: () => ({ hasPro: false, claimTrial: vi.fn(), claimError: null }),
}));

describe('DiscountCodeGeneratorPage', () => {
  beforeEach(() => {
    localStorage.clear();
    localStorage.setItem('business_display_name', '\"><script>globalThis.tenantPwned=true</script>');
  });

  it('encodes every user-controlled value in the copied iframe and referral attributes', async () => {
    const hostile = '\"><script>globalThis.pwned=true</script>&line=one two';
    render(<DiscountCodeGeneratorPage />);

    fireEvent.change(screen.getByPlaceholderText('e.g. 20% or $10'), { target: { value: hostile } });
    fireEvent.change(screen.getByPlaceholderText('e.g. SUMMER20'), { target: { value: hostile } });
    fireEvent.click(screen.getByRole('button', { name: 'Generate Widget Embed' }));

    const snippet = (screen.getAllByRole('textbox').find((element) => element.tagName === 'TEXTAREA') as HTMLTextAreaElement).value;
    expect(snippet).not.toContain('<script>');
    expect(snippet).not.toContain(hostile);
    expect(snippet).toContain(encodeURIComponent(hostile));
    expect(snippet).toContain(encodeURIComponent('\"><script>globalThis.tenantPwned=true</script>'));
  });
});
