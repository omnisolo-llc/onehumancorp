import re

with open('srcs/server/telemetry/telemetry.go', 'r') as f:
    content = f.read()

# Add to vars
vars_to_add = """
	SessionCostGauge metric.Float64Gauge
"""

content = content.replace("	tokenBurnRateGauge                 metric.Float64Gauge", "	tokenBurnRateGauge                 metric.Float64Gauge" + vars_to_add)

# Add to InitWithMeter
init_to_add = """
	SessionCostGauge, err = m.Float64Gauge(
		"ohc_agent_cost_usd_total",
		metric.WithDescription("Total session cost in USD for the agent"),
	)
	if err != nil {
		errs = append(errs, err)
	}

"""

content = content.replace('	tokenBurnRateGauge, err = m.Float64Gauge(', init_to_add + '	tokenBurnRateGauge, err = m.Float64Gauge(')

with open('srcs/server/telemetry/telemetry.go', 'w') as f:
    f.write(content)
