use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};
use std::mem;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use failure::Error;
use num_integer::gcd;
use serde_derive::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::index::storage::{FSStorage, ReadData, Storage, StorageInfo, ToWriter};
use crate::index::{Comparable, DatasetInfo, Index, SigStore};

type Object = u64; // TODO: what is this?!?!
const IP: f64 = 0.001;

#[derive(TypedBuilder)]
pub struct LazoIndex<L> {
    #[builder(default)]
    storage: Option<Rc<dyn Storage>>,

    k: usize, // max_hash for scaled mh
    d: f64,
    fp_rate: f64,
    fn_rate: f64,
    numThresholds: usize,

    gcdSliceSize: usize,
    gcdBands: usize,
    hashTables: Vec<HashMap<u64, HashSet<Object>>>,
    segmentIds: Vec<HashMap<HashSet<Object>, Object>>,
    hashRanges: Vec<usize>,
    keyCardinality: HashMap<Object, u64>,

    thresholdToBandRows: HashMap<usize, (usize, usize)>,

    #[builder(default)]
    pub(crate) datasets: Vec<SigStore<L>>,
}

#[derive(Serialize, Deserialize)]
struct LazoInfo<L> {
    version: u32,
    storage: StorageInfo,
    leaves: Vec<L>,
}

fn getOptimalBandsAndRows(K: usize, num_thresholds: usize) -> (usize, usize) {
    let minError = std::f64::MAX;
    let mut optimalBands = 0;
    let mut optimalRows = 0;

    for band in 1..=K {
        let maximumRows = K / band;
        for rows in 1..=maximumRows {
            let falsePositives = computeFalsePositiveProbability(threshold, band, rows);
            let falseNegatives = computeFalseNegativeProbability(threshold, band, rows);
            let error = fp_rate * falsePositives + fn_rate * falseNegatives;
            if error < minError {
                minError = error;
                optimalBands = band;
                optimalRows = rows;
            }
        }
    }

    (optimalBands, optimalRows)
}

impl<'a, L> Index<'a> for LazoIndex<L>
where
    L: Clone + Comparable<L> + 'a,
    SigStore<L>: From<L>,
{
    type Item = L;
    //type SignatureIterator = std::slice::Iter<'a, Self::Item>;

    fn insert(&mut self, node: L) -> Result<(), Error> {
        unimplemented!()
    }

    fn save<P: AsRef<Path>>(&self, _path: P) -> Result<(), Error> {
        /*
        let file = File::create(path)?;
        match serde_json::to_writer(file, &self) {
            Ok(_) => Ok(()),
            Err(_) => Err(SourmashError::SerdeError.into()),
        }
        */
        unimplemented!()
    }

    fn load<P: AsRef<Path>>(_path: P) -> Result<(), Error> {
        unimplemented!()
    }

    fn signatures(&self) -> Vec<Self::Item> {
        unimplemented!()
    }

    fn signature_refs(&self) -> Vec<&Self::Item> {
        unimplemented!()
    }

    /*
    fn iter_signatures(&'a self) -> Self::SignatureIterator {
        self.datasets.iter()
    }
    */
}

impl<L> LazoIndex<L>
where
    L: ToWriter,
    SigStore<L>: ReadData<L>,
{
    pub fn new(K: usize, D: f64, fp_rate: f64, fn_rate: f64) -> Self {
        let num_thresholds: usize = (1. / D).round() as usize;
        let (bands, rows) = getOptimalBandsAndRows(K, num_thresholds);
        let gcd_band_size = gcd(bands, rows);
        let num_ranges = K / gcd_band_size;
        let hashTables = vec![HashMap::new(); num_ranges];
        let segmentIds = vec![HashMap::new(); num_ranges];
        let hashRanges: Vec<_> = (0..num_ranges).map(|i| i * gcd_band_size).collect();

        LazoIndex {
            hashTables,
            hashRanges,
            segmentIds,
            datasets: Default::default(),
            storage: None,
        }
    }

    pub fn save_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        storage: Option<Rc<dyn Storage>>,
    ) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<LazoIndex<L>, Error> {
        unimplemented!();
    }

    pub fn from_reader<R, P>(rdr: &mut R, path: P) -> Result<LazoIndex<L>, Error>
    where
        R: Read,
        P: AsRef<Path>,
    {
        unimplemented!();
    }

    pub fn storage(&self) -> Option<Rc<dyn Storage>> {
        self.storage.clone()
    }
}
