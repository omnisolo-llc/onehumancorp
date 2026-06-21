I'm using the writing-plans skill to create the implementation plan.
## Plan Details
1. **Remove Fallbacks**: Update `/api/help/route.ts`, `/api/videos/route.ts`, `/api/tooltips/route.ts`, `/api/changelog/route.ts` and `/api/api-docs-spec/route.ts` in `src/ui/next/src/app` to return empty array/object (`[]` or `{}`) instead of mock data when the backend is unavailable or returns an error. Also remove `fallback.ts`.
2. **Update Tests**: Modify E2E tests (`documentation_cuj.spec.ts`, `help.spec.ts`) to mock backend API responses using `page.route` to return required test data. Ensure tests still verify UI functionality.
3. **Run Prettier**: Ensure formatting is correct using `npx prettier --write` on the modified files.
4. **Test**: Run full test suite using Bazel to ensure zero regressions.
