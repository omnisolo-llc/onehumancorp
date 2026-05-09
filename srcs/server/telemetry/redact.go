package telemetry

import (
	"reflect"
	"strings"
)

var sensitiveExactKeys = map[string]bool{
	"tenant_id":       true,
	"organization_id": true,
	"org_id":          true,
	"session_data":    true,
	"session_id":      true,
	"token":           true,
	"email":           true,
	"password":        true,
	"pii":             true,
	"api_key":         true,
	"secret_key":      true,
	"credit":          true,
	"card":            true,
	"cvv":             true,
	"dob":             true,
	"birth":           true,
	"passport":        true,
	"bank":            true,
	"account":         true,
	"stripe":          true,
	"billing":         true,
	"ip_address":      true,
	"mac_address":     true,
	"geolocation":     true,
	"auth":            true,
	"cookie":          true,
	"credential":      true,
	"phone":           true,
	"ssn":             true,
	"address":         true,
	"name":            true,
	"first_name":      true,
	"last_name":       true,
	"payload":         true,
}

func IsSensitiveKeyExact(key string) bool {
	lowerKey := strings.ToLower(key)

	// Fast path exact match
	if sensitiveExactKeys[lowerKey] {
		return true
	}

	// Handle camel case structurally by lowercasing but stripping underscores
	if IsSensitiveKeyExactNoUnderscore(lowerKey) {
		return true
	}

	// For highly sensitive data, use substring matching to prevent compound key leakage
	// (e.g. "user_password", "customer_email")
	if strings.Contains(lowerKey, "password") ||
		strings.Contains(lowerKey, "secret") ||
		strings.Contains(lowerKey, "token") ||
		strings.Contains(lowerKey, "email") ||
		strings.Contains(lowerKey, "ssn") ||
		strings.Contains(lowerKey, "credit_card") ||
		strings.Contains(lowerKey, "cvv") ||
		strings.Contains(lowerKey, "stripe") ||
		strings.Contains(lowerKey, "bank_account") ||
		strings.Contains(lowerKey, "passport") {
		return true
	}

	return false
}

func IsSensitiveKeyExactNoUnderscore(key string) bool {
	for k := range sensitiveExactKeys {
		if strings.ReplaceAll(k, "_", "") == key {
			return true
		}
	}
	return false
}

// isEmail checks for the basic structure of an email in a string value.
func isEmail(s string) bool {
	return strings.Contains(s, "@") && strings.Contains(s, ".")
}

// RedactPII redacts PII using deep traversal via reflect.
func RedactPII(val interface{}) interface{} {
	if val == nil {
		return nil
	}
	v := reflect.ValueOf(val)
	redacted := redactReflect(v)
	if redacted.IsValid() && redacted.CanInterface() {
		return redacted.Interface()
	}
	return val
}

func redactReflect(v reflect.Value) reflect.Value {
	if !v.IsValid() {
		return v
	}

	// Check explicit types that we shouldn't touch or clone structurally.
	if v.Type().String() == "time.Time" {
		return v
	}

	switch v.Kind() {
	case reflect.Ptr:
		if v.IsNil() {
			return v
		}
		elem := redactReflect(v.Elem())
		res := reflect.New(v.Type().Elem())
		res.Elem().Set(elem)
		return res

	case reflect.Interface:
		if v.IsNil() {
			return v
		}
		elem := redactReflect(v.Elem())
		if elem.IsValid() && elem.CanInterface() {
			return reflect.ValueOf(elem.Interface())
		}
		return v

	case reflect.Map:
		if v.IsNil() {
			return v
		}
		res := reflect.MakeMap(v.Type())
		iter := v.MapRange()
		for iter.Next() {
			k := iter.Key()
			val := iter.Value()

			if k.Kind() == reflect.String && IsSensitiveKeyExact(k.String()) {
				elemType := v.Type().Elem()
				if elemType.Kind() == reflect.Interface {
					res.SetMapIndex(k, reflect.ValueOf("[REDACTED]"))
				} else if elemType.Kind() == reflect.String {
					res.SetMapIndex(k, reflect.ValueOf("[REDACTED]"))
				} else {
					res.SetMapIndex(k, reflect.Zero(elemType))
				}
			} else {
				res.SetMapIndex(k, redactReflect(val))
			}
		}
		return res

	case reflect.Slice:
		if v.IsNil() {
			return v
		}
		res := reflect.MakeSlice(v.Type(), v.Len(), v.Cap())
		for i := 0; i < v.Len(); i++ {
			res.Index(i).Set(redactReflect(v.Index(i)))
		}
		return res

	case reflect.Array:
		res := reflect.New(v.Type()).Elem()
		for i := 0; i < v.Len(); i++ {
			res.Index(i).Set(redactReflect(v.Index(i)))
		}
		return res

	case reflect.Struct:
		res := reflect.New(v.Type()).Elem()
		for i := 0; i < v.NumField(); i++ {
			field := v.Type().Field(i)
			// we can only set exported fields
			if field.PkgPath != "" {
				continue
			}

			if IsSensitiveKeyExact(field.Name) {
				if field.Type.Kind() == reflect.String {
					res.Field(i).SetString("[REDACTED]")
				} else if field.Type.Kind() == reflect.Interface {
					res.Field(i).Set(reflect.ValueOf("[REDACTED]"))
				} else {
					// leave as zero value
				}
			} else {
				res.Field(i).Set(redactReflect(v.Field(i)))
			}
		}

		// If we skip unexported fields, we lose data for complex types.
		// For safety in telemetry, we could serialize to map. But let's check explicit types above.
		return res

	case reflect.String:
		if isEmail(v.String()) {
			return reflect.ValueOf("[EMAIL_REDACTED]")
		}
		return v

	default:
		return v
	}
}

// RedactInterfacePII redacts PII from an interface map.
func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	if attrs == nil {
		return nil
	}
	res := RedactPII(attrs)
	if m, ok := res.(map[string]interface{}); ok {
		return m
	}
	return attrs
}
