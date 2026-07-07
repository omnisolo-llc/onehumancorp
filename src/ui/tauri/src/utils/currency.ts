export function formatCurrency(amountCents: number, currencyCode: string): string {
  const amount = amountCents / 100;
  return new Intl.NumberFormat(undefined, {
    style: 'currency',
    currency: currencyCode,
  }).format(amount);
}
