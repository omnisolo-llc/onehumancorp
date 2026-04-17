package token_calculator

type ModelPricing struct {
	Name                    string
	CostPerInputToken       float64
	CostPerOutputToken      float64
	CostPerCachedInputToken float64
}

func FindCheapestModel(inputTokens, outputTokens, cachedTokens int, models []ModelPricing) string {
	if len(models) == 0 {
		return ""
	}

	cheapestName := models[0].Name
	lowestCost := float64(-1)

	for _, m := range models {
		cost := float64(inputTokens)*m.CostPerInputToken + float64(outputTokens)*m.CostPerOutputToken + float64(cachedTokens)*m.CostPerCachedInputToken
		if lowestCost == -1 || cost < lowestCost {
			lowestCost = cost
			cheapestName = m.Name
		}
	}
	return cheapestName
}
