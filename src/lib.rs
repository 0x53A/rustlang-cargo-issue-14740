#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]
#![feature(cell_update)]
#![feature(gen_blocks)]
#![feature(let_chains)]



use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

// todo: viewport width x height: 480 x 720
// Stretch Mode Canvas Items, Aspect Keep
