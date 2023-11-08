pub mod constants;
pub mod events;
pub mod seaport;

use ethers::prelude::abigen;

abigen!(IERC20, "./abi/IERC20.json");
