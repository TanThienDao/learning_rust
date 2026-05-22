pub mod inventory;
pub mod order;
use std::{
    fmt,
    io::{stdin,stdout},
};
pub use inventory::product::{Item, ProductCategory};
pub use inventory::{FLOOR_SOACE, MANAGER as INVENTORY_MANAGER, talk_to_manager};
pub use order::MANAGER as ORDER_MANAGER;
