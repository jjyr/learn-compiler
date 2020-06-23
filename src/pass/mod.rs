mod allocate_registers;
mod build_interference;
mod flattern;
mod lower_conditionals;
mod partial_eval;
mod patch_inst;
mod print_x86;
mod select_inst;
mod type_check;
mod uncover_live;
mod uniquify;

pub use allocate_registers::allocate_registers;
pub use build_interference::build_interference;
pub use flattern::flattern;
pub use lower_conditionals::lower_conditionals;
pub use partial_eval::partial_eval;
pub use patch_inst::patch_inst;
pub use print_x86::print_x86;
pub use select_inst::select_inst;
pub use type_check::type_check;
pub use uncover_live::uncover_live;
pub use uniquify::uniquify;
