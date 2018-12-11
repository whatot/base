fn main() {
    c1_visibility();
    println!();
    c2_struct_visibility();
    println!();
    c3_use();
    println!();
    c4_super_self();
}

mod c1_my {
    // Items in modules default to private visibility
    fn private_function() {
        println!("called c1_my::private_function()");
    }

    // Use the `pub` modifier to override default visibility
    pub fn function() {
        println!("called c1_my::function()");
    }

    // Items can access other items in the same module, even when private.
    pub fn indirect_access() {
        print!("called c1_my::indirect_access(), then\n> ");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("c1_my::nested::function()");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called c1_my::nested::private_function()");
        }
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called c1_my::private_nested::function()");
        }
    }
}

fn function() {
    println!("called function()");
}

fn c1_visibility() {
    // Modules allow disambiguation between items that have the same name.
    function();
    c1_my::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.
    c1_my::indirect_access();
    c1_my::nested::function();
}

mod c2_my {
    // A public struct with a public field of generic type T
    pub struct WhiteBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T,
    }

    impl<T> BlackBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox { contents: contents }
        }
    }
}

fn c2_struct_visibility() {
    // Public structs with public fields can be constructed as usual
    let white_box = c2_my::WhiteBox { contents: "public information" };

    // and their fields can be normally accessed.
    println!("The white box contains: {}", white_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `BlackBox` has private fields
    //let black_box = c2_my::BlackBox { contents: "classified information" };

    // However, structs with private fields can be created using
    // public constructors
    let _black_box = c2_my::BlackBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    //println!("The black box contains: {}", _black_box.contents);
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called deeply::nested::function()");
        }
    }
}

use crate::deeply::nested::function as other_function;

fn c3_use() {
    // easier access to deeply::nested::function
    other_function();

    println!("Entering block");

    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::deeply::nested::function;
        function();

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        println!("Leaving block");
    }
    function();
}

mod cool {
    pub fn function() {
        println!("called cool::function()");
    }
}

mod c4_my {
    fn function() {
        println!("called c4_my::function()");
    }

    mod cool {
        pub fn function() {
            println!("called c4_my::cool::function()");
        }
    }

    pub fn indirect_call() {
        // Let's access all the functions named `function` from this scope!
        println!("called c4_my::indirect_call()");

        // The `self` keyword refers to the current module scope -
        // in this case `c4_my`.
        // Calling `self::function()` and calling `function()` directly both
        // give the same result, because they refer to the same function.
        self::function();
        function();

        // We can also use `self` to access another module inside `my`:
        self::cool::function();

        // The `super` keyword refers to the parent scope
        // (outside the `my` module).
        super::function();

        // This will bind to the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn c4_super_self() {
    c4_my::indirect_call();
}
