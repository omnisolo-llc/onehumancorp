use crate::app;
use slint::ComponentHandle;
use slint::Model;
use std::rc::Rc;

fn create() -> app::Integrations { crate::ui_tests::init(); app::Integrations::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn integr_xss_tool_name() {
    let ui = create();
    let xss = "<script>alert('mcp')</script>";
    let tools = slint::VecModel::from(vec![
        app::UiMcpTool {
            id: "t1".into(),
            name: xss.into(),
            description: "desc".into(),
        }
    ]);
    ui.set_tools(Rc::new(tools).into());
    assert_eq!(ui.get_tools().row_data(0).unwrap().name, xss);
}

#[test] fn integr_injection_id() {
    let ui = create();
    let inj = "tool'); DROP TABLE tools; --";
    let tools = slint::VecModel::from(vec![
        app::UiMcpTool {
            id: inj.into(),
            name: "SqlTool".into(),
            description: "desc".into(),
        }
    ]);
    ui.set_tools(Rc::new(tools).into());
    assert_eq!(ui.get_tools().row_data(0).unwrap().id, inj);
}

#[test] fn integr_massive_tools() {
    let ui = create();
    let v: Vec<app::UiMcpTool> = (0..200).map(|i| app::UiMcpTool {
        id: format!("id-{}", i).into(),
        name: format!("Tool {}", i).into(),
        description: "description".into(),
    }).collect();
    ui.set_tools(Rc::new(slint::VecModel::from(v)).into());
    assert_eq!(ui.get_tools().row_count(), 200);
}

// --- Interaction / Flow Tests ---

#[test] fn integr_flow_configure_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    ui.on_configure_integration(move |name| { *c.borrow_mut() = name.to_string(); });
    ui.invoke_configure_integration("Slack".into());
    assert_eq!(*called.borrow(), "Slack");
}

#[test] fn integr_flow_invoke_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    ui.on_invoke_tool(move |id| { *c.borrow_mut() = id.to_string(); });
    ui.invoke_invoke_tool("TOOL-X".into());
    assert_eq!(*called.borrow(), "TOOL-X");
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v_i {
    ($id:ident, $name:expr, $tool_id:expr) => {
        #[test] fn $id() {
            let ui = create();
            let t = slint::VecModel::from(vec![app::UiMcpTool {
                id: $tool_id.into(),
                name: $name.into(),
                description: "desc".into(),
            }]);
            ui.set_tools(Rc::new(t).into());
            assert_eq!(ui.get_tools().row_data(0).unwrap().name, $name);
            assert_eq!(ui.get_tools().row_data(0).unwrap().id, $tool_id);
        }
    };
}

test_v_i!(u1, "Shell Executor", "shell");
test_v_i!(u2, "Browser Automation", "browser");
test_v_i!(u3, "File System Access", "fs");

test_v_i!(u11, "n11", "id11");
test_v_i!(u12, "n12", "id12");
test_v_i!(u13, "n13", "id13");
test_v_i!(u14, "n14", "id14");
test_v_i!(u15, "n15", "id15");
test_v_i!(u16, "n16", "id16");
test_v_i!(u17, "n17", "id17");
test_v_i!(u18, "n18", "id18");
test_v_i!(u19, "n19", "id19");
test_v_i!(u20, "n20", "id20");

test_v_i!(u21, "🚀 Speed Test", "speed");
test_v_i!(u22, "Tool 'Quotes'", "quoted");
test_v_i!(u23, "Tool ; Semi", "semi");
test_v_i!(u24, "", "");
test_v_i!(u25, "Very Long Tool Name ".repeat(5), "long");

test_v_i!(u31, "n31", "id31");
test_v_i!(u32, "n32", "id32");
test_v_i!(u33, "n33", "id33");
test_v_i!(u34, "n34", "id34");
test_v_i!(u35, "n35", "id35");
test_v_i!(u36, "n36", "id36");
test_v_i!(u37, "n37", "id37");
test_v_i!(u38, "n38", "id38");
test_v_i!(u39, "n39", "id39");
test_v_i!(u40, "n40", "id40");

test_v_i!(u41, "n41", "id41");
test_v_i!(u42, "n42", "id42");
test_v_i!(u43, "n43", "id43");
test_v_i!(u44, "n44", "id44");
test_v_i!(u45, "n45", "id45");
test_v_i!(u46, "n46", "id46");
test_v_i!(u47, "n47", "id47");
test_v_i!(u48, "n48", "id48");
test_v_i!(u49, "n49", "id49");
test_v_i!(u50, "n50", "id50");

test_v_i!(u51, "n51", "id51");
test_v_i!(u52, "n52", "id52");
test_v_i!(u53, "id53", "id53");
test_v_i!(u54, "id54", "id54");
test_v_i!(u55, "id55", "id55");
test_v_i!(u56, "id56", "id56");
test_v_i!(u57, "id57", "id57");
test_v_i!(u58, "id58", "id58");
test_v_i!(u59, "id59", "id59");
test_v_i!(u60, "id60", "id60");

test_v_i!(u61, "id61", "id61");
test_v_i!(u62, "id62", "id62");
test_v_i!(u63, "id63", "id63");
test_v_i!(u64, "id64", "id64");
test_v_i!(u65, "id65", "id65");
test_v_i!(u66, "id66", "id66");
test_v_i!(u67, "id67", "id67");
test_v_i!(u68, "id68", "id68");
test_v_i!(u69, "id69", "id69");
test_v_i!(u70, "id70", "id70");

test_v_i!(u71, "id71", "id71");
test_v_i!(u72, "id72", "id72");
test_v_i!(u73, "id73", "id73");
test_v_i!(u74, "id74", "id74");
test_v_i!(u75, "id75", "id75");
test_v_i!(u76, "id76", "id76");
test_v_i!(u77, "id77", "id77");
test_v_i!(u78, "id78", "id78");
test_v_i!(u79, "id79", "id79");
test_v_i!(u80, "id80", "id80");
