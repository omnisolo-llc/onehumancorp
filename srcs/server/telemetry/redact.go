package telemetry

import (
	"reflect"
	"strings"
)

// isSensitive checks if a key is considered sensitive.
func isSensitive(key string) bool {
	k := strings.ToLower(key)
	return k == "first_name" ||
		k == "last_name" ||
		k == "secret_key" ||
		k == "api_key" ||
		k == "password" ||
		k == "token" ||
		k == "email" ||
		k == "tenant_id" ||
		k == "organization_id" ||
		k == "session_data" ||
		k == "session_id" ||
		k == "ip_address" ||
		k == "mac_address" ||
		k == "geolocation"
}

// RedactInterfacePII redacts PII from an interface map.
func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	if attrs == nil {
		return nil
	}

	val := redactDeep(reflect.ValueOf(attrs))
	if val.IsValid() && val.CanInterface() {
		if result, ok := val.Interface().(map[string]interface{}); ok {
			return result
		}
	}
	return attrs
}

func redactDeep(v reflect.Value) reflect.Value {
	if !v.IsValid() {
		return v
	}

	switch v.Kind() {
	case reflect.Map:
		newMap := reflect.MakeMap(v.Type())
		for _, key := range v.MapKeys() {
			if key.Kind() == reflect.String && isSensitive(key.String()) {
				// We need to set it to the appropriate type, usually interface{} in map[string]interface{}
				// To do this dynamically:
				elemType := v.Type().Elem()
				if elemType.Kind() == reflect.Interface {
					newMap.SetMapIndex(key, reflect.ValueOf("[REDACTED]"))
				} else if elemType.Kind() == reflect.String {
					newMap.SetMapIndex(key, reflect.ValueOf("[REDACTED]"))
				} else {
					// Cannot redact easily if map value is not string or interface{}, leave it
					newMap.SetMapIndex(key, v.MapIndex(key))
				}
			} else {
				newMap.SetMapIndex(key, redactDeep(v.MapIndex(key)))
			}
		}
		return newMap
	case reflect.Slice, reflect.Array:
		newSlice := reflect.MakeSlice(v.Type(), v.Len(), v.Cap())
		for i := 0; i < v.Len(); i++ {
			newSlice.Index(i).Set(redactDeep(v.Index(i)))
		}
		return newSlice
	case reflect.Struct:
		newMap := make(map[string]interface{})
		t := v.Type()
		for i := 0; i < v.NumField(); i++ {
			field := t.Field(i)
			if field.PkgPath != "" { // unexported
				continue
			}
			name := field.Name
			jsonTag := field.Tag.Get("json")
			if jsonTag != "" && jsonTag != "-" {
				parts := strings.Split(jsonTag, ",")
				name = parts[0]
			}
			if isSensitive(name) {
				newMap[name] = "[REDACTED]"
			} else {
				val := redactDeep(v.Field(i))
				if val.IsValid() && val.CanInterface() {
					newMap[name] = val.Interface()
				}
			}
		}
		return reflect.ValueOf(newMap)
	case reflect.Ptr, reflect.Interface:
		if v.IsNil() {
			return v
		}
		// Try to preserve interface type
		elem := redactDeep(v.Elem())
		if v.Kind() == reflect.Interface {
			// Wrapping back into interface might be tricky, we just return the elem
			// But for assignment into slices/maps of interface{}, it works fine.
			return elem
		}
		return elem // Return by value since we are copying
	default:
		return v
	}
}
