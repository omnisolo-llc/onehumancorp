# Execution Plan

1.  **Add `wizard_drafts` to Database Schema**:
    *   Create Goose migration file `src/server/db/migrations/063_wizard_drafts.sql` using:
        ```bash
        cat << 'EOF' > src/server/db/migrations/063_wizard_drafts.sql
        -- +goose Up
        CREATE TABLE wizard_drafts (
            user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
            draft_state TEXT NOT NULL,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        );
        -- +goose StatementBegin
        -- +goose StatementEnd
        -- +goose Down
        DROP TABLE IF EXISTS wizard_drafts;
        EOF
        ```
    *   Create another migration `src/server/db/migrations/064_wizard_drafts_pg.sql` for Postgres RLS:
        ```bash
        cat << 'EOF' > src/server/db/migrations/064_wizard_drafts_pg.sql
        -- +goose Up
        ALTER TABLE wizard_drafts ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_wizard_drafts ON wizard_drafts
            USING (user_id = current_setting('app.current_tenant', true));
        -- +goose StatementBegin
        -- +goose StatementEnd
        -- +goose Down
        DROP POLICY IF EXISTS tenant_isolation_wizard_drafts ON wizard_drafts;
        EOF
        ```
    *   Verify the created files using `cat src/server/db/migrations/063_wizard_drafts.sql` and `cat src/server/db/migrations/064_wizard_drafts_pg.sql`.
    *   (I already ran a python script to update `embedsrcs` in `src/server/db/BUILD.bazel` in a previous trajectory).

2.  **Add Backend Endpoints for Draft State**:
    *   Update `src/server/dashboard/handlers_wizard.go` with `sed`:
        ```bash
        cat << 'EOF' >> src/server/dashboard/handlers_wizard.go

        func (s *Server) handleWizardGetDraft(w http.ResponseWriter, r *http.Request) {
            if r.Method != http.MethodGet {
                http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
                return
            }
            claims := auth.ClaimsFromContext(r.Context())
            if claims == nil || claims.UserID == "" {
                http.Error(w, "unauthorized", http.StatusUnauthorized)
                return
            }

            var draftState string
            err := s.db.QueryRowContext(r.Context(), "SELECT draft_state FROM wizard_drafts WHERE user_id = $1", claims.UserID).Scan(&draftState)
            if err != nil {
                if err == sql.ErrNoRows {
                    w.Header().Set("Content-Type", "application/json")
                    w.Write([]byte("{}"))
                    return
                }
                http.Error(w, "failed to get draft state", http.StatusInternalServerError)
                return
            }

            w.Header().Set("Content-Type", "application/json")
            w.Write([]byte(draftState))
        }

        func (s *Server) handleWizardSaveDraft(w http.ResponseWriter, r *http.Request) {
            if r.Method != http.MethodPost {
                http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
                return
            }
            claims := auth.ClaimsFromContext(r.Context())
            if claims == nil || claims.UserID == "" {
                http.Error(w, "unauthorized", http.StatusUnauthorized)
                return
            }

            bodyBytes, err := io.ReadAll(r.Body)
            if err != nil {
                http.Error(w, "invalid payload", http.StatusBadRequest)
                return
            }

            _, err = s.db.ExecContext(r.Context(), \`
                INSERT INTO wizard_drafts (user_id, draft_state, updated_at)
                VALUES ($1, $2, CURRENT_TIMESTAMP)
                ON CONFLICT (user_id) DO UPDATE SET draft_state = EXCLUDED.draft_state, updated_at = EXCLUDED.updated_at
            \`, claims.UserID, string(bodyBytes))

            if err != nil {
                http.Error(w, "failed to save draft", http.StatusInternalServerError)
                return
            }

            w.WriteHeader(http.StatusOK)
        }
        EOF
        ```
    *   Update `src/server/dashboard/server.go` using a targeted `sed` command:
        ```bash
        sed -i '/mux.HandleFunc("\/api\/wizard\/configure", server.handleWizardConfigure)/a \
        \tmux.HandleFunc("/api/wizard/draft", func(w http.ResponseWriter, r *http.Request) {\
        \t\tif r.Method == http.MethodGet {\
        \t\t\tserver.handleWizardGetDraft(w, r)\
        \t\t} else if r.Method == http.MethodPost {\
        \t\t\tserver.handleWizardSaveDraft(w, r)\
        \t\t}\
        \t})' src/server/dashboard/server.go
        ```
    *   *Verification*: `git diff src/server/dashboard/handlers_wizard.go src/server/dashboard/server.go` to ensure changes were written correctly.

