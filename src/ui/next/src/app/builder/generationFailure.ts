/** Only known server error codes become actionable provider messages. Never expose raw provider bodies. */
export function generationFailureMessage(status: number, payload: unknown): string {
  const code = payload && typeof payload === 'object' && 'code' in payload ? payload.code : undefined;
  if (status === 503 && code === 'generation_unavailable') return 'Configure the builder operator tenant and an authorized text-generation provider before generating a draft.';
  if (status === 409 && code === 'generation_budget_unavailable') return 'The authorized usage budget cannot cover this draft. No provider request was sent.';
  if (status === 502 && code === 'generation_outcome_unknown') return 'The provider did not confirm a complete draft. No automatic retry was sent. Your existing draft was kept.';
  if (status === 403 && code === 'provider_tenant_forbidden') return 'The configured generation provider is not authorized for this business.';
  return 'The generated draft could not be confirmed. Your existing draft was kept.';
}
