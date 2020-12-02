
struct UriParts {
    method: String,
    uri: String,
    hostname: String,
    pathname: String,
    version: String,
    port: u16,
}

fn parse_uri(request: String) -> UriParts {
    let parts: Vec<&str> = request.split(' ').collect();
    if parts.len() < 3 {
        panic!("Request has too few parts");
    }
    let uri_parts: Vec<&str> = parts[1].splitn(4, '/').collect();
    if uri_parts.len() < 3 {
        panic!("Not enough URI Parts");
    }
    
    let mut pathname: String = "".to_string();
    if uri_parts.len() > 3 {
        pathname = uri_parts[3].to_string()
    }
    let mut port: u16 = 80;
    let mut hostname = uri_parts[2];
    if uri_parts[2].contains(":"){
        let host_parts: Vec<&str> = uri_parts[2].split(":").collect();
        if host_parts.len() <= 0 {
            panic!("Something went wrong with finding port");
        }
        hostname = host_parts[0];
        port = host_parts[1].parse::<u16>().unwrap();
    }

    let uri_parts_struct = UriParts {
        method: parts[0].to_string(),
        version: parts[2].to_string(),
        uri: parts[1].to_string(),
        hostname: hostname.to_string(),
        pathname: pathname,
        port: port,
    };
    return uri_parts_struct
}

fn main() {
    println!("Hello, world!");
}
