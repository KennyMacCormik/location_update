use crate::infatica::internal::models::{InfaticaGeoNodeRecord, InfaticaIspRecord, InfaticaRegionRecord, InfaticaZipRecord};

pub struct InfaticaQueryResults{
	geo_nodes: Vec<InfaticaGeoNodeRecord>,
	region_codes: Vec<InfaticaRegionRecord>,
	zip_codes: Vec<InfaticaZipRecord>,
	isp_codes: Vec<InfaticaIspRecord>,
}

impl InfaticaQueryResults {
	pub fn new(
		geo_nodes: Vec<InfaticaGeoNodeRecord>,
		region_codes: Vec<InfaticaRegionRecord>,
		zip_codes: Vec<InfaticaZipRecord>,
		isp_codes: Vec<InfaticaIspRecord>,
	) -> Self {
		Self {
			geo_nodes,
			region_codes,
			zip_codes,
			isp_codes,
		}
	}

	pub fn geo_nodes(&self) -> &Vec<InfaticaGeoNodeRecord> {
		&self.geo_nodes
	}

	pub fn region_codes(&self) -> &Vec<InfaticaRegionRecord> {
		&self.region_codes
	}

	pub fn zip_codes(&self) -> &Vec<InfaticaZipRecord> {
		&self.zip_codes
	}

	pub fn isp_codes(&self) -> &Vec<InfaticaIspRecord> {
		&self.isp_codes
	}
}