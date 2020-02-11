use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Element {
    #[serde(rename = "Element")]
    pub(crate) name: String,

    #[serde(rename = "Symbol")]
    pub(crate) symbol: String,

    #[serde(rename = "AtomicNumber")]
    pub(crate) atomic_num: u8,

    #[serde(rename = "AtomicMass")]
    pub(crate) mass_per_mole: f64,

    #[serde(rename = "NumberofNeutrons")]
    pub(crate) number_neutrons: u8,

    #[serde(rename = "NumberofProtons")]
    pub(crate) number_protons: u8,

    #[serde(rename = "NumberofElectrons")]
    pub(crate) number_electrons: u8,

    #[serde(rename = "Period")]
    pub(crate) period: u8,

    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "Group")]
    pub(crate) group: Option<u8>,

    #[serde(rename = "Phase")]
    pub(crate) phase: String,

    #[serde(rename = "Radioactive")]
    pub(crate)radioactive: String,

    #[serde(rename = "Natural")]
    pub(crate) natural: String,

    #[serde(rename = "Metal")]
    pub(crate) metal: String,

    #[serde(rename = "Nonmetal")]
    pub(crate) nonmetal: String,

    #[serde(rename = "Metalloid")]
    pub(crate) metalloid: String,

    #[serde(rename = "Type")]
    pub(crate) elem_type: String,

    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "AtomicRadius")]
    pub(crate) atomic_radius: Option<f64>,

    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "Electronegativity")]
    pub(crate)electronegativity: Option<f64>,

    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "FirstIonization")]
    pub(crate) first_ionization: Option<f64>,

    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "Density")]
    pub(crate) density: Option<f64>,

    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "MeltingPoint")]
    pub(crate) melting: Option<f64>,

    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "BoilingPoint")]
    pub(crate) boiling: Option<f64>,

    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "NumberOfIsotopes")]
    pub(crate) number_isotopes: Option<u8>,

    #[serde(rename = "Discoverer")]
    pub(crate)discoverer: String,

    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "Year")]
    pub(crate)year: Option<u16>,

    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "SpecificHeat")]
    pub(crate)specific_heat: Option<f64>,

    #[serde(rename = "NumberofShells")]
    pub(crate)number_shells: u8,

    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "NumberofValence")]
    pub(crate) number_valence: Option<u8>,
}

