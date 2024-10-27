use crate::entities::EndpointData;

pub struct CurlService {
    endpoint_data: EndpointData,
}

impl CurlService {
    pub fn new(endpoint_data: EndpointData) -> Self {
        Self { endpoint_data }
    }

    pub fn generate(&self) -> String {
        format!("{:?}", self.endpoint_data)
    }
}
