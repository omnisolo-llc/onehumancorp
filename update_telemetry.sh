sed -i '/requestCounter, err = m.Int64Counter(/i\
\	if err := initSwarmTasksCompletedCounter(m); err != nil {\
\		errs = append(errs, err)\
\	}' srcs/server/telemetry/telemetry.go
