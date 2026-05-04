use crate::app;

fn create() -> app::WebsiteBuilder { crate::ui_tests::init(); app::WebsiteBuilder::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn builder_invalid_color() {
    let ui = create();
    ui.set_primary_color("not-a-color".into());
    assert_eq!(ui.get_primary_color(), "not-a-color");
}

#[test] fn builder_hex_color_variants() {
    let ui = create();
    ui.set_primary_color("#F00".into());
    assert_eq!(ui.get_primary_color(), "#F00");
    ui.set_primary_color("#FF0000".into());
    assert_eq!(ui.get_primary_color(), "#FF0000");
    ui.set_primary_color("#FF0000FF".into());
    assert_eq!(ui.get_primary_color(), "#FF0000FF");
}

#[test] fn builder_xss_template_name() {
    let ui = create();
    let xss = "'; alert(1); //";
    ui.set_selected_template(xss.into());
    assert_eq!(ui.get_selected_template(), xss);
}

#[test] fn builder_punycode_domain() {
    let ui = create();
    let domain = "xn--bcher-kva.ch";
    ui.set_domain_choice(domain.into());
    assert_eq!(ui.get_domain_choice(), domain);
}

// --- Interaction / Flow Tests ---

#[test] fn builder_flow_rapid_template_switch() {
    let ui = create();
    let templates = ["Modern", "Minimal", "Corporate", "Classic", "Bold"];
    for _ in 0..10 {
        for t in templates {
            ui.set_selected_template(t.into());
            assert_eq!(ui.get_selected_template(), t);
        }
    }
}

#[test] fn builder_flow_price_input_validation() {
    let ui = create();
    ui.set_product_price("10.00".into());
    assert_eq!(ui.get_product_price(), "10.00");
    ui.set_product_price("free".into());
    assert_eq!(ui.get_product_price(), "free");
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_product_name() {
    let ui = create();
    ui.set_product_name("My Store".into());
    assert_eq!(ui.get_product_name(), "My Store");
    ui.set_product_name("".into());
    assert_eq!(ui.get_product_name(), "");
    ui.set_product_name("n11".into());
    assert_eq!(ui.get_product_name(), "n11");
}

#[test]
fn create_verify_product_description() {
    let ui = create();
    ui.set_product_description("Selling cool stuff.".into());
    assert_eq!(ui.get_product_description(), "Selling cool stuff.");
    ui.set_product_description("d26".into());
    assert_eq!(ui.get_product_description(), "d26");
    ui.set_product_description("d27".into());
    assert_eq!(ui.get_product_description(), "d27");
}

#[test]
fn create_verify_step() {
    let ui = create();
    ui.set_step(1);
    assert_eq!(ui.get_step(), 1);
    ui.set_step(46);
    assert_eq!(ui.get_step(), 46);
    ui.set_step(47);
    assert_eq!(ui.get_step(), 47);
}

#[test]
fn create_verify_primary_color() {
    let ui = create();
    ui.set_primary_color("rgb(255,0,0)".into());
    assert_eq!(ui.get_primary_color(), "rgb(255,0,0)");
    ui.set_primary_color("pc41".into());
    assert_eq!(ui.get_primary_color(), "pc41");
    ui.set_primary_color("pc42".into());
    assert_eq!(ui.get_primary_color(), "pc42");
}

#[test]
fn create_verify_selected_template() {
    let ui = create();
    ui.set_selected_template("Dark Mode".into());
    assert_eq!(ui.get_selected_template(), "Dark Mode");
    ui.set_selected_template("st36".into());
    assert_eq!(ui.get_selected_template(), "st36");
    ui.set_selected_template("st37".into());
    assert_eq!(ui.get_selected_template(), "st37");
}

#[test]
fn create_verify_domain_choice() {
    let ui = create();
    ui.set_domain_choice("shop.com".into());
    assert_eq!(ui.get_domain_choice(), "shop.com");
    ui.set_domain_choice("dc31".into());
    assert_eq!(ui.get_domain_choice(), "dc31");
    ui.set_domain_choice("dc32".into());
    assert_eq!(ui.get_domain_choice(), "dc32");
}

#[test]
fn create_verify_product_price() {
    let ui = create();
    ui.set_product_price("99.99".into());
    assert_eq!(ui.get_product_price(), "99.99");
    ui.set_product_price("p21".into());
    assert_eq!(ui.get_product_price(), "p21");
    ui.set_product_price("p22".into());
    assert_eq!(ui.get_product_price(), "p22");
}


#[test]
fn test_drag_and_drop_blocks() {
    let ui = create();

    // Simulate reordering
    let blocks: Vec<String> = slint::Model::iter(&ui.get_blocks()).map(|s| s.to_string()).collect();
    assert_eq!(blocks[0], "Hero");
    assert_eq!(blocks[1], "Product Grid");

    // Set UI to step 5 to mimic dashboard
    ui.set_step(5);

    ui.on_move_block_up({
        use slint::ComponentHandle;
        let ui_handle = ui.as_weak();
        move |index| {
            let ui = ui_handle.unwrap();
            let mut blocks: Vec<slint::SharedString> = slint::Model::iter(&ui.get_blocks()).collect();
            let idx = index as usize;
            if idx > 0 && idx < blocks.len() {
                blocks.swap(idx, idx - 1);
                let model = std::rc::Rc::new(slint::VecModel::from(blocks));
                ui.set_blocks(model.into());
            }
        }
    });

    ui.on_move_block_down({
        use slint::ComponentHandle;
        let ui_handle = ui.as_weak();
        move |index| {
            let ui = ui_handle.unwrap();
            let mut blocks: Vec<slint::SharedString> = slint::Model::iter(&ui.get_blocks()).collect();
            let idx = index as usize;
            if idx < blocks.len() - 1 {
                blocks.swap(idx, idx + 1);
                let model = std::rc::Rc::new(slint::VecModel::from(blocks));
                ui.set_blocks(model.into());
            }
        }
    });

    ui.invoke_move_block_up(1);
    let blocks_after: Vec<String> = slint::Model::iter(&ui.get_blocks()).map(|s| s.to_string()).collect();
    assert_eq!(blocks_after[0], "Product Grid");
    assert_eq!(blocks_after[1], "Hero");

    ui.invoke_move_block_down(0);
    let blocks_down: Vec<String> = slint::Model::iter(&ui.get_blocks()).map(|s| s.to_string()).collect();
    assert_eq!(blocks_down[0], "Hero");
    assert_eq!(blocks_down[1], "Product Grid");
}

#[test]
fn test_block_editor() {
    let ui = create();

    ui.on_start_edit_block({
        use slint::ComponentHandle;
        let ui_handle = ui.as_weak();
        move |index| {
            let ui = ui_handle.unwrap();
            ui.set_editing_block_index(index);
            ui.set_step(6);
        }
    });

    ui.on_finish_edit_block({
        use slint::ComponentHandle;
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_editing_block_index(-1);
            ui.set_step(5);
        }
    });

    ui.invoke_start_edit_block(2);
    assert_eq!(ui.get_step(), 6);
    assert_eq!(ui.get_editing_block_index(), 2);

    ui.invoke_finish_edit_block();
    assert_eq!(ui.get_step(), 5);
    assert_eq!(ui.get_editing_block_index(), -1);
}
