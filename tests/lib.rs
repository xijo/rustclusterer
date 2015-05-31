extern crate rustclusterer;
use rustclusterer::*;

#[test]
fn it_works() {
    let input = "[{ \"lat\": 10.0, \"lng\": 10.0, \"identifier\": \"p1\" }]".to_string();
    let expected = "[{\"lat\":10.0,\"lng\":10.0,\"identifiers\":[\"p1\"],\"nelat\":7.0,\"nelng\":4.0,\"swlat\":13.0,\"swlng\":16.0}]".to_string();

    assert_eq!(expected, cluster(input, 3.0));
}

#[test]
fn it_works_with_many_points() {

    assert_eq!(4.0, 2.0 * 2.0);
    // let points: Vec<Point> = vec![
    //     Point { lat: 10.0, lng: 10.0, identifier: "p1".to_string() },
    // ];
}
