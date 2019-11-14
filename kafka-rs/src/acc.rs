use kafka_protocol::codec::Compression;
use kafka_protocol::types::*;

pub struct RecAcc {
    size_limit: usize,
    compression: Compression,
    cr_estimate: f64,
    rbb: RecordBatchBuilder,
}

impl RecAcc {
    pub fn new(size_limit: usize) -> Self {
        RecAcc {
            size_limit,
            compression: Compression::None,
            cr_estimate: 1.0,
            rbb: RecordBatchBuilder::new(),
        }
    }

    pub fn set_cr_estimate(&mut self, cr_estimate: f64) {
        self.cr_estimate = cr_estimate;
    }

    pub fn has_room_for(&self, rec: RecData) -> bool {
        let estimate = self.estime_size();

        if estimate >= self.size_limit {
            false
        } else {
            (estimate + rec.size() + Varlong::MAX_SIZE) <= self.size_limit
        }
    }

    fn estime_size(&self) -> usize {
        static CR_ESTIMATION_FACTOR: f64 = 1.05;

        let records_size: usize = self.rbb.records_size();
        match self.compression {
            Compression::None => RecordBatch::OVERHEAD_SIZE + records_size,
            _ => {
                RecordBatch::OVERHEAD_SIZE
                    + records_size * (self.cr_estimate * CR_ESTIMATION_FACTOR) as usize
            }
        }
    }
}
