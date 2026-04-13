package pricing

import "time"

// ForecastMonthlyBill estimates the total monthly cost based on a given average daily token cost
// and the number of days already elapsed in the current month.
// (daysElapsed is included for future expansion where non-linear forecasting might be needed).
func ForecastMonthlyBill(dailyAverageCost float64, daysElapsed int, totalDaysInMonth int) float64 {
	if totalDaysInMonth <= 0 {
		return 0.0
	}

	_ = daysElapsed // Acknowledge parameter is currently unused in simple linear model

	// Assuming consistent usage, multiply daily average by total days
	projectedTotal := dailyAverageCost * float64(totalDaysInMonth)
	return projectedTotal
}

// ForecastMonthlyBillFromDate uses the current date to determine days in month.
func ForecastMonthlyBillFromDate(dailyAverageCost float64, currentDate time.Time) float64 {
	year, month, _ := currentDate.Date()

	// Idiomatic Go way to get the last day of the current month, safe from DST changes:
	// Day 0 of the *next* month is the last day of the *current* month.
	lastOfThisMonth := time.Date(year, month+1, 0, 0, 0, 0, 0, currentDate.Location())

	totalDaysInMonth := lastOfThisMonth.Day()
	daysElapsed := currentDate.Day()

	return ForecastMonthlyBill(dailyAverageCost, daysElapsed, totalDaysInMonth)
}
