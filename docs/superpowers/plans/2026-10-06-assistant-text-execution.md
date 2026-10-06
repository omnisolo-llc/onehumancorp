# Assistant text execution implementation plan

1. Add failing schema, source-bound HTTP, BFF and UI contracts for real admission,
   output readback, idempotent uncertainty, cancellation and independent archive.
2. Add canonical-database metadata/attempt association migration 1042; reuse the
   receipt transaction authority fence rather than maintaining a new auth path.
3. Implement bounded assistant text request adapters and safe attempt association
   before dispatch. Preserve existing legacy history as explicitly unexecuted.
4. Replace client-generated backend status and fabricated running fallbacks with
   truthful text capability, stable request/readback and evidence-driven results.
5. Run available focused schema/Node checks on frozen inputs; retain exact native
   and PostgreSQL contracts for mandatory CI and report all unrun gates.
