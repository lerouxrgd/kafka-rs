use failure::{Error, Fail, SyncFailure};
use tera::{Context, Tera};

use crate::common::{ApiKeyRows, ErrorCodeRows};

pub const HEADERS: &str = r#"
#[derive(Debug, serde::Serialize)]
pub struct HeaderRequest {
    pub api_key: ApiKey,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: crate::types::NullableString,
}

#[derive(Debug, serde::Deserialize)]
pub struct HeaderResponse {
    pub correlation: i32,
}
"#;

pub const ERROR_CODES_TERA: &str = "error_codes.tera";
pub const ERROR_CODES_TEMPLATE: &str = r#"
///  Numeric codes to indicate what problem occurred on the Kafka server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i16)]
pub enum ErrorCode {
    {%- for e in err_codes %}
{{ e.2 }}
    {{ e.0 }} = {{ e.1 }},
    {%- endfor %}
}
"#;

pub const API_KEYS_TERA: &str = "api_keys.tera";
pub const API_KEYS_TEMPLATE: &str = r#"
///  Numeric codes used to specify request types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i16)]
pub enum ApiKey {
    {%- for k in api_keys %}
    {{ k.0 }} = {{ k.1 }},
    {%- endfor %}
}
"#;

pub const STRUCT_TERA: &str = "struct.tera";
pub const STRUCT_TEMPLATE: &str = r#"
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum {{ name_enum }} {
    {%- for version in versions %}
    V{{ loop.index }} {
        ???: ???,
    },
    {%- endfor %}
}

pub mod {{ name_module }} {
    {%- for version in versions %}
    pub mod v{{ loop.index }} {
        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct {{ ??? }} {
{{ ??? }}
            pub ???: ???,
        }
        {%- endfor %}
    }
    {%- endfor %}
}
"#;

/// Describes errors happened while templating Rust code.
#[derive(Fail, Debug)]
#[fail(display = "Template failure: {}", _0)]
pub struct TemplateError(String);

impl TemplateError {
    pub fn new<S: Into<String>>(msg: S) -> TemplateError {
        TemplateError(msg.into())
    }
}

macro_rules! err(
    ($($arg:tt)*) => (Err(TemplateError::new(format!($($arg)*))))
);

/// Fix for converting error-chain to failure.
/// see  https://github.com/rust-lang-nursery/failure/issues/109
trait ResultExt<T, E> {
    fn sync(self) -> Result<T, SyncFailure<E>>
    where
        Self: Sized,
        E: ::std::error::Error + Send + 'static;
}

/// Fix for converting error-chain to failure.
/// see  https://github.com/rust-lang-nursery/failure/issues/109
impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn sync(self) -> Result<T, SyncFailure<E>>
    where
        Self: Sized,
        E: ::std::error::Error + Send + 'static,
    {
        self.map_err(SyncFailure::new)
    }
}

/// The main, stateless, component for templating. Current implementation uses Tera.
/// Its responsability is to generate String of Rust code/types for Kafka API.
pub struct Templater {
    tera: Tera,
}

impl Templater {
    /// Creates a new `Templater.`
    pub fn new() -> Result<Templater, Error> {
        let mut tera = Tera::new("/dev/null/*").sync()?;
        tera.add_raw_template(ERROR_CODES_TERA, ERROR_CODES_TEMPLATE)
            .sync()?;
        tera.add_raw_template(API_KEYS_TERA, API_KEYS_TEMPLATE)
            .sync()?;
        Ok(Templater { tera })
    }

    /// Generates a Rust enum with all Kafka error codes.
    pub fn str_err_codes(&self, err_codes: &ErrorCodeRows) -> Result<String, Error> {
        let mut ctx = Context::new();
        ctx.insert("err_codes", err_codes);
        Ok(self.tera.render(ERROR_CODES_TERA, &ctx).sync()?)
    }

    /// Generates a Rust enum with all Kafka api keys.
    pub fn str_api_keys(&self, api_keys: &ApiKeyRows) -> Result<String, Error> {
        let mut ctx = Context::new();
        ctx.insert("api_keys", api_keys);
        Ok(self.tera.render(API_KEYS_TERA, &ctx).sync()?)
    }
}
