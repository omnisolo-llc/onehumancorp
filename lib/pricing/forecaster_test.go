package pricing

import (
	"math"
	"testing"
	"time"
)

func TestForecastMonthlyBill(t *testing.T) {
	cost := ForecastMonthlyBill(10.0, 15, 30)
	expected := 300.0
	if math.Abs(cost-expected) > 1e-9 {
		t.Errorf("expected %f, got %f", expected, cost)
	}
}

func TestForecastMonthlyBillFromDate(t *testing.T) {
	// April has 30 days
	testDate := time.Date(2026, time.April, 15, 12, 0, 0, 0, time.UTC)
	cost := ForecastMonthlyBillFromDate(10.0, testDate)
	expected := 300.0
	if math.Abs(cost-expected) > 1e-9 {
		t.Errorf("expected %f, got %f", expected, cost)
	}

	// February 2024 (Leap year) has 29 days
	testDateLeap := time.Date(2024, time.February, 15, 12, 0, 0, 0, time.UTC)
	costLeap := ForecastMonthlyBillFromDate(10.0, testDateLeap)
	expectedLeap := 290.0
	if math.Abs(costLeap-expectedLeap) > 1e-9 {
		t.Errorf("expected %f, got %f", expectedLeap, costLeap)
	}
}
