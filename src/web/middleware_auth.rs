pub use crate::error::{Error, Result};

use crate::{
    context::Ctx,
    model::ModelController,
    web::{self, AUTH_TOKEN},
};
use axum::{
    RequestPartsExt,
    extract::{FromRequestParts, State},
    http::{Request, request::Parts},
    middleware::Next,
    response::Response,
};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies, cookie};

pub async fn require_auth_middleware<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!(
        "->> {:<12} - require_auth_middleware - {ctx:?}",
        "MIDDLEWARE"
    );
    ctx?;
    Ok(next.run(req).await)
}

pub async fn ctx_resolver_middleware<B>(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - ctx_resolver_middleware", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token.ok_or(Error::NoAuthCookie).and_then(parse_token) {
        Ok((user_id, _e, _s)) => Ok(Ctx::new(user_id)),
        Err(e) => return Err(e),
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::NoAuthCookie)) {
        cookies.remove(Cookie::named(web::AUTH_TOKEN));
    }

    //  store ctx_result into request extension
	// Has to be unique by type - otherwise last write wins
    req.extensions_mut().insert(result_ctx);
    Ok(next.run(req).await)
}

// region: context extractor
#[async_trait::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "extractor");
        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::CtxNotInRequestExtension)?
            .clone()
    }
}
// end_region: context extractor

//  parse token of format: `user-[user-id].[expiration].[signature]`
// Returns (user_id,expiration,signature)

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, signature) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // literal regex
        &token
    )
    .ok_or(Error::WrongTokenFormat)?;
    let user_id = user_id.parse().map_err(|_| Error::WrongTokenFormat)?;
    Ok((user_id, exp.to_string(), signature.to_string()))
}
