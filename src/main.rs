use bson::doc;
use mongodb::{options::ClientOptions, Client};
use serde_json::{json, Value};
//
fn main() {
    let client= Client::with_uri_str("mongodb+srv://<username>:<password>@cluster0-geoiq.mongodb.net/test?retryWrites=true&w=majority").expect("Failed to connect");
    let db = client.database("test").collection("user_details");
    let docs = doc! { "user_name": "John123" };
    let data = db.insert_one(docs, None).unwrap();
    println!("id is : {}", data.inserted_id);
    let docs1 = doc! { "user_name": "John","password":"1244","email":"abc@asd.com" };
    let data = db.find_one(docs1,None).unwrap();
    match data {
        Some(data) => {
            let data: Value = json!(data);
            println!("data is : {}", data["user_name"]);
        }
        None => println!("No record Found"),
    }
}
