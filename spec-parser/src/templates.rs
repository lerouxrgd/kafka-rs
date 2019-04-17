use failure::{Error, Fail, SyncFailure};
use tera::{Context, Tera};

pub mod shape {
    /// Vector of (name, code_id, doc)
    pub type ErrorCodeRows = Vec<(String, String, String)>;

    /// Vector of (name, key_id)
    pub type ApiKeyRows = Vec<(String, String)>;

    /// Vector of (name, rust_type, doc)
    pub type Fields = Vec<(String, String, String)>;

    pub type VersionRows = Vec<Fields>;
}

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

pub const REQ_RESP_ENUM_TERA: &str = "req_resp_enum.tera";
pub const REQ_RESP_ENUM_TEMPLATE: &str = r#"
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum {{ name }} {
    {%- for fields in versions %}
    V{{ loop.index0 }} {
        {%- for f in fields %}
{{ f.2 }}
        {{ f.0 }}: {{ f.1 }},
        {%- endfor %}
    },
    {%- endfor %}
}
"#;

pub const REQ_RESP_MOD_TERA: &str = "req_resp_mod.tera";
pub const REQ_RESP_MOD_TEMPLATE: &str = r#"
pub mod {{ name }} {
    {%- for ver in versions %}
    pub mod v{{ loop.index0 }} {
        {%- for struct in ver %}
        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct {{ struct.0 }} {
            {%- for f in struct.1 %}
{{ f.2 }}
            pub {{ f.0 }}: {{ f.1 }},
            {%- endfor %}
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
        tera.add_raw_template(REQ_RESP_ENUM_TERA, REQ_RESP_ENUM_TEMPLATE)
            .sync()?;
        tera.add_raw_template(REQ_RESP_MOD_TERA, REQ_RESP_MOD_TEMPLATE)
            .sync()?;
        Ok(Templater { tera })
    }

    pub fn str_headers(&self) -> &'static str {
        HEADERS
    }

    /// Generates a Rust enum with all Kafka error codes.
    pub fn str_err_codes(&self, err_codes: &shape::ErrorCodeRows) -> Result<String, Error> {
        let mut ctx = Context::new();
        ctx.insert("err_codes", err_codes);
        Ok(self.tera.render(ERROR_CODES_TERA, &ctx).sync()?)
    }

    /// Generates a Rust enum with all Kafka api keys.
    pub fn str_api_keys(&self, api_keys: &shape::ApiKeyRows) -> Result<String, Error> {
        let mut ctx = Context::new();
        ctx.insert("api_keys", api_keys);
        Ok(self.tera.render(API_KEYS_TERA, &ctx).sync()?)
    }

    pub fn str_req_resp_enum(
        &self,
        name: &str,
        versions: &shape::VersionRows,
    ) -> Result<String, Error> {
        let mut ctx = Context::new();
        ctx.insert("name", name);
        ctx.insert("versions", versions);
        Ok(self.tera.render(REQ_RESP_ENUM_TERA, &ctx).sync()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_req_resp_enum() {
        let templater = Templater::new().unwrap();

        let name = "CreateTopicsRequest";

        let versions = vec![vec![
            (
                "create_topic_requests".to_owned(),
                "Vec<create_topic_request::v0::CreateTopicsRequests>".to_owned(),
                "        /// I am a comment.".to_owned(),
            ),
            (
                "timeout".to_owned(),
                "i32".to_owned(),
                "        /// I am another comment.".to_owned(),
            ),
        ]];

        let res = templater.str_req_resp_enum(name, &versions).unwrap();
        println!("{}", res);
    }
}
