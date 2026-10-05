# Quote TaxJar contracts

This focused lane compiles the mounted `api/quotes.rs::create_quote` handler,
its tenant/reference validation, the optional TaxJar configuration helper, and
the actual TaxJar client/provider. Tests use isolated PostgreSQL schemas and a
loopback HTTP fixture with public synthetic credentials; no TaxJar account or
external API is used. Each
fixture holds a shared environment lock while selecting its local provider
origin, and restores the previous environment even on assertion failure.

Money remains integer cents in application structs. Only the TaxJar JSON wire
boundary uses decimal dollars. The accepted tax range is zero through
`92233720368547758.07` dollars (`i64::MAX` cents). Negative values, nonzero
fractional cents, unsupported decimal precision, nonnumeric values and overflow
are provider errors. No rounding or saturating cast is performed. Extra trailing
zeroes within the decimal representation's supported precision are accepted.
Tax rates remain separate approximate `f64` values and are never used to derive
cent amounts. Existing optional-provider-error behavior is retained: a failed
calculation adds no automatic tax line. Tax jurisdiction/default inputs and
required-deposit policy are outside this change.

The focused tests cover saved 0.29/0.57/1.13/8.03 tax amounts, exponent notation,
large outbound numeric JSON, repeated line-item multiplication, checked sum and
product overflow, invalid responses, provider failure, and tax-total overflow.
The existing ignored PostgreSQL configuration regressions are retained.

The adapter pins `rust_decimal` 1.42.1 with default features disabled and only
`std`. It reuses the workspace's existing `serde_json/raw_value` feature to
retain original JSON numeric text without enabling global
`serde_json/arbitrary_precision`, `rust_decimal/rkyv`, or database features.
`rust_decimal` 1.42.1 is MIT, specifies MSRV 1.67.1, and fits Rust 1.95.

Primary references:

- [Decimal exact parsing and checked operations](https://docs.rs/rust_decimal/1.42.1/rust_decimal/struct.Decimal.html)
- [Scientific parser source](https://docs.rs/rust_decimal/1.42.1/src/rust_decimal/decimal.rs.html)
- [RawValue preserves JSON text](https://docs.rs/serde_json/latest/serde_json/value/struct.RawValue.html)
- [TaxJar tax calculation contract](https://developers.taxjar.com/api/reference/#post-calculate-sales-tax-for-an-order)
- [rust_decimal MIT license](https://github.com/paupino/rust-decimal/blob/master/LICENSE)

These focused results do not replace repository `make lint` / `make test`,
quote acceptance receipts, full server compilation, or provider sandbox tests.

The Cargo build script uses the shared `ohc-rust-source-extract` parser, selects
original structs/impl/functions including attributes, and writes source and
selection SHA-256 proofs to `OUT_DIR/source-manifest.json`. Cargo watches the
original source files, so editing the mounted handler regenerates the compiled
harness. The runtime standalone-mode function is fixed to hosted mode; its
configuration helper is compiled unchanged and its existing tests separately
cover standalone behavior.

To execute all tests (including the pre-existing explicitly guarded database
cases), supply a disposable local PostgreSQL database matching their guard:

```sh
OHC_QUOTE_TEST_DATABASE_URL=postgres://postgres@127.0.0.1:5432/ohc_quote_lookup_test \
TAXJAR_API_KEY=local-regression-taxjar-key \
cargo test --locked --manifest-path scripts/quote-taxjar/Cargo.toml -- --include-ignored --test-threads=1
```
