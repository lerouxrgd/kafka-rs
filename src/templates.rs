use failure::{Error, Fail, SyncFailure};
use tera::{Context, Tera};

pub const ENUM_TERA: &str = "enum.tera";
pub const ENUM_TEMPLATE: &str = r#"
{%- if doc %}
/// {{ doc }}
{%- endif %}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorCode {
    {%- for s in symbols %}
    {{ s }},
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
/// Its responsability is to generate String representing Rust code/types for Kafka API.
pub struct Templater {
    tera: Tera,
}

impl Templater {
    /// Creates a new `Templater.`
    pub fn new() -> Result<Templater, Error> {
        let mut tera = Tera::new("/dev/null/*").sync()?;
        tera.add_raw_template(ENUM_TERA, ENUM_TEMPLATE).sync()?;
        Ok(Templater { tera })
    }
}
