use serde_json::{Error, Value};

use crate::client::BoundRequest;
use crate::entities::{EndpointData, RequestPayload};
use crate::error::CarteroError;

pub struct CurlService {
    endpoint_data: EndpointData,
}

impl CurlService {
    pub fn new(endpoint_data: EndpointData) -> Self {
        Self { endpoint_data }
    }

    pub fn generate(&self) -> Result<String, CarteroError> {
        let bound_request = BoundRequest::try_from(self.endpoint_data.clone())?;
        let mut command = "curl".to_string();

        command.push_str(&*{
            let method_str: String = bound_request.method.into();
            format!(" -X {} {}", method_str, bound_request.url)
        });

        if !bound_request.headers.is_empty() {
            let size = bound_request.headers.len();
            let mut keys: Vec<&String> = bound_request.headers.keys().collect();
            keys.sort();

            command.push_str(" \\\n");

            for (i, key) in keys.iter().enumerate() {
                let val = bound_request.headers.get(*key).unwrap();

                command.push_str(&*{
                    let mut initial = format!("  -H '{key}: {val}'");

                    if i < size - 1 {
                        initial.push_str(" \\\n");
                    }

                    initial
                });
            }
        }

        // TODO: Add support for multipart and others.
        if let RequestPayload::Raw {
            encoding: _,
            content,
        } = &self.endpoint_data.body
        {
            command.push_str(&*'fmt: {
                let body = String::from_utf8_lossy(content).to_string();
                let value: Result<Value, Error> = serde_json::from_str(body.as_ref());

                if let Err(_) = value {
                    break 'fmt format!("");
                }

                let value = value.unwrap();
                let trimmed_json_str = serde_json::to_string(&value);

                if let Err(_) = trimmed_json_str {
                    break 'fmt format!("");
                }

                let trimmed_json_str = trimmed_json_str.unwrap();
                let trimmed_json_str = trimmed_json_str.replace("'", "\\\\'");

                format!(" \\\n  -d '{}'", trimmed_json_str)
            });
        }

        Ok(command)
    }
}
