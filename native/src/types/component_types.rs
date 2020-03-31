use ::types::base_types::Quote;

#[derive(Deserialize)]
pub struct Input {
	pub draws: Vec<Draw>,
	pub factors: PricingFactor,
	pub prices: DailyPrice,
	pub quote: Quote,
	pub coverage: f64
}

#[derive(Deserialize)]
pub struct Draw {
	#[serde(rename = "month1ButterPriceDraw")]
	pub m1bpd: f64,

	#[serde(rename = "month2ButterPriceDraw")]
	pub m2bpd: f64,

	#[serde(rename = "month3ButterPriceDraw")]
	pub m3bpd: f64,

	#[serde(rename = "month1CheesePriceDraw")]
	pub m1cpd: f64,

	#[serde(rename = "month2CheesePriceDraw")]
	pub m2cpd: f64,

	#[serde(rename = "month3CheesePriceDraw")]
	pub m3cpd: f64,

	#[serde(rename = "month1DryWheyPriceDraw")]
	pub m1dwpd: f64,

	#[serde(rename = "month2DryWheyPriceDraw")]
	pub m2dwpd: f64,

	#[serde(rename = "month3DryWheyPriceDraw")]
	pub m3dwpd: f64,

	#[serde(rename = "yieldDrawQuantity")]
	pub ydq: f64
}

#[derive(Copy, Clone)]
pub struct SimulationValue {
    pub sbfp: f64,
    pub spp: f64,
    pub sosp: f64,
    pub syaf: f64
}

#[derive(Deserialize, Copy, Clone)]
pub struct DailyPrice {

	#[serde(rename = "expectedButterfatPrice")]
	pub ebfp: f64,

	#[serde(rename = "expectedProteinPrice")]
	pub epp: f64,

	#[serde(rename = "expectedOtherSolidsPrice")]
	pub eosp: f64,
	
	#[serde(rename = "monthOneButterSigma")]
	pub m1bs: f64,

	#[serde(rename = "monthOneExpectedButterPrice")]
	pub m1ebp: f64,

	#[serde(rename = "monthTwoButterSigma")]
	pub m2bs: f64,

	#[serde(rename = "monthTwoExpectedButterPrice")]
	pub m2ebp: f64,

	#[serde(rename = "monthThreeButterSigma")]
	pub m3bs: f64,

	#[serde(rename = "monthThreeExpectedButterPrice")]
	pub m3ebp: f64,

	#[serde(rename = "monthOneCheeseSigma")]
	pub m1cs: f64,

	#[serde(rename = "monthOneExpectedCheesePrice")]
	pub m1ecp: f64,

	#[serde(rename = "monthTwoCheeseSigma")]
	pub m2cs: f64,

	#[serde(rename = "monthTwoExpectedCheesePrice")]
	pub m2ecp: f64,

	#[serde(rename = "monthThreeCheeseSigma")]
	pub m3cs: f64,

	#[serde(rename = "monthThreeExpectedCheesePrice")]
	pub m3ecp: f64,

	#[serde(rename = "monthOneDryWheySigma")]
	pub m1dws: f64,

	#[serde(rename = "monthOneExpectedDryWheyPrice")]
	pub m1edwp: f64,

	#[serde(rename = "monthTwoDryWheySigma")]
	pub m2dws: f64,

	#[serde(rename = "monthTwoExpectedDryWheyPrice")]
	pub m2edwp: f64,

	#[serde(rename = "monthThreeDryWheySigma")]
	pub m3dws: f64,

	#[serde(rename = "monthThreeExpectedDryWheyPrice")]
	pub m3edwp: f64,

	#[serde(rename = "expectedYield")]
    pub ey: f64,

    #[serde(rename = "expectedYieldStandardDeviation")]
    pub eysd: f64,

    #[serde(rename = "loadingFactor")]
    pub lf: f64
}

#[derive(Deserialize)]
pub struct PricingFactor {
	#[serde(rename = "butterMakeAllowance")]
	pub bma: f64,

	#[serde(rename = "butterManufacturingYield")]
	pub bmy: f64,

	#[serde(rename = "dryWheyMakeAllowance")]
	pub dwma: f64,

	#[serde(rename = "dryWheyManufacturingYield")]
	pub dwmy: f64,

	#[serde(rename = "cheeseMakeAllowance")]
	pub cma: f64,

	#[serde(rename = "cheeseManufacturingYieldCasein")]
	pub cmyc: f64,

	#[serde(rename = "cheeseManufacturingYieldButterfat")]
	pub cmyb: f64,

	#[serde(rename = "butterfatRetentionRate")]
	pub brr: f64,

	#[serde(rename = "butterfatToProteinRatio")]
	pub btpr: f64
}