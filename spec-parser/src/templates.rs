use tera::{Context, Error, Tera};

/// Types used by templatings methods
pub mod motif {
    /// Vector of (name, code_id, doc)
    pub type ErrorCodeRows = Vec<(String, String, String)>;

    /// Vector of (name, key_id)
    pub type ApiKeyRows = Vec<(String, String)>;

    /// Vector of (name, rust_type, doc)
    pub type Fields = Vec<(String, String, String)>;

    /// A req/resp enum's versioned fields. Each element is a version.
    pub type EnumVfields = Vec<Fields>;

    /// A req/resp module's versioned structs. Each element is a version.
    /// Each version is a vector of (struct_name, struct_fields)
    pub type ModVstructs = Vec<Vec<(String, Fields)>>;
}

const HEADERS: &str = r#"
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct HeaderRequest {
    pub api_key: crate::model::ApiKey,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: crate::types::NullableString,
}

#[derive(Debug, serde::Deserialize)]
pub struct HeaderResponse {
    pub correlation: i32,
}
"#;

const ERROR_CODES_TERA: &str = "error_codes.tera";
const ERROR_CODES_TEMPLATE: &str = r#"
///  Numeric codes to indicate what problem occurred on the Kafka server.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    num_enum::IntoPrimitive,
    num_enum::TryFromPrimitive,
)]
#[repr(i16)]
pub enum ErrorCode {
    {%- for e in err_codes %}
{{ e.2 }}
    {{ e.0 }} = {{ e.1 }},
    {%- endfor %}
}
"#;

const API_KEYS_TERA: &str = "api_keys.tera";
const API_KEYS_TEMPLATE: &str = r#"
///  Numeric codes used to specify request types.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    num_enum::IntoPrimitive,
    num_enum::TryFromPrimitive,
    strum_macros::EnumIter,
)]
#[repr(i16)]
pub enum ApiKey {
    {%- for k in api_keys %}
    {{ k.0 }} = {{ k.1 }},
    {%- endfor %}
}
"#;

const REQ_RESP_ENUM_TERA: &str = "req_resp_enum.tera";
const REQ_RESP_ENUM_TEMPLATE: &str = r#"
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, strum_macros::EnumCount)]
pub enum {{ name }} {
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
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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

/// The main, stateless, component for templating. Current implementation uses Tera.
/// It generates String of Rust code/types corresponding to Kafka protocol.
pub struct Templater {
    tera: Tera,
}

impl Templater {
    /// Creates a new `Templater.`
    pub fn new() -> Result<Templater, Error> {
        let mut tera = Tera::new("/dev/null/*")?;
        tera.add_raw_template(ERROR_CODES_TERA, ERROR_CODES_TEMPLATE)?;
        tera.add_raw_template(API_KEYS_TERA, API_KEYS_TEMPLATE)?;
        tera.add_raw_template(REQ_RESP_ENUM_TERA, REQ_RESP_ENUM_TEMPLATE)?;
        tera.add_raw_template(REQ_RESP_MOD_TERA, REQ_RESP_MOD_TEMPLATE)?;
        Ok(Templater { tera })
    }

    /// Generates structs for request and response headers
    pub fn str_headers(&self) -> &'static str {
        HEADERS
    }

    /// Generates an enum with all Kafka error codes.
    pub fn str_err_codes(&self, err_codes: &motif::ErrorCodeRows) -> Result<String, Error> {
        let mut ctx = Context::new();
        ctx.insert("err_codes", err_codes);
        Ok(self.tera.render(ERROR_CODES_TERA, &ctx)?)
    }

    /// Generates an enum with all Kafka api keys.
    pub fn str_api_keys(&self, api_keys: &motif::ApiKeyRows) -> Result<String, Error> {
        let mut ctx = Context::new();
        ctx.insert("api_keys", api_keys);
        Ok(self.tera.render(API_KEYS_TERA, &ctx)?)
    }

    /// Generates a versioned enum for a given request/response of the Kafka protocol
    pub fn str_req_resp_enum(
        &self,
        enum_name: &str,
        versions: &motif::EnumVfields,
    ) -> Result<String, Error> {
        let mut ctx = Context::new();
        ctx.insert("name", enum_name);
        ctx.insert("versions", versions);
        Ok(self.tera.render(REQ_RESP_ENUM_TERA, &ctx)?)
    }

    /// Generates versioned modules for the inner structs of versioned req_resp enums
    pub fn str_req_resp_mod(
        &self,
        module_name: &str,
        versions: &motif::ModVstructs,
    ) -> Result<String, Error> {
        let mut ctx = Context::new();
        ctx.insert("name", module_name);
        ctx.insert("versions", versions);
        Ok(self.tera.render(REQ_RESP_MOD_TERA, &ctx)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_req_resp_enum() {
        let templater = Templater::new().unwrap();

        let enum_name = "CreateTopicsRequest";

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

        let res = templater.str_req_resp_enum(enum_name, &versions).unwrap();

        let expected = "
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreateTopicsRequest {
    V0 {
        /// I am a comment.
        create_topic_requests: Vec<create_topic_request::v0::CreateTopicsRequests>,
        /// I am another comment.
        timeout: i32,
    },
}
";

        assert_eq!(expected, res);
    }

    #[test]
    fn template_req_resp_mod() {
        let templater = Templater::new().unwrap();

        let mod_name = "create_topics_request";

        let versions = vec![vec![
            (
                "CreateTopicsRequests".to_owned(),
                vec![
                    (
                        "topic".to_owned(),
                        "String".to_owned(),
                        "            /// I am a comment.".to_owned(),
                    ),
                    (
                        "num_partitions".to_owned(),
                        "i32".to_owned(),
                        "            /// I am another comment.".to_owned(),
                    ),
                ],
            ),
            (
                "ReplicaAssignment".to_owned(),
                vec![(
                    "partition".to_owned(),
                    "i32".to_owned(),
                    "            /// I am a comment.".to_owned(),
                )],
            ),
        ]];

        let res = templater.str_req_resp_mod(mod_name, &versions).unwrap();

        let expected = "
pub mod create_topics_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct CreateTopicsRequests {
            /// I am a comment.
            pub topic: String,
            /// I am another comment.
            pub num_partitions: i32,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ReplicaAssignment {
            /// I am a comment.
            pub partition: i32,
        }
    }
}
";

        assert_eq!(expected, res);
    }
}
