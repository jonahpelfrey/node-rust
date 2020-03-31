use ::common::helpers::{calculate_simulated_price, to_fixed, norms_inv, subsidy_percent};
use ::types::component_types::{DailyPrice, Draw, PricingFactor, SimulationValue};
use ::types::base_types::{Endorsement, Quote};

pub fn get_all_endorsements(prices: DailyPrice, quote: Quote, draws: Vec<Draw>, factors: PricingFactor) -> Vec<Endorsement> {
	let simulation_values = create_simulation_values(draws, prices, factors);
	let mut endorsements = Vec::with_capacity(4);
	
	for coverage_level in [0.80, 0.85, 0.90, 0.95].iter() {
		endorsements.push(calculate_endorsement(prices, quote, *coverage_level, &simulation_values));
	}

	return endorsements;
}

pub fn get_endorsement(prices: DailyPrice, quote: Quote, draws: Vec<Draw>, factors: PricingFactor, coverage_level: f64) -> Endorsement {
	let simulation_values = create_simulation_values(draws, prices, factors);
	return calculate_endorsement(prices, quote, coverage_level, &simulation_values);
}

fn calculate_endorsement(prices: DailyPrice, quote: Quote, coverage_level: f64, simulation_values: &[SimulationValue]) -> Endorsement {
	
	let mut _sl = [0.0; 5000];

	// Inputs
    let _ebp: f64 = prices.ebfp;
    let _epp: f64 = prices.epp;
    let _eosp: f64 = prices.eosp;
    let _lf: f64 = prices.lf;
    let _dbt: f64 = quote.dbt;
    let _dpt: f64 = quote.dpt;
    let _dp: f64 = quote.dp;
    let _protection: f64 = quote.protection;

    // Weighted Prices
    let _webr: f64 = _ebp * _dbt;
    let _wepr: f64 = _epp * _dpt;
    let _weosr: f64 = _eosp * 5.7;
    let _wp: f64 = _webr + _wepr + _weosr;

     // Expected Revenue Amount
    let _era: f64 = {
        let x = (_wp * _dp) / 100.00;
        x.round()
    };

    // Revenue Guarantee
    let rg: f64 = (_era * coverage_level).round();

    // Coverage Price
    let cp: f64 = _wp * coverage_level;

    for (i, element) in simulation_values.iter().enumerate() {
    	// Weighted Component Price
        let wcp: f64 = {
            let x: f64 = element.sbfp * _dbt;
            let y: f64 = element.spp * _dpt;
            let z: f64 = element.sosp * 5.7;
            x + y + z
        };

        // Effective Milk Production
        //let emp: f64 = 1000000.0 * element.syaf;
        let emp: f64 = _dp * element.syaf;

        // Simulated Revenue Amount
        let sra: f64 = ((wcp * emp) / 100.0).round();

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

fn create_simulation_values(draws: Vec<Draw>, prices: DailyPrice, factors: PricingFactor) -> Vec<SimulationValue> {
	let mut _sm1bpd = [0.0; 5000];
	let mut _sm2bpd = [0.0; 5000];
	let mut _sm3bpd = [0.0; 5000];

	let mut _sm1cpd = [0.0; 5000];
	let mut _sm2cpd = [0.0; 5000];
	let mut _sm3cpd = [0.0; 5000];

	let mut _sm1dwpd = [0.0; 5000];
	let mut _sm2dwpd = [0.0; 5000];
	let mut _sm3dwpd = [0.0; 5000];

	let mut _cache: Vec<SimulationValue> = Vec::new();

	for i in 0..5000 {
		_sm1bpd[i] = draws[i].m1bpd;
		_sm2bpd[i] = draws[i].m2bpd;
		_sm3bpd[i] = draws[i].m3bpd;

		_sm1cpd[i] = draws[i].m1cpd;
		_sm2cpd[i] = draws[i].m2cpd;
		_sm3cpd[i] = draws[i].m3cpd;

		_sm1dwpd[i] = draws[i].m1dwpd;
		_sm2dwpd[i] = draws[i].m2dwpd;
		_sm3dwpd[i] = draws[i].m3dwpd;
	}

	for i in 0..5000 {

		//Butter
		let sm1bp = calculate_simulated_price(_sm1bpd[i], prices.m1bs, prices.m1ebp);
		let sm2bp = calculate_simulated_price(_sm2bpd[i], prices.m2bs, prices.m2ebp);
		let sm3bp = calculate_simulated_price(_sm3bpd[i], prices.m3bs, prices.m3ebp);

		//Cheese
		let sm1cp = calculate_simulated_price(_sm1cpd[i], prices.m1cs, prices.m1ecp);
		let sm2cp = calculate_simulated_price(_sm2cpd[i], prices.m2cs, prices.m2ecp);
		let sm3cp = calculate_simulated_price(_sm3cpd[i], prices.m3cs, prices.m3ecp);

		//Dry Whey
		let sm1dwp = calculate_simulated_price(_sm1dwpd[i], prices.m1dws, prices.m1edwp);
		let sm2dwp = calculate_simulated_price(_sm2dwpd[i], prices.m2dws, prices.m2edwp);
		let sm3dwp = calculate_simulated_price(_sm3dwpd[i], prices.m3dws, prices.m3edwp);

		//Butterfat
		let sm1bfp = {
			let x = (sm1bp - factors.bma) * factors.bmy;
			to_fixed(x, 4)
		};

		let sm2bfp = {
			let x = (sm2bp - factors.bma) * factors.bmy;
			to_fixed(x, 4)
		};

		let sm3bfp = {
			let x = (sm3bp - factors.bma) * factors.bmy;
			to_fixed(x, 4)
		};

		let sbfp = {
			let x = (sm1bfp + sm2bfp + sm3bfp) / 3.0;
			to_fixed(x, 2)
		};

		//Other Solids
		let sm1osp = {
			let x = (sm1dwp - factors.dwma) * factors.dwmy;
			to_fixed(x, 4)
		};

		let sm2osp = {
			let x = (sm2dwp - factors.dwma) * factors.dwmy;
			to_fixed(x, 4)
		};

		let sm3osp = {
			let x = (sm3dwp - factors.dwma) * factors.dwmy;
			to_fixed(x, 4)
		};

		let sosp = {
			let x = (sm1osp + sm2osp + sm3osp) / 3.0;
			to_fixed(x, 2)
		};

		//Protein
		let sm1pp = {
			let a = (sm1cp - factors.cma) * factors.cmyc;
			let pc = to_fixed(a, 4);

			let b = (sm1cp - factors.cma) * factors.cmyb;
			let c = to_fixed(b, 4);
			let d = sm1bfp * factors.brr;
			let f = to_fixed(d, 4);
			let bc = (c - f) * factors.btpr;
		
			let x = pc + bc;
			to_fixed(x, 4)
		};

		let sm2pp = {
			let a = (sm2cp - factors.cma) * factors.cmyc;
			let pc = to_fixed(a, 4);

			let b = (sm2cp - factors.cma) * factors.cmyb;
			let c = to_fixed(b, 4);
			let d = sm2bfp * factors.brr;
			let f = to_fixed(d, 4);
			let bc = (c - f) * factors.btpr;

			let x = pc + bc;
			to_fixed(x, 4)
		};

		let sm3pp = {
			let a = (sm3cp - factors.cma) * factors.cmyc;
			let pc = to_fixed(a, 4);

			let b = (sm3cp - factors.cma) * factors.cmyb;
			let c = to_fixed(b, 4);
			let d = sm3bfp * factors.brr;
			let f = to_fixed(d, 4);
			let bc = (c - f) *  factors.btpr;

			let x = pc + bc;
			to_fixed(x, 4)
		};

		let spp = {
			let x = (sm1pp + sm2pp + sm3pp) / 3.0;
			to_fixed(x, 2)
		};

		//Simulated Yield Adjustment
		let syaf = {
            let x = (norms_inv(draws[i].ydq, 0.0, 1.0) * prices.eysd) + prices.ey;
            let y = (to_fixed(x, 4)) / prices.ey;
            to_fixed(y, 4)
        };

		_cache.push(SimulationValue {
			sbfp: sbfp,
			sosp: sosp,
			spp: spp,
			syaf: syaf
		})
	}

	return _cache
}