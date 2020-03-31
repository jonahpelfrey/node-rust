use ::common::helpers::{calculate_simulated_price, to_fixed, norms_inv, subsidy_percent};
use ::types::class_types::{Draw, DailyPrice, SimulationValue};
use ::types::base_types::{Endorsement, Quote};

pub fn get_all_endorsements(prices: DailyPrice, quote: Quote, draws: Vec<Draw>) -> Vec<Endorsement> {
	let simulation_values = create_simulation_values(draws, prices);
	let mut endorsements = Vec::with_capacity(4);

	for coverage_level in [0.80, 0.85, 0.90, 0.95].iter() {
		endorsements.push(calculate_endorsement(prices, quote, &simulation_values, *coverage_level));
	}

	return endorsements;
}

pub fn get_endorsement(prices: DailyPrice, quote: Quote, draws: Vec<Draw>, coverage_level: f64) -> Endorsement {
	let simulation_values = create_simulation_values(draws, prices);
	return calculate_endorsement(prices, quote, &simulation_values, coverage_level);
}

fn calculate_endorsement(prices: DailyPrice, quote: Quote, simulation_values: &[SimulationValue], coverage_level: f64) -> Endorsement {
	
	let mut _sl = [0.0; 5000];

	// Inputs
    let _ec3: f64 = prices.ec3p;
    let _ec4: f64 = prices.ec4p;
    let _lf: f64 = prices.lf;
    let _cw: f64 = quote.cw;

    let _dp: f64 = quote.dp;
    let _protection: f64 = quote.protection;

    // Weighted Prices
    let _wc3p: f64 = _ec3 * _cw;
    let _wc4p: f64 = _ec4 * (1.0 - _cw);
    let _wp: f64 = _wc3p + _wc4p;

    // Expected Revenue Amount
    let _era: f64 = {
        let x = (_wp * _dp) / 100.00;
        x.round()
    };

    // Revenue Guarantee
    let rg: f64 = (_era * coverage_level).round();

    // Coverage Price
    let cp: f64 = _wp * coverage_level;

    // Simulation Values 
    for (i, element) in simulation_values.iter().enumerate() {

        let wc3p = element.sc3p * _cw;
        let wc4p = element.sc4p * (1.0 - _cw);
        let sy = _dp * element.syaf;

        // Simulated Revenue Amount
        let sra: f64 = {
            let x = (wc3p + wc4p) * sy;
            (x / 100.0).round()
        };

        // Calculated Loss
        let cl = rg - sra;

        // Update Simulated Loss
        _sl[i] = if cl > 0.0 {
            cl
        } else { 0.0 }
    }

    // Average Simulated Loss
    let asl: f64 = {
        let total = _sl.iter().fold(0.0, |acc, x| acc + x);
        (total as f64 / 5000.0)
    };

    // Premium Floor
    let pf: f64 = (_dp * 0.02) / 100.0;

    // Simulated Loss Average
    let sla = if pf > asl {
        pf
    } else { asl };

    // Preliminary Total Premium
    let ptp: f64 = (sla * _protection).round();

    // Total Premium Amount
    let tpa: f64 = (ptp * _lf).round();

    // Liability
    let _lby = rg * _protection;

    // Subsidy Percent
    let sp: f64 = subsidy_percent(coverage_level);

    // Subsidy Amount
    let sa: f64 = (tpa * sp).round();

    // Producer Premium Amount
    let ppa: f64 = {
        let tmp: f64 = {
            let x: f64 = (tpa - sa).round();
            if x > 1.0 { x }
            else { 1.0 }
        };

        (tmp / 10000.0)
    };

    // Scale Premiums
    let tpa: f64 = tpa / 10000.0;
    let sa: f64 = sa / 10000.0;

    return Endorsement {
        netPremium: ppa,
        subsidy: sa,
        grossPremium: tpa,
        protectedPrice: cp,
        level: coverage_level,
        revenueGuarantee: rg,
        liability: _lby
    }

}

fn create_simulation_values(draws: Vec<Draw>, prices: DailyPrice) -> Vec<SimulationValue> {

    let mut _sm1c3d = [0.0; 5000];
    let mut _sm2c3d = [0.0; 5000];
    let mut _sm3c3d = [0.0; 5000];

    let mut _sm1c4d = [0.0; 5000];
    let mut _sm2c4d = [0.0; 5000];
    let mut _sm3c4d = [0.0; 5000];

    let mut _cache: Vec<SimulationValue> = Vec::new();

    for i in 0..5000 {
        _sm1c3d[i] = draws[i].m1c3pd;
        _sm2c3d[i] = draws[i].m2c3pd;
        _sm3c3d[i] = draws[i].m3c3pd;

        _sm1c4d[i] = draws[i].m1c4pd;
        _sm2c4d[i] = draws[i].m2c4pd;
        _sm3c4d[i] = draws[i].m3c4pd;
    }

    for i in 0..5000 {
        let sm1c3p = calculate_simulated_price(_sm1c3d[i], prices.m1c3s, prices.m1ec3p);
        let sm2c3p = calculate_simulated_price(_sm2c3d[i], prices.m2c3s, prices.m2ec3p);
        let sm3c3p = calculate_simulated_price(_sm3c3d[i], prices.m3c3s, prices.m3ec3p);
        let sc3p: f64 = {
            let x = (sm1c3p + sm2c3p + sm3c3p) / 3.0;
            to_fixed(x, 2)
        };

        let sm1c4p = calculate_simulated_price(_sm1c4d[i], prices.m1c4s, prices.m1ec4p);
        let sm2c4p = calculate_simulated_price(_sm2c4d[i], prices.m2c4s, prices.m2ec4p);
        let sm3c4p = calculate_simulated_price(_sm3c4d[i], prices.m3c4s, prices.m3ec4p);
        let sc4p: f64 = {
            let x = (sm1c4p + sm2c4p + sm3c4p) / 3.0;
            to_fixed(x, 2)
        };

        let syaf = {
            let x = (norms_inv(draws[i].ydq, 0.0, 1.0) * prices.eysd) + prices.ey;
            let y = (to_fixed(x, 4)) / prices.ey;
            to_fixed(y, 4)
        };

        _cache.push(SimulationValue {
            sc3p: sc3p,
            sc4p: sc4p,
            syaf: syaf
        })
    }

    return _cache
}
