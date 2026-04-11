with open("srcs/server/hub/rag_sync_impl.go", "r") as f:
    code = f.read()

code = code.replace("""\t\tresult, err := tx.Exec(ctx, query, now, id)
\t\tif err != nil {
\t\t\ttelemetry.RecordRagSyncErrorsTotal(ctx, 1)
\t\t\treturn err
\t\t}
\t\trowsAff, _ := result.RowsAffected()
\t\tsuccessCount += int(rowsAff)""", """\t\trowsAff, err := tx.Exec(ctx, query, now, id)
\t\tif err != nil {
\t\t\ttelemetry.RecordRagSyncErrorsTotal(ctx, 1)
\t\t\treturn err
\t\t}
\t\tsuccessCount += int(rowsAff)""")

with open("srcs/server/hub/rag_sync_impl.go", "w") as f:
    f.write(code)
