/// Ethcore definition of a KeyValueDB with embeeded metrics
pub trait KeyValueDB: kvdb::KeyValueDB + stats::PrometheusMetrics {}
