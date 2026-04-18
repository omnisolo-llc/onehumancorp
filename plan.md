1. Modify `deploy/scripts/ohc_hybrid_cli.sh` to add the new option:
   - Add `echo -e "  8) Seed Database with Mock Data"` to the menu.
   - Add a case for `8)` that calls `bash "$SCRIPT_DIR/ohc-seed-data.sh"`.
2. Ensure pre-commit steps are handled and verified.
3. Submit changes as the Guide agent.