3.  **Update Frontend Business Setup Wizard for Cross-Device Resume**:
    *   Apply modifications to `src/app/lib/screens/business_setup_wizard_screen.dart` via `patch`.
        ```bash
        cat << 'EOF' > patch_frontend.patch
        --- src/app/lib/screens/business_setup_wizard_screen.dart
        +++ src/app/lib/screens/business_setup_wizard_screen.dart
        @@ -67,6 +67,23 @@

         class BusinessSetupNotifier extends Notifier<BusinessSetupState> {
           @override
        -  BusinessSetupState build() => const BusinessSetupState();
        +  BusinessSetupState build() {
        +    _loadDraft();
        +    return const BusinessSetupState();
        +  }
        +
        +  Future<void> _loadDraft() async {
        +    try {
        +      final user = ref.read(authStateProvider).valueOrNull;
        +      final baseUrl = ref.read(backendUrlProvider);
        +      if (user != null && baseUrl.isNotEmpty) {
        +        final res = await http.get(
        +          Uri.parse('$baseUrl/api/wizard/draft'),
        +          headers: {'Authorization': 'Bearer ${user.token}'},
        +        );
        +        if (res.statusCode == 200 && res.body.isNotEmpty && res.body != '{}') {
        +          final data = jsonDecode(res.body);
        +          state = state.copyWith(
        +            step: data['step'] as int?,
        +            businessType: data['businessType'] as String?,
        +            companyName: data['companyName'] as String?,
        +            businessDescription: data['businessDescription'] as String?,
        +            whatYouSell: (data['whatYouSell'] as List<dynamic>?)?.map((e) => e as String).toList(),
        +            paymentMethod: data['paymentMethod'] as String?,
        +            adminName: data['adminName'] as String?,
        +            adminEmail: data['adminEmail'] as String?,
        +          );
        +        }
        +      }
        +    } catch (e) {
        +      // Ignore draft loading errors
        +    }
        +  }
        +
        +  Future<void> _saveDraft() async {
        +    try {
        +      final user = ref.read(authStateProvider).valueOrNull;
        +      final baseUrl = ref.read(backendUrlProvider);
        +      if (user != null && baseUrl.isNotEmpty) {
        +        final body = jsonEncode({
        +          'step': state.step,
        +          'businessType': state.businessType,
        +          'companyName': state.companyName,
        +          'businessDescription': state.businessDescription,
        +          'whatYouSell': state.whatYouSell,
        +          'paymentMethod': state.paymentMethod,
        +          'adminName': state.adminName,
        +          'adminEmail': state.adminEmail,
        +        });
        +        await http.post(
        +          Uri.parse('$baseUrl/api/wizard/draft'),
        +          headers: {
        +            'Authorization': 'Bearer ${user.token}',
        +            'Content-Type': 'application/json',
        +          },
        +          body: body,
        +        );
        +      }
        +    } catch (e) {
        +      // Ignore draft saving errors
        +    }
        +  }

           void nextStep() {
             if (state.step < 6) {
               state = state.copyWith(step: state.step + 1);
        +      _saveDraft();
             }
           }

           void prevStep() {
             if (state.step > 0) {
               state = state.copyWith(step: state.step - 1);
        +      _saveDraft();
             }
           }

           void updateBusinessType(String type) {
             state = state.copyWith(businessType: type);
        +    _saveDraft();
             nextStep();
           }

        -  void updateCompany(String name) => state = state.copyWith(companyName: name);
        -  void updateDescription(String desc) => state = state.copyWith(businessDescription: desc);
        +  void updateCompany(String name) { state = state.copyWith(companyName: name); _saveDraft(); }
        +  void updateDescription(String desc) { state = state.copyWith(businessDescription: desc); _saveDraft(); }

           void toggleWhatYouSell(String item) {
             final list = List<String>.from(state.whatYouSell);
             if (list.contains(item)) {
               list.remove(item);
             } else {
               list.add(item);
             }
             state = state.copyWith(whatYouSell: list);
        +    _saveDraft();
           }

        -  void updatePaymentMethod(String method) => state = state.copyWith(paymentMethod: method);
        +  void updatePaymentMethod(String method) { state = state.copyWith(paymentMethod: method); _saveDraft(); }

        -  void updateAdminName(String name) => state = state.copyWith(adminName: name);
        -  void updateAdminEmail(String val) => state = state.copyWith(adminEmail: val);
        -  void updateAdminPassword(String val) => state = state.copyWith(adminPassword: val);
        +  void updateAdminName(String name) { state = state.copyWith(adminName: name); _saveDraft(); }
        +  void updateAdminEmail(String val) { state = state.copyWith(adminEmail: val); _saveDraft(); }
        +  void updateAdminPassword(String val) { state = state.copyWith(adminPassword: val); _saveDraft(); }

           Future<void> launch(BuildContext context, WidgetRef ref) async {
             final user = ref.read(authStateProvider).valueOrNull;
        @@ -137,6 +187,17 @@
                 if (context.mounted) {
                   context.go('/dashboard');
                 }
        +        // Clear the draft on success
        +        try {
        +           await http.post(
        +              Uri.parse('$baseUrl/api/wizard/draft'),
        +              headers: {
        +                'Authorization': 'Bearer ${user.token}',
        +                'Content-Type': 'application/json',
        +              },
        +              body: '{}',
        +           );
        +        } catch (_) {}
               }
             } catch (e) {
               state = state.copyWith(isLoading: false, errorMessage: e.toString());
        EOF
        patch -p0 < patch_frontend.patch
        ```
    *   *Verification*: `git diff src/app/lib/screens/business_setup_wizard_screen.dart`.

