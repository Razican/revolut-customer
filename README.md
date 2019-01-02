# Revolut consumer API

[![Build Status][build_svg]][build_status]
[![codecov][coverage_svg]][coverage_status]

This crate is meant to interact with the Revolut customer API, not to be confused with the
business API. This API is not public, and therefore it's subject to change. It has been reverse
engineered and ported from <https://github.com/SebastianJ/revolut-api>.

**Example:**

```rust
use revolut_customer::Client;

let mut client = Client::default();
client.sign_in("+1555555555", "9999")?;
client.confirm_sign_in("+1555555555", "111-111")?;

println!("User ID: {}", client.user_id().unwrap());
println!("Access token: {}", client.access_token().unwrap());
```

[build_svg]: https://travis-ci.org/Razican/revolut-customer.svg?branch=master
[build_status]: https://travis-ci.org/Razican/revolut-customer
[coverage_svg]: https://codecov.io/gh/Razican/revolut-customer/branch/master/graph/badge.svg
[coverage_status]: https://codecov.io/gh/Razican/revolut-customer
