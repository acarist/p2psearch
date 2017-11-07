extern crate futures;
extern crate term;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;
extern crate hyper_tls;

use std::io;
use std::env;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use serde_json::Value;
use hyper_tls::HttpsConnector;

fn main() {
	let mut t = term::stdout().unwrap();

	let args: Vec<String> = env::args().collect();

  let  query: &str = &args[1].to_string();
  
  if query.is_empty() {
  	panic!("Movie name not provided!");
  }
  let url: &str = "https://yts.ag/api/v2/list_movies.json?query_term=";

	let mut core = Core::new().expect("?");
	let handle = core.handle();
	let client = Client::configure()
    .connector(HttpsConnector::new(4, &handle).expect("?"))
    .build(&handle);

	let uri = format!("{}{}", url, query)
	.parse()
	.expect("?");

	let work = client.get(uri).and_then(|res| {
	    res.body().concat2().and_then(move |body| {
	        let v: Value = serde_json::from_slice(&body).map_err(|e| {
	            io::Error::new(
	                io::ErrorKind::Other,
	                e
	            )
	        })?;
					println!("{}", v);
					println!("Finding Count: {}", v["data"]["movie_count"]);
					if v["data"]["movie_count"] == 0 {
						panic!("Torrent not found");
					}

					let movies = v.as_object()
		        .and_then(|object| object.get("data"))
		        .and_then(|object| object.get("movies"))
		        .and_then(|movies| movies.as_array())
		        .unwrap_or_else(|| {
		            panic!("Failed to get 'movies' value from json");
		        });
					for movie in movies {
							t.fg(term::color::WHITE).unwrap();
							println!("====================================================");
					    println!("Name: {}", movie["title_long"]);
					    println!("Imdb Code: {}", movie["imdb_code"]);
					    println!("Synopsis: {}", movie["synopsis"]);

					    let torrents = movie["torrents"].as_array()
			        .unwrap_or_else(|| {
			            panic!("Failed to get 'movies' value from json");
			        });
			        t.fg(term::color::RED).unwrap();
			        println!("Torrents : {}", torrents.len());
			        println!("----------------------------------------------------");
			        t.fg(term::color::GREEN).unwrap();
			        for torrent in torrents {
			        	
			        	println!("Torrent URL: {}", torrent["url"]);
						    println!("Quality: {}", torrent["quality"]);
						    println!("Seeds {}", torrent["seeds"]);
						    println!("Peers: {}", torrent["peers"]);
						    println!("Size: {}", torrent["size"]);
						    
			        }
					}
	        Ok(())
	    })
	});
	core.run(work).expect("?");
}