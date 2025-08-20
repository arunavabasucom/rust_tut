// tests runs in different thread 
// we can change the default option to get different options 
// and the logs are captured but not shown 
// cargo test -- --help (-- -> it is for resulting binary of the tests)
// to change the threads --test-threads n_threads -> we need to use 
// cargo test -- --show-output 
// cargo test {{test name}}
// to run the ignored test -> cargo test -- --ignored
// in the rust the convention is to write the tests in the actual logic itself 
// just want to test the integraton ones -> cargo test --test integration
// for main.rs -> we have a binary crate and it can not run  integration tests 
// for lib.rs -> we have a library crate so we can run the integrations tests
// we can addup a thin layer of binary crate 