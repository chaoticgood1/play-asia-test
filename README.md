All the CRUD required tests are all in tests/item_api_tests.rs.

To login to get JWT: [Login demo to get token here](https://github.com/chaoticgood1/play-asia-test/blob/1d5e34ff3d45c7dea2f3a996dd8ad6057421cc07/tests/item_api_tests.rs#L367)

```
user: admin1
pass: admin1
```

To run
```
cargo make run
```

To run tests
```
cargo test --test item_api_tests -- --nocapture
```



