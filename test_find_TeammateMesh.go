package main

import (
    "fmt"
    "go/ast"
    "go/parser"
    "go/token"
)

func main() {
    fset := token.NewFileSet()
    f, err := parser.ParseFile(fset, "srcs/server/orchestration/mesh.go", nil, 0)
    if err != nil {
        fmt.Println(err)
        return
    }

    for _, d := range f.Decls {
        if genDecl, ok := d.(*ast.GenDecl); ok && genDecl.Tok == token.TYPE {
            for _, spec := range genDecl.Specs {
                if typeSpec, ok := spec.(*ast.TypeSpec); ok {
                    if typeSpec.Name.Name == "TeammateMesh" {
						fmt.Println("Found TeammateMesh")
						if interfaceType, ok := typeSpec.Type.(*ast.InterfaceType); ok {
							for _, method := range interfaceType.Methods.List {
								fmt.Println(method.Names[0].Name)
							}
						}
					}
                }
            }
        }
    }
}
