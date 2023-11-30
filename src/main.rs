#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(router::router().into())
}

pub mod imports;

#[cfg(test)]
pub mod testing;

mod router;
mod tasks;