4.  **Add tests**:
    *   Update `src/server/dashboard/handlers_wizard_test.go` with `sed`:
        ```bash
        cat << 'EOF' >> src/server/dashboard/handlers_wizard_test.go
        func TestHandleWizardDraft(t *testing.T) {
		store, _ := db.NewSqliteProvider(":memory:")
		store.RunMigrations(context.Background())

		s := &Server{
			db: store,
		}

		// Insert a test user
		_, err := store.ExecContext(context.Background(), "INSERT INTO users (id, email) VALUES ('user1', 'test@test.com')")
		if err != nil {
			t.Fatalf("Failed to create user: %v", err)
		}

		// Save draft test
		saveReq, _ := http.NewRequest(http.MethodPost, "/api/wizard/draft", bytes.NewBuffer([]byte(\`{"step": 1}\`)))
		saveReq = saveReq.WithContext(auth.NewContextWithClaims(saveReq.Context(), &auth.Claims{UserID: "user1"}))
		saveRr := httptest.NewRecorder()
		s.handleWizardSaveDraft(saveRr, saveReq)

		if saveRr.Code != http.StatusOK {
			t.Errorf("Expected status OK, got %v", saveRr.Code)
		}

		// Get draft test
		getReq, _ := http.NewRequest(http.MethodGet, "/api/wizard/draft", nil)
		getReq = getReq.WithContext(auth.NewContextWithClaims(getReq.Context(), &auth.Claims{UserID: "user1"}))
		getRr := httptest.NewRecorder()
		s.handleWizardGetDraft(getRr, getReq)

		if getRr.Code != http.StatusOK {
			t.Errorf("Expected status OK, got %v", getRr.Code)
		}
		if getRr.Body.String() != \`{"step": 1}\` {
			t.Errorf("Expected body to be {\"step\": 1}, got %v", getRr.Body.String())
		}
        }
        EOF
        ```
    *   *Verification*: `git diff src/server/dashboard/handlers_wizard_test.go`.

5.  **Run Tests globally**:
    *   Run `bazelisk test //...` globally to verify tests pass and regressions aren't introduced.

6.  **Pre-commit steps**:
    *   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
