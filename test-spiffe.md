The problem description says "API endpoints are secured using SPIFFE/SPIRE identity middleware."
I need to find what this middleware is called. Let's see if there's any SPIFFE HTTP middleware.
Wait, `src/server/auth/middleware.go` implements generic middleware, but I'll check how it validates SPIFFE.
