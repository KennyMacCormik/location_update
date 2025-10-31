use crate::infatica::errors::InfaticaQueryError;
use crate::infatica::internal::geo_nodes::geo_nodes;
use crate::infatica::internal::isp_codes::isp_codes;
use crate::infatica::internal::region_codes::region_codes;
use crate::infatica::internal::zip_codes::zip_codes;
use crate::infatica::models::InfaticaQueryResults;
use crate::models::InfaticaConfig;

/// Executes **all four Infatica queries concurrently**.
///
/// ### Behavior
/// - Runs [`geo_nodes`], [`region_codes`], [`zip_codes`], and [`isp_codes`] using [`tokio::join!`].
/// - Aggregates all encountered errors into a single `Vec<InfaticaQueryError>`.
/// - If any query fails, returns `Err(Vec<...>)` containing **all** errors (no early return).
/// - If all succeed, returns [`InfaticaQueryResults`] containing the fetched datasets.
///
/// ### Errors
/// - Network failures, deserialization errors, or invalid URLs bubble up through [`HTTPError`].
///
/// ### Example
/// ```no_run
/// # use crate::infatica::get_all;
/// # use crate::models::InfaticaConfig;
/// # async fn example(cfg: InfaticaConfig) {
/// match get_all(&cfg).await {
///     Ok(results) => println!("Fetched {} geo-nodes", results.geo_nodes().len()),
///     Err(errors) => eprintln!("Some queries failed: {:?}", errors),
/// }
/// # }
/// ```
pub async fn get_all(cfg: &InfaticaConfig) -> Result<InfaticaQueryResults, Vec<InfaticaQueryError>>{
	// Run all endpoint calls concurrently.
	let (
		geo_res,
		region_res,
		zip_res,
		isp_res,
	) = tokio::join!(
        geo_nodes(cfg),
        region_codes(cfg),
        zip_codes(cfg),
        isp_codes(cfg),
    );

	let mut errors = Vec::new();

	// Holders for successful data
	let (
		geo_nodes,
		region_codes,
		zip_codes,
		isp_codes,
	) = {
		let mut g = Vec::new();
		let mut r = Vec::new();
		let mut z = Vec::new();
		let mut i = Vec::new();

		match geo_res {
			Ok(v) => g = v,
			Err(e) => errors.push(InfaticaQueryError::GeoNodes(e)),
		}

		match region_res {
			Ok(v) => r = v,
			Err(e) => errors.push(InfaticaQueryError::RegionCodes(e)),
		}

		match zip_res {
			Ok(v) => z = v,
			Err(e) => errors.push(InfaticaQueryError::ZipCodes(e)),
		}

		match isp_res {
			Ok(v) => i = v,
			Err(e) => errors.push(InfaticaQueryError::IspCodes(e)),
		}

		(g, r, z, i)
	};

	// If any of the four failed, propagate all failures together.
	if !errors.is_empty() {
		return Err(errors);
	}

	// Otherwise, all succeeded — return a grouped result.
	Ok(
		InfaticaQueryResults::new(
			geo_nodes,
			region_codes,
			zip_codes,
			isp_codes,
		)
	)
}