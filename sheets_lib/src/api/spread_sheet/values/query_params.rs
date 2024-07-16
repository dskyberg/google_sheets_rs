use crate::{DateTimeRenderOptionType, DimensionType, ValueInputOption, ValueRenderOption};

pub struct ValueQueryParams {
    pub major_dimension: Option<DimensionType>,
    pub value_input_option: Option<ValueInputOption>,
    pub include_values_in_response: Option<bool>,
    pub value_render_option: Option<ValueRenderOption>,
    pub response_value_render_option: Option<ValueRenderOption>,
    pub response_date_time_render_option: Option<DateTimeRenderOptionType>,
}

impl Default for ValueQueryParams {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueQueryParams {
    pub fn new() -> Self {
        Self {
            major_dimension: None,
            value_input_option: None,
            include_values_in_response: None,
            value_render_option: None,
            response_value_render_option: None,
            response_date_time_render_option: None,
        }
    }

    pub fn with_major_dimension(mut self, opt: DimensionType) -> Self {
        self.major_dimension = Some(opt);
        self
    }

    pub fn with_value_input_option(mut self, opt: ValueInputOption) -> Self {
        self.value_input_option = Some(opt);
        self
    }

    pub fn with_include_values_in_response(mut self, opt: bool) -> Self {
        self.include_values_in_response = Some(opt);
        self
    }

    pub fn with_value_render_option(mut self, opt: ValueRenderOption) -> Self {
        self.value_render_option = Some(opt);
        self
    }

    pub fn with_response_value_render_option(mut self, opt: ValueRenderOption) -> Self {
        self.response_value_render_option = Some(opt);
        self
    }

    pub fn with_response_date_time_render_option(mut self, opt: DateTimeRenderOptionType) -> Self {
        self.response_date_time_render_option = Some(opt);
        self
    }
}

impl std::fmt::Display for ValueQueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::<String>::new();

        if let Some(opt) = &self.major_dimension {
            params.push(format!("majorDimension={}", opt));
        }
        if let Some(opt) = &self.value_input_option {
            params.push(format!("valueInputOption={}", opt));
        }
        if let Some(opt) = &self.value_render_option {
            params.push(format!("valueRenderOption={}", opt));
        }
        if let Some(opt) = &self.include_values_in_response {
            params.push(format!("includeValuesInResponse={}", opt));
        }
        if let Some(opt) = &self.response_value_render_option {
            params.push(format!("responseValueRenderOption={}", opt));
        }
        if let Some(opt) = &self.response_date_time_render_option {
            params.push(format!("dateTimeRenderOption={}", opt));
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
