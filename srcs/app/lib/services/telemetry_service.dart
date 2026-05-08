class TelemetryService {
  void trackEvent(String eventName, {Map<String, dynamic>? properties}) {
    // In a real app, this would send data to OpenTelemetry or Prometheus
    print('TELEMETRY: $eventName, Properties: $properties');
  }

  void trackError(String errorName, String message) {
    print('TELEMETRY ERROR: $errorName, Message: $message');
  }
}
