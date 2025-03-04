#![deny(warnings)]
use sauron::prelude::*;
use std::{
    cell::RefCell,
    rc::Rc,
};
use test_fixtures::simple_program;
use wasm_bindgen_test::*;
use web_sys::InputEvent;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

// Verify that our DomUpdater's patch method works.
// We test a simple case here, since diff_patch.rs is responsible for testing more complex
// diffing and patching.
#[wasm_bindgen_test]
fn patches_dom() {
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let vdom: Node<()> = div(vec![], vec![]);
    let simple_program = simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(
        &simple_program,
        vdom,
        &sauron_core::body(),
    );

    let new_vdom = div(vec![id("patched")], vec![]); //html! { <div id="patched"></div> };
    dom_updater.update_dom(&simple_program, new_vdom);

    assert_eq!(document.query_selector("#patched").unwrap().is_some(), true);
}

// When you replace a DOM node with another DOM node we need to make sure that the closures
// from the new DOM node are stored by the DomUpdater otherwise they'll get dropped and
// won't work.
#[wasm_bindgen_test]
fn updates_active_closure_on_replace() {
    console_error_panic_hook::set_once();

    let body = sauron_core::body();

    let simple_program = simple_program();
    let old = div(vec![], vec![]);
    let mut dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, old, &body);

    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "update-active-closures-on-replace";

    let replace_node = input(
        vec![
            id(elem_id),
            on_input(move |event: sauron_core::html::events::InputEvent| {
                *text_clone.borrow_mut() = event.value.to_string();
            }),
            value("End Text"),
        ],
        vec![],
    );

    // New node replaces old node.
    // We are testing that we've stored this new node's closures even though `new` will be dropped
    // at the end of this block.
    dom_updater.update_dom(&simple_program, replace_node);

    let input_event = InputEvent::new("input").unwrap();

    assert_eq!(&*text.borrow(), "Start Text");

    // After dispatching the on_input event our `text` should have a value of the input elements value.
    let input = sauron_core::document().get_element_by_id(&elem_id).unwrap();
    web_sys::EventTarget::from(input)
        .dispatch_event(&input_event)
        .unwrap();

    assert_eq!(&*text.borrow(), "End Text");
}

// When you replace a DOM node with another DOM node we need to make sure that the closures
// from the new DOM node are stored by the DomUpdater otherwise they'll get dropped and
// won't work.
#[wasm_bindgen_test]
fn updates_active_closures_on_append() {
    console_error_panic_hook::set_once();

    let body = sauron_core::body();

    let old = div(vec![], vec![]);
    let simple_program = simple_program();
    let mut dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, old, &body);

    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "update-active-closures-on-append";

    {
        let append_node = div(
            vec![],
            vec![input(
                vec![
                    id(elem_id),
                    on_input(
                        move |event: sauron_core::html::events::InputEvent| {
                            *text_clone.borrow_mut() = event.value.to_string();
                        },
                    ),
                    value("End Text"),
                ],
                vec![],
            )],
        );

        // New node gets appended into the DOM.
        // We are testing that we've stored this new node's closures even though `new` will be dropped
        // at the end of this block.
        dom_updater.update_dom(&simple_program, append_node);
    }

    let input_event = InputEvent::new("input").unwrap();

    assert_eq!(&*text.borrow(), "Start Text");

    // After dispatching the on_input event our `text` should have a value of the input elements value.
    let input = sauron_core::document().get_element_by_id(elem_id).unwrap();
    web_sys::EventTarget::from(input)
        .dispatch_event(&input_event)
        .unwrap();

    assert_eq!(&*text.borrow(), "End Text");
}
