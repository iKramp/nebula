// enable a bunch of clippy lints
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::assertions_on_result_states)]
#![warn(clippy::create_dir)]
#![warn(clippy::decimal_literal_representation)]
#![warn(clippy::deref_by_slicing)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::exit)]
#![warn(clippy::expect_used)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::fn_to_numeric_cast_any)]
#![warn(clippy::format_push_string)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::let_underscore_must_use)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::mem_forget)]
#![warn(clippy::non_ascii_literal)]
#![warn(clippy::panic)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::same_name_method)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::single_char_lifetime_names)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_to_string)]
#![warn(clippy::todo)]
#![warn(clippy::try_err)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unreachable)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::verbose_file_reads)]
// disable some Clippy lints
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::suboptimal_flops)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::similar_names)]
#![allow(clippy::ptr_as_ptr)]
#![allow(clippy::shadow_unrelated)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::missing_errors_doc)]
#![allow(unused_imports)]
use anyhow::{Ok, Result};
use iced::futures::FutureExt;
use std::env;
use tokio_postgres::NoTls;
mod database;
use database::database_commands;
mod networking;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<()> {
    //TODO: commented for now so others can run the server until i set up a DB that can accept outside connections
    let client = database::connect_to_db("nebula").await?;
    let _db_manager = database::database_actions::DbManager::new(&client).await;

    networking::listen_for_client();

    println!("finished executing");
    Ok(())
}
