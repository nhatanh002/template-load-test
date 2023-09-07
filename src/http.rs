//! Simple Goose load test example. Duplicates the simple example on the
//! Locust project page (<https://locust.io/>).
//!
//! ## License
//!
//! Copyright 2020-2022 Jeremy Andrews
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//! <http://www.apache.org/licenses/LICENSE-2.0>
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

use goose::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("TypicalInteraction")
                .set_wait_time(Duration::from_secs(0), Duration::from_secs(0))?
                // Login - TBD
                // .register_transaction(transaction!(login).set_on_start())
                .register_transaction(transaction!(get_a_cat))
        )
        .execute()
        .await?;

    Ok(())
}

async fn login(user: &mut GooseUser) -> TransactionResult {
    let params = [("username", "test_user"), ("password", "")];
    let _goose = user.post_form("/login", &params).await?;

    Ok(())
}

/// A very simple transaction
async fn get_a_cat(user: &mut GooseUser) -> TransactionResult {
    let _goose = user.get("/cats/dd77eef2-7c71-4ccd-9bd2-97f445164ff6").await?;
    // let _goose = user.get("/cats/ping").await?;

    Ok(())
}
