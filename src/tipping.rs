pub struct TippingRequest<'a> {
    pub api_key: &'a str,
    pub account_handle: &'a str,
    pub amount: f32,
    pub currency: &'a str,
    pub description: &'a str,
    pub environment: &'a str,
    pub api_version: &'a str,
    pub correlation_id: &'a str,
}

impl<'a, T> From<(T, T, f32, T)> for TippingRequest<'a>
where
    T: Into<&'a str>,
{
    fn from((api_key, account_handle, amount, currency): (T, T, f32, T)) -> Self {
        TippingRequest {
            api_key: api_key.into(),
            account_handle: account_handle.into(),
            amount: amount,
            currency: currency.into(),
            description: "Tip",
            environment: "api.strike.me",
            api_version: "v1",
            correlation_id: "",
        }
    }
}

impl<'a, T> From<(T, T, f32, T, T)> for TippingRequest<'a>
where
    T: Into<&'a str>,
{
    fn from((api_key, account_handle, amount, currency, description): (T, T, f32, T, T)) -> Self {
        TippingRequest {
            api_key: api_key.into(),
            account_handle: account_handle.into(),
            amount: amount,
            currency: currency.into(),
            description: description.into(),
            environment: "api.strike.me",
            api_version: "v1",
            correlation_id: "",
        }
    }
}

//Do you need to implement the From for the default constructor?
impl<'a, T> From<(T, T, f32, T, T, T, T, T)> for TippingRequest<'a>
where
    T: Into<&'a str>,
{
    fn from(
        (
            api_key,
            account_handle,
            amount,
            currency,
            description,
            environment,
            api_version,
            correlation_id,
        ): (T, T, f32, T, T, T, T, T),
    ) -> Self {
        TippingRequest {
            api_key: api_key.into(),
            account_handle: account_handle.into(),
            amount: amount,
            currency: currency.into(),
            description: description.into(),
            environment: environment.into(),
            api_version: api_version.into(),
            correlation_id: correlation_id.into(),
        }
    }
}
