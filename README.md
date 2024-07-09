# telegram-authorizer

Telegram [miniapp](https://core.telegram.org/bots/webapps) authorizer layer for Axum.

[![Rust](https://github.com/evilbluebeaver/telegram-authorizer/actions/workflows/rust.yml/badge.svg)](https://github.com/evilbluebeaver/telegram-authorizer/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/telegram-authorizer)](https://crates.io/crates/telegram-authorizer)


## Usage

### Router

``` rust
...
Router::new()
    .route("/", get(login))
    .layer(telegram_authorizer::AuthorizationLayer(bot_token));`
...
```
### Handler
``` rust
use telegram_authorizer::TelegramUser;

pub async fn login(TelegramUser(id): TelegramUser) -> impl IntoResponse {
    tracing::info!("user: {:?}", id);
    ...
}
```
### Client

One should send [initData](https://core.telegram.org/bots/webapps#initializing-mini-apps) as query string.
