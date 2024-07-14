use crate::{ResponseValueRenderOption, ValueInputOption};

pub struct ValueQueryParams {
    pub value_input_option: Option<ValueInputOption>,
    pub response_value_render_option: Option<ResponseValueRenderOption>,
}

impl Default for ValueQueryParams {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueQueryParams {
    pub fn new() -> Self {
        Self {
            value_input_option: Some(ValueInputOption::default()),
            response_value_render_option: Some(ResponseValueRenderOption::default()),
        }
    }
}
impl std::fmt::Display for ValueQueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::<String>::new();

        if let Some(vio) = &self.value_input_option {
            params.push(vio.to_query());
        }
        if let Some(vro) = &self.response_value_render_option {
            params.push(vro.to_query());
        }
        match params.is_empty() {
            true => write!(f, ""),
            false => write!(f, "?{}", params.join("&")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params() {
        let params = ValueQueryParams::default();
        println!("{}", &params);
    }
}
