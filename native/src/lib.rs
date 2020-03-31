// #[macro_use]
extern crate neon;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate neon_serde;

extern crate serde_json;

use neon::prelude::*;
use std::time::{Instant};

mod common;
mod types;
mod endorsements;

use types::base_types::{Output};
use types::component_types;
use types::class_types;

use endorsements::class_endorsement;
use endorsements::component_endorsement;


export! {
	fn fetch_class_endorsements(e: class_types::Input) -> Output {
		let run_time = Instant::now();
		let endorsements = class_endorsement::get_all_endorsements(e.prices, e.quote, e.draws);
		let elapsed = run_time.elapsed();
    	let ms = elapsed.subsec_millis();
    	let ns = elapsed.subsec_nanos();

		Output {
			endorsements: endorsements,
			performance: format!("Runtime: {:?} | {}ms | {}ns", elapsed, ms, ns)
		}
	}

	fn fetch_class_endorsement(e: class_types::Input) -> Output {
		let run_time = Instant::now();
		let endorsement = class_endorsement::get_endorsement(e.prices, e.quote, e.draws, e.coverage);
		let elapsed = run_time.elapsed();
    	let ms = elapsed.subsec_millis();
    	let ns = elapsed.subsec_nanos();

		Output {
			endorsements: vec![endorsement],
			performance: format!("Runtime: {:?} | {}ms | {}ns", elapsed, ms, ns)
		}
	}

	fn fetch_component_endorsements(e: component_types::Input) -> Output {
		let run_time = Instant::now();
		let endorsements = component_endorsement::get_all_endorsements(e.prices,e.quote, e.draws,e.factors);
		let elapsed = run_time.elapsed();
    	let ms = elapsed.subsec_millis();
    	let ns = elapsed.subsec_nanos();

		Output {
			endorsements: endorsements,
			performance: format!("Runtime: {:?} | {}ms | {}ns", elapsed, ms, ns)
		}
	}

	fn fetch_component_endorsement(e: component_types::Input) -> Output {
		let run_time = Instant::now();
		let endorsement = component_endorsement::get_endorsement(e.prices, e.quote, e.draws, e.factors, e.coverage);
		let elapsed = run_time.elapsed();
    	let ms = elapsed.subsec_millis();
    	let ns = elapsed.subsec_nanos();

		Output {
			endorsements: vec![endorsement],
			performance: format!("Runtime: {:?} | {}ms | {}ns", elapsed, ms, ns)
		}
	}
}
