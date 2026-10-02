# Native chat tenant-context regression gate

Run `bash scripts/chat-tenant-isolation/run.sh` with an explicit
`OHC_CHAT_TEST_DATABASE_URL` selecting an owned loopback UTF8 PostgreSQL database
named `ohc_*_test`. Missing or unsafe prerequisites fail before connection. The
fixture creates uniquely named schemas and login roles only inside that selected
database, verifies non-superuser/NOBYPASSRLS identity, and drops its own objects.

The small crate imports the complete actual chat module, original models, service
methods and their maintained tests. It embeds the same migration directory read
by the production SQLx macro, checks every SQL/version/checksum, and executes the
actual native-chat migration in each disposable schema. Registry dependencies
must match the repository lockfile. There is no SQL mock, provider call, message
delivery, or replacement service implementation.

The original two test names remain. Database absence and SQL failure cannot become
passing tests or a zero-row isolation claim. The restricted pool reuses one real
connection across tenants. This gate covers service/data isolation and migration
discovery; HTTP authentication, live messaging providers and full server acceptance
remain separate checks.

The gate requires all eight cases, with no ignored or filtered acceptance. Parent
IDs are checked inside the same INSERT statement against the supplied tenant;
channel, conversation and message writes cannot reference another tenant's rows.
The regression runs those attempts under both the restricted role and table owner.
PostgreSQL foreign-key checks alone do not establish that tenant relationship
([row-security documentation](https://www.postgresql.org/docs/current/ddl-rowsecurity.html)).
The existing five methods retain their transaction-local tenant context. This is
not proof of caller authentication or live message delivery.
