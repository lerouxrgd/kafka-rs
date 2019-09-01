# kafka-protocol

Rust types and serde based codecs for `Kafka` binary protocol.

## Compatibility

### Produce

| API Version | Records type   | Kafka version |
|-------------|----------------|---------------|
| V0          | MessageSet::V0 | < 0.10.0      |
| V1          | MessageSet::V0 | < 0.10.0      |
| V2          | MessageSet::V1 | >= 0.10       |
| >= V3       | RecordBatch    | >= 0.11       |

### Fetch

| API version | Records type   | Kafka version |
|-------------|----------------|---------------|
| V0          | MessageSet::V0 | < 0.10.0      |
| V1          | MessageSet::V0 | < 0.10.0      |
| V2          | MessageSet::V1 | = 0.10.0      |
| V3          | MessageSet::V1 | = 0.10.2      |
| >= V4       | RecordBatch    | >= 0.11       |
