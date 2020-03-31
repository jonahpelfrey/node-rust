#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct Endorsement {
	pub subsidy: f64,
	pub netPremium: f64,
	pub grossPremium: f64,
	pub protectedPrice: f64,
	pub level: f64,
	pub revenueGuarantee: f64,
	pub liability: f64
}

#[derive(Deserialize, Copy, Clone)]
pub struct Quote {
    #[serde(rename = "classWeight")]
    pub cw: f64,

    #[serde(rename = "butterfat")]
    pub dbt: f64,

    #[serde(rename = "protein")]
    pub dpt: f64,

    #[serde(rename = "production")]
    pub dp: f64,

    #[serde(rename = "protection")]
    pub protection: f64
}

#[derive(Serialize)]
pub struct Output {
	pub performance: String,
	pub endorsements: Vec<Endorsement>
}