// ──────────────────────────────────────────────────────────────────────────────
// redis.rs – connection setup + helpers
// ──────────────────────────────────────────────────────────────────────────────
use redis::{
    aio::{MultiplexedConnection},
    AsyncCommands, Client,
};
use actix_web::web;

/// Build a single multiplexed connection the Actix app can share.
///
/// ```rust
/// let redis = redis_conn("redis://127.0.0.1/").await.unwrap();
/// HttpServer::new(move || {
///     App::new()
///         .app_data(redis.clone())
///         // …
/// })
/// # ;
/// ```
pub async fn redis_conn(url: &str) -> redis::RedisResult<web::Data<MultiplexedConnection>> {
    let client = Client::open(url)?;
    let conn = client
        .get_multiplexed_async_connection()
        .await?;

    Ok(web::Data::new(conn))
}

// ─────────────────────────────────────────
// Cache helpers
// ─────────────────────────────────────────
pub async fn set_cache(
    redis: &web::Data<MultiplexedConnection>,
    key: &str,
    val: &str,
    ttl: Option<usize>,
) {
    let mut conn = redis.get_ref().clone();
    let res: Result<(), redis::RedisError> = match ttl {
        Some(secs) => conn.set_ex(key, val, secs.try_into().unwrap()).await,
        None       => conn.set(key, val).await,
    };
    if let Err(e) = res {
        eprintln!("Redis SET failed: {e}");
    }
}

pub async fn has_cache(
    redis: &web::Data<MultiplexedConnection>,
    key: &str,
) -> bool {
    let mut conn = redis.get_ref().clone();
    conn.exists::<_, bool>(key).await.unwrap_or(false)
}

pub async fn get_cache(
    redis: &web::Data<MultiplexedConnection>,
    key: &str,
) -> String {
    // `MultiplexedConnection` is cheap to clone (just another Arc inside).
    let mut conn = redis.get_ref().clone();

    conn.get::<_, String>(key).await.unwrap_or_default()
}

/// Convenience wrapper: use/compute-and-cache JSON strings.
///
/// ```rust
/// let body = async_cache_as_json(&redis, "posts:42", || async {
///     fetch_posts_from_db().await
/// }, 300).await;
/// ```
pub async fn async_cache_as_json<F, Fut>(
    redis: &web::Data<MultiplexedConnection>,
    key: &str,
    compute: F,
    ttl_secs: usize,
) -> String
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = String>,
{
    if has_cache(redis, key).await {
        get_cache(redis, key).await
    } else {
        let value = compute().await;
        set_cache(redis, key, &value, Some(ttl_secs)).await;
        value
    }
}
