// Copyright 2023 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::Parser;
use mas_config::RootConfig;
use mas_router::UrlBuilder;
use tracing::{info_span, log::info};

use crate::util::{database_from_config, mailer_from_config, templates_from_config};

#[derive(Parser, Debug, Default)]
pub(super) struct Options {}

impl Options {
    pub async fn run(&self, root: &super::Options) -> anyhow::Result<()> {
        let span = info_span!("cli.worker.init").entered();
        let config: RootConfig = root.load_config()?;

        // Connect to the database
        info!("Conntecting to the database");
        let pool = database_from_config(&config.database).await?;

        let url_builder = UrlBuilder::new(config.http.public_base.clone());

        // Load and compile the templates
        let templates = templates_from_config(&config.templates, &url_builder).await?;

        let mailer = mailer_from_config(&config.email, &templates).await?;
        mailer.test_connection().await?;
        drop(config);

        info!("Starting task scheduler");
        let monitor = mas_tasks::init(&pool, &mailer);

        span.exit();

        monitor.run().await?;
        Ok(())
    }
}
