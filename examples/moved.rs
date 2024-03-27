use std::ptr;

fn main() {
    let mut self_1 = SelfRef::new("xfy1");
    self_1.init_mut();

    let mut self_2 = SelfRef::new("xfy2");
    self_2.init_mut();

    println!(
        "self_ref value {v:?} and pointer {p:?} and pointer_value {pv}",
        v = self_1.value(),
        p = self_1.pointer,
        pv = self_1.pointer_value()
    );
    println!(
        "self_ref value {v:?} and pointer {p:?} and pointer_value {pv}",
        v = self_2.value(),
        p = self_2.pointer,
        pv = self_2.pointer_value()
    );

    std::mem::swap(&mut self_1, &mut self_2);
    println!("after swap");

    println!(
        "self_ref value {v:?} and pointer {p:?} and pointer_value {pv}",
        v = self_1.value(),
        p = self_1.pointer,
        pv = self_1.pointer_value()
    );
    println!(
        "self_ref value {v:?} and pointer {p:?} and pointer_value {pv}",
        v = self_2.value(),
        p = self_2.pointer,
        pv = self_2.pointer_value()
    );

    {
        let mut self_ref = SelfRef::new("function");
        self_ref.init_mut();
        println!(
            "self_ref value {v:?} and pointer {p:?} and pointer_value {pv}",
            v = self_ref.value(),
            p = self_ref.pointer,
            pv = self_ref.pointer_value()
        );
        print_value(self_ref);
    }
}

fn print_value(self_ref: SelfRef) {
    println!(
        "self_ref in function value {v:?} and pointer {p:?} and pointer_value {pv}",
        v = self_ref.value(),
        p = self_ref.pointer,
        pv = self_ref.pointer_value()
    );
}

#[derive(Debug)]
struct SelfRef {
    value: String,
    pointer: *const String,
}

impl SelfRef {
    fn new(v: &str) -> Self {
        let value = v.to_string();

        Self {
            value,
            pointer: ptr::null(),
        }
    }

    fn init_mut(&mut self) {
        let p: *const String = &self.value;
        self.pointer = p;
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn pointer_value(&self) -> &str {
        assert!(!self.pointer.is_null(), "must init");
        unsafe { &(*self.pointer) }
    }
}
