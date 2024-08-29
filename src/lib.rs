mod bindings {
    use super::Component;

    wit_bindgen::generate!({ generate_all });

    export!(Component);
}

pub struct Component;
