use server_integrations_taxjar::client::TaxRate;

#[test]
fn dto_rejects_negative_or_fractional_cents() {
    for amount in [
        "-0.29",
        "0.291",
        "0.29000000000000000000000000001",
        "2.9000000000000000000000000001e-1",
        "1e-29",
    ] {
        let json = format!(r#"{{"amount_to_collect":{amount},"rate":0.0725}}"#);
        assert!(
            serde_json::from_str::<TaxRate>(&json).is_err(),
            "amount {amount} must be rejected"
        );
    }
}

#[test]
fn dto_rejects_cent_overflow() {
    for amount in ["92233720368547758.08", "1e100"] {
        let json = format!(r#"{{"amount_to_collect":{amount},"rate":0.0725}}"#);
        assert!(
            serde_json::from_str::<TaxRate>(&json).is_err(),
            "amount {amount} must be rejected"
        );
    }
}

#[test]
fn dto_round_trip_preserves_exact_maximum_cents() {
    let tax: TaxRate =
        serde_json::from_str(r#"{"amount_to_collect":92233720368547758.07,"rate":0.0725}"#)
            .unwrap();
    assert_eq!(tax.rate, 0.0725);
    let json = serde_json::to_string(&tax).unwrap();
    #[derive(serde::Deserialize)]
    struct Wire {
        amount_to_collect: Box<serde_json::value::RawValue>,
    }
    let wire: Wire = serde_json::from_str(&json).unwrap();
    assert_eq!(wire.amount_to_collect.get(), "92233720368547758.07");
}

#[test]
fn dto_accepts_exact_numeric_dollars_and_keeps_rate_independent() {
    for (amount, cents) in [
        ("0", 0),
        ("0.00", 0),
        ("0.2900", 29),
        ("0.57", 57),
        ("1.13", 113),
        ("8.03", 803),
        ("29e-2", 29),
        ("2.9E-1", 29),
        ("2.9e+1", 2900),
        ("92233720368547758.07", i64::MAX),
    ] {
        let json = format!(
            r#"{{"amount_to_collect":{amount},"rate":0.0725,"unneeded":"provider field"}}"#
        );
        let tax: TaxRate = serde_json::from_str(&json).unwrap();
        assert_eq!(tax.amount_to_collect_cents, cents, "amount {amount}");
        assert_eq!(tax.rate, 0.0725);
    }
}

#[test]
fn dto_requires_numeric_money() {
    for json in [
        r#"{"rate":0.0725}"#,
        r#"{"amount_to_collect":null}"#,
        r#"{"amount_to_collect":"0.29"}"#,
        r#"{"amount_to_collect":true}"#,
    ] {
        assert!(serde_json::from_str::<TaxRate>(json).is_err());
    }
}

#[test]
fn request_serialization_uses_numeric_dollars_without_losing_cents() {
    use server_integrations_taxjar::client::TaxJarParams;
    let mut params = TaxJarParams {
        amount_cents: i64::MAX,
        shipping_cents: 29,
        to_country: "US",
        to_zip: "90002",
        to_state: "CA",
        from_country: "US",
        from_zip: "92093",
        from_state: "CA",
    };
    let json = serde_json::to_string(&params).unwrap();
    assert!(json.contains(r#""amount":92233720368547758.07"#));
    assert!(json.contains(r#""shipping":0.29"#));
    assert!(json.contains(r#""to_zip":"90002""#));
    params.shipping_cents = -1;
    assert!(serde_json::to_string(&params).is_err());
    params.shipping_cents = 0;
    params.amount_cents = -1;
    assert!(serde_json::to_string(&params).is_err());
}

#[test]
fn line_totals_use_checked_integer_arithmetic() {
    use server_integrations_taxjar::money::checked_total_cents;
    assert_eq!(checked_total_cents([(29, 3), (57, 2)]), Ok(201));
    assert_eq!(checked_total_cents([(0, 0), (0, 1), (1, 0)]), Ok(0));
    assert_eq!(checked_total_cents([(i64::MAX, 1)]), Ok(i64::MAX));
    for lines in [
        vec![(i64::MAX, 2)],
        vec![(i64::MAX, 1), (1, 1)],
        vec![(-1, 1)],
        vec![(1, -1)],
    ] {
        assert!(checked_total_cents(lines).is_err());
    }
}
