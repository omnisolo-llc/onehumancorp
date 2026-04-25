# DB Locks and Migrations Fix

During testing, we discovered that `RowsAffected()` is not fully supported with our configuration and may fail unpredictably, so we updated DB providers to bypass this. We also implemented `task_dependencies` join table properly in KAIROS instead of the previously mocked JSON implementations.
