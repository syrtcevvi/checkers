pub mod board;
mod game_data;

pub use self::{
    board::{Board, Message as BoardMessage},
    game_data::GameData,
};
