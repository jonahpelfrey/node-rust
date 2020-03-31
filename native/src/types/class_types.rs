use ::types::base_types::Quote;

#[derive(Deserialize)]
pub struct Input {
    pub draws: Vec<Draw>,
    pub prices: DailyPrice,
    pub quote: Quote,
    pub coverage: f64
}

#[derive(Deserialize)]
pub struct Draw {
    
    #[serde(rename = "month1ClassiiiPriceDraw")]
    pub m1c3pd: f64,

    #[serde(rename = "month2ClassiiiPriceDraw")]
    pub m2c3pd: f64,

    #[serde(rename = "month3ClassiiiPriceDraw")]
    pub m3c3pd: f64,

    #[serde(rename = "month1ClassivPriceDraw")]
    pub m1c4pd: f64,

    #[serde(rename = "month2ClassivPriceDraw")]
    pub m2c4pd: f64,

    #[serde(rename = "month3ClassivPriceDraw")]
    pub m3c4pd: f64,

    #[serde(rename = "yieldDrawQuantity")]
    pub ydq: f64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SimulationValue {
    #[serde(rename = "simulatedClassThreePrice")]
    pub sc3p: f64,

    #[serde(rename = "simulatedClassFourPrice")]
    pub sc4p: f64,

    #[serde(rename = "simulatedYieldAdjustmentFactor")]
    pub syaf: f64
}

#[derive(Deserialize, Copy, Clone)]
pub struct DailyPrice {

    #[serde(rename = "expectedClassThreePrice")]
    pub ec3p: f64,

    #[serde(rename = "expectedClassFourPrice")]
    pub ec4p: f64,

    #[serde(rename = "loadingFactor")]
    pub lf: f64,

    #[serde(rename = "monthOneClassThreeSigma")]
    pub m1c3s: f64,

    #[serde(rename = "monthOneExpectedClassThreePrice")]
    pub m1ec3p: f64,

    #[serde(rename = "monthTwoClassThreeSigma")]
    pub m2c3s: f64,

    #[serde(rename = "monthTwoExpectedClassThreePrice")]
    pub m2ec3p: f64,

    #[serde(rename = "monthThreeClassThreeSigma")]
    pub m3c3s: f64,

    #[serde(rename = "monthThreeExpectedClassThreePrice")]
    pub m3ec3p: f64,

    #[serde(rename = "monthOneClassFourSigma")]
    pub m1c4s: f64,

    #[serde(rename = "monthOneExpectedClassFourPrice")]
    pub m1ec4p: f64,

    #[serde(rename = "monthTwoClassFourSigma")]
    pub m2c4s: f64,

    #[serde(rename = "monthTwoExpectedClassFourPrice")]
    pub m2ec4p: f64,

    #[serde(rename = "monthThreeClassFourSigma")]
    pub m3c4s: f64,

    #[serde(rename = "monthThreeExpectedClassFourPrice")]
    pub m3ec4p: f64,

    #[serde(rename = "expectedYield")]
    pub ey: f64,

    #[serde(rename = "expectedYieldStandardDeviation")]
    pub eysd: f64
}