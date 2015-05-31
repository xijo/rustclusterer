extern crate rustc_serialize;
use rustc_serialize::json;





/////////////////////////
extern crate libc;
use std::ffi;
// use std::str;

use libc::c_char;
use std::ffi::CStr;

fn from_c_string(c_string: *const libc::c_char) -> String {
    // ffi::CString::from_ptr(c_string)
    unsafe {
        // str::from_utf8(ffi::c_str_to_bytes(&c_string)).unwrap().to_string()
        let c_str: &CStr = CStr::from_ptr(c_string);
        let buf: &[u8] = c_str.to_bytes();
        let str_slice: &str = std::str::from_utf8(buf).unwrap();
        let str_buf: String = str_slice.to_owned();
        str_buf
    }
}

fn to_c_string(rust_string: String) -> *const libc::c_char {
    // ffi::CString::from_slice(rust_string.as_bytes()).as_ptr()
    ffi::CString::new(rust_string).unwrap().as_ptr()
}

#[no_mangle]
pub extern "C" fn last_message() -> *const libc::c_char {
    to_c_string("We apologize for the inconvinience.".to_string())
}

#[no_mangle]
pub extern "C" fn greeting(gender: char) -> *const libc::c_char {
    to_c_string(match gender {
        'f' => "Miss",
        'm' => "Mister",
        _ => ""
    }.to_string())
}

#[no_mangle]
pub extern "C" fn format_address(c_street: *const libc::c_char, number: i32, c_city: *const libc::c_char) -> *const libc::c_char {

    let street = from_c_string(c_street);
    let city = from_c_string(c_city);

    let address = format!("{} {}, {}", street, number, city);

    to_c_string(address)
}
///////////////////////////////




















#[derive(RustcEncodable, RustcDecodable)]
struct Point {
    lat: f32,
    lng: f32,
    identifier: String,
}

impl Point {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(RustcEncodable)]
pub struct ClusterEncoder<'a> {
    lat:         f32,
    lng:         f32,
    identifiers: Vec<&'a str>,
    nelat:       f32,
    nelng:       f32,
    swlat:       f32,
    swlng:       f32,
}

impl<'a> ClusterEncoder<'a> {
    fn new(cluster: &'a Cluster) -> ClusterEncoder<'a> {
        ClusterEncoder {
            lat:         cluster.lat,
            lng:         cluster.lng,
            nelat:       cluster.nelat,
            nelng:       cluster.nelng,
            swlat:       cluster.swlat,
            swlng:       cluster.swlng,
            identifiers: cluster.identifiers(),
        }
    }
}

struct Cluster<'a> {
    lat:    f32,
    lng:    f32,
    points: Vec<&'a Point>,
    nelat:  f32,
    nelng:  f32,
    swlat:  f32,
    swlng:  f32,
}

impl<'a> Cluster<'a> {
    fn new<'b>(point: &'a Point, size: f32) -> Cluster<'a> {
        Cluster {
            lat:    point.lat,
            lng:    point.lng,
            points: vec![point],
            nelat:  point.lat - size,
            nelng:  point.lng - size * 2.0, // should be size*2 but error: the trait `core::ops::Mul<_>` is not implemented for the type `f32`
            swlat:  point.lat + size,
            swlng:  point.lng + size * 2.0, // same here, b/c lat 760 but lng 360
        }
    }

    fn contains(&self, p: &Point) -> bool {
        if p.lat >= self.nelat &&
            p.lat <= self.swlat &&
            p.lng >= self.nelng &&
            p.lng <= self.swlng {
                true
        } else {
            false
        }
    }

    fn identifiers(&self) -> Vec<&str> {
        if self.points.len() < 10 {
            self.points.iter().map(|p| p.identifier()).collect::<Vec<&str>>()
        } else {
            vec![]
        }
    }

    fn reposition(&mut self) {
        self.lat = self.points.iter().fold(0.0, |mem, x| mem + x.lat);
        self.lat = self.lat / (self.points.len() as f32);
        self.lng = self.points.iter().fold(0.0, |mem, x| mem + x.lng);
        self.lng = self.lng / (self.points.len() as f32);
    }
}

#[no_mangle]
pub extern "C" fn cluster(c_input: *const libc::c_char) -> *const libc::c_char {
    let input = from_c_string(c_input);
    let side_length = 3.0;
    let mut points: Vec<Point> = json::decode(&input).unwrap();
    let mut clusters: Vec<Cluster> = vec![];

    for point in &mut points {

        let mut build_new_cluster = clusters.len() == 0;

        for cluster in &mut clusters {
            if cluster.contains(point) {
                cluster.points.push(point);
                build_new_cluster = false;
                break;
            } else {
                build_new_cluster = true;
            }
        }

        if build_new_cluster {
            let new_cluster: Cluster = Cluster::new(point, side_length);
            clusters.push(new_cluster);
        }
    }

    // for cluster in &mut clusters {
    //     cluster.reposition();
    // }

    // let output = clusters.iter().map(|c| ClusterEncoder::new(c)).collect::<Vec<ClusterEncoder>>();

    // to_c_string(json::encode(&output).unwrap())
    to_c_string("".to_string())
}
