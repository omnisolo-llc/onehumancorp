1. **Create `srcs/server/domain/invite.go`**
   - Provide the Go backend service `InviteService` handling `CreateInvite` and `AcceptInvite`.
   - Verified via: Code already written and verified via tests.
2. **Create `srcs/server/domain/invite_test.go`**
   - Provide 100% test coverage for `CreateInvite` and `AcceptInvite`.
   - Verified via: Code already written and verified via tests.
3. **Create `srcs/server/api/invite_handler.go`**
   - Add the HTTP handler wrapping the `InviteService` implementation.
   - Verified via: Code already written and verified via tests.
4. **Create `srcs/server/api/invite_handler_test.go`**
   - Provide 100% test coverage for the handlers.
   - Verified via: Code already written and verified via tests.
5. **Update `BUILD.bazel` files**
   - Add the new files to `srcs/server/domain/BUILD.bazel` and `srcs/server/api/BUILD.bazel`.
   - Verified via: Files already updated and bazelisk build passes.
6. **Update `srcs/server/main.go`**
   - Wire up `InviteService` and `InviteHandler` to the HTTP router.
   - Verified via: File already updated and bazelisk build passes.
7. **Complete pre commit steps**
   - Complete pre commit steps to make sure proper testing, verifications, reviews and reflections are done.
8. **Submit the change.**
   - Once all tests pass, submit the change with a descriptive commit message.
