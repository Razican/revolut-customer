//! Amount representation testing.

use revolut_customer::Amount;

/// Tests that amounts are parsed correctly.
#[test]
fn it_amount_parse() {
    let amount = "175.64".parse::<Amount>().unwrap();
    assert_eq!(amount, Amount::from_repr(175_64));
    assert_eq!(format!("{}", amount), "175.64");

    let amount = "175.6".parse::<Amount>().unwrap();
    assert_eq!(amount, Amount::from_repr(175_60));
    assert_eq!(format!("{}", amount), "175.6");

    let amount = "175.00".parse::<Amount>().unwrap();
    assert_eq!(amount, Amount::from_repr(175_00));
    assert_eq!(format!("{}", amount), "175");

    let amount = "175.0".parse::<Amount>().unwrap();
    assert_eq!(amount, Amount::from_repr(175_00));
    assert_eq!(format!("{}", amount), "175");

    let amount = "175".parse::<Amount>().unwrap();
    assert_eq!(amount, Amount::from_repr(175_00));
    assert_eq!(format!("{}", amount), "175");

    let amount = Amount::max_value();
    assert_eq!(amount, Amount::from_repr(u64::max_value()));
    assert_eq!(
        format!("{}", amount),
        format!("{}.{}", u64::max_value() / 1_00, u64::max_value() % 1_00)
    );

    let amount = format!("{}", amount).parse::<Amount>().unwrap();
    assert_eq!(amount, Amount::max_value());

    let amount = Amount::min_value();
    assert_eq!(amount, Amount::from_repr(0_00));
    assert_eq!(format!("{}", amount), "0");

    let amount = "0".parse::<Amount>().unwrap();
    assert_eq!(amount, Amount::min_value());

    let amount = "0.0049".parse::<Amount>().unwrap();
    assert_eq!(amount, Amount::from_repr(0));
    assert_eq!(format!("{}", amount), "0");

    let amount = "0.0050".parse::<Amount>().unwrap();
    assert_eq!(amount, Amount::from_repr(0_01));
    assert_eq!(format!("{}", amount), "0.01");

    let amount = "175.669".parse::<Amount>().unwrap();
    assert_eq!(amount, Amount::from_repr(175_67));
    assert_eq!(format!("{}", amount), "175.67");

    let amount = "175.665".parse::<Amount>().unwrap();
    assert_eq!(amount, Amount::from_repr(175_67));
    assert_eq!(format!("{}", amount), "175.67");

    let amount = "175.664".parse::<Amount>().unwrap();
    assert_eq!(amount, Amount::from_repr(175_66));
    assert_eq!(format!("{}", amount), "175.66");

    let amount = ".6465".parse::<Amount>().unwrap();
    assert_eq!(amount, Amount::from_repr(0_65));
    assert_eq!(format!("{}", amount), "0.65");
}

/// Tests that amounts are formatted correctly.
#[test]
fn it_amount_other_format() {
    let amount = Amount::from_repr(56_00); // 56.00
    assert_eq!(format!("{}", amount), "56");
    assert_eq!(format!("{:.2}", amount), "56.00");
    assert_eq!(format!("{:.5}", amount), "56.00000");
    assert_eq!(format!("{:05}", amount), "00056");
    assert_eq!(format!("{:05.2}", amount), "56.00");
    assert_eq!(format!("{:05.1}", amount), "056.0");

    let amount = Amount::from_repr(0_05); // 0.05
    assert_eq!(format!("{:.0}", amount), "0");
    assert_eq!(format!("{:.1}", amount), "0.1");

    let amount = Amount::from_repr(1_50); // 1.50
    assert_eq!(format!("{:.0}", amount), "2");
}

/// Tests that improperly formatted amount are not parsed to a valid amount.
#[test]
fn it_amount_bad_format() {
    let amount = "175.".parse::<Amount>();
    assert!(amount.is_err());

    let amount = "175.837.9239".parse::<Amount>();
    assert!(amount.is_err());

    let amount = ".098320.2930".parse::<Amount>();
    assert!(amount.is_err());
}

/// Test operations with amounts.
#[test]
fn it_amount_ops() {
    let mut amount = Amount::min_value();
    let amount_10 = Amount::from_repr(10_00);
    assert_eq!(amount + amount_10, Amount::from_repr(10_00));
    amount += amount_10;
    assert_eq!(amount, amount_10);

    assert_eq!(amount - amount_10, Amount::min_value());
    amount -= amount_10;
    assert_eq!(amount, Amount::min_value());

    amount += amount_10;
    assert_eq!(amount * 10_u32, Amount::from_repr(100_00));
    amount *= 10_u32;
    assert_eq!(amount, Amount::from_repr(100_00));

    assert_eq!(amount / 10_u32, Amount::from_repr(10_00));
    amount /= 10_u32;
    assert_eq!(amount, Amount::from_repr(10_00));

    let mut amount = Amount::from_repr(12_34);
    assert_eq!(amount % 10_u32, Amount::from_repr(2_34));
    amount %= 10_u32;
    assert_eq!(amount, Amount::from_repr(2_34));
    assert_eq!(amount % 1_u32, Amount::from_repr(0_34));
}
