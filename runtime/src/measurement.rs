use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::slice;

#[repr(C)]
pub struct RawMeasurementResult {
    pub shots: u64,
    pub result_size: u64,
    pub measurements: *mut MeasurementResultEntry,
}

#[derive(Clone, Debug)]
pub struct MeasurementResult {
    pub shots: u64,
    pub measurements: Vec<MeasurementResultEntry>,
}

impl MeasurementResult {
    pub unsafe fn into_raw(self) -> RawMeasurementResult {
        let raw = RawMeasurementResult {
            shots: self.shots,
            result_size: self.measurements.len() as u64,
            measurements: self.measurements.as_ptr() as *mut MeasurementResultEntry,
        };
        std::mem::forget(self);
        raw
    }
}

impl From<RawMeasurementResult> for MeasurementResult {
    fn from(raw: RawMeasurementResult) -> Self {
        Self {
            shots: raw.shots,
            measurements: unsafe {
                let mut measurements = slice::from_raw_parts(
                    raw.measurements, raw.result_size as usize
                ).to_vec();
                let mut measurements = measurements.into_iter()
                    .filter(|entry| entry.count > 0)
                    .collect::<Vec<_>>();
                measurements.sort_by_key(|entry| entry.value);
                measurements
            },
        }
    }
}

impl From<MeasurementResult> for RawMeasurementResult {
    fn from(measurement: MeasurementResult) -> Self {
        Self {
            shots: measurement.shots,
            result_size: measurement.measurements.len() as u64,
            measurements: measurement.measurements.as_ptr() as *mut MeasurementResultEntry,
        }
    }
}

impl Display for MeasurementResult {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.measurements.iter()
            .map(|entry| (entry.value, entry.count))
            .collect::<BTreeMap<u64, u64>>()
            .fmt(formatter)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MeasurementResultEntry {
    pub value: u64,
    pub count: u64,
}
