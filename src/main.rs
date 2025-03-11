mod utils;
mod api;

use api::api::get_test_api;
use utils::is_test::is_submodule_test;
use utils::is_module_test;

fn main() {
    get_test_api();
    is_module_test("module");
    is_submodule_test("submodule");
    println!("Hello, world!");
}
