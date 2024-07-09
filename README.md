# telegram-authorizer

Telegram webapp authorizer layer for Axum.
[![Crates.io](https://img.shields.io/crates/v/telegram-authorizer)](https://crates.io/crates/telegram-authorizer)


## Usage

### Initialization

``` rust
Router::new()
    .route("/", get(login))
    .layer(telegram_authorizer::AuthorizationLayer(bot_token));`
```
### Handler
``` rust
use telegram_authorizer::TelegramUser;

pub async fn login(
    TelegramUser { id }: TelegramUser,
) -> impl IntoResponse {
    tracing::info!("user: {:?}", id);
    Ok(Json(controller::handle(state, id.to_string()).await?))
}
```
### Client

One should send [initData](https://core.telegram.org/bots/webapps#initializing-mini-apps) as query string.
