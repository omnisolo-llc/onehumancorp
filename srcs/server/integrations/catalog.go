package integrations

import (
	"onehumancorp/srcs/server/integrations/core"
	"onehumancorp/srcs/server/integrations/libsql"
)

var Catalog = map[string]core.Integration{
	"libsql": libsql.NewLibSQLIntegration(nil),
}
