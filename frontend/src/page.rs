pub mod register;
pub use route::*;
pub use register::Register;

pub mod route {
    pub const ACCOUNT_REGISTER: &str = "/account/register";
}