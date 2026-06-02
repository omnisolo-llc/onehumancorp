import { NextResponse } from 'next/server';

export async function GET() {
    // In a real implementation, this would call the Rust gRPC ForecastEngine.
    // Here we'll return a mocked forecast for demonstration.
    const currentBalance = 10000;
    const avgDailyRevenue = 500;
    const avgDailyExpenses = 2000;

    const predictedRevenue = avgDailyRevenue * 30;
    const predictedExpenses = avgDailyExpenses * 30;
    const predictedBalance = currentBalance + predictedRevenue - predictedExpenses;

    let alert = null;
    let type = "surplus";
    if (predictedBalance < 0) {
        alert = `You might have a $${Math.abs(predictedBalance / 100).toFixed(2)} shortfall next month. Let's resolve it.`;
        type = "shortfall";
    }

    return NextResponse.json({
        forecast_cents: predictedBalance,
        alert_message: alert,
        type: type,
        current_balance: currentBalance,
        monthly_revenue_cents: predictedRevenue,
        monthly_expenses_cents: predictedExpenses,
    });
}
