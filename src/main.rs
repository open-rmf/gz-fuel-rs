use gz_fuel::FuelClient;
use std::time::Duration;

fn main() {
    let mut client = FuelClient::default();
    dbg!(&client.cache_path);
    let should_update = client.should_update_cache(&Some(Duration::from_secs(100000)));
    dbg!(&should_update);
    if should_update {
        client.update_cache_blocking(true);
    }
}
