use mongodb::{options::ClientOptions, Client};
//
fn main() {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb+srv://author:author123@cluster0-geoiq.mongodb.net/test?retryWrites=true&w=majority").unwrap();

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).unwrap();

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None).unwrap() {
        println!("{}", db_name);
    }
}
