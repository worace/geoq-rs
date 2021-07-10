#![feature(try_blocks)]
mod geoq;
use geoq::commands;
use geoq::error::Error;
use geoq::text;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::process;

fn run(matches: ArgMatches) -> Result<(), Error> {
    match matches.subcommand() {
        ("wkt", Some(_)) => commands::wkt::run(),
        ("read", Some(_)) => commands::read::run(),
        ("gj", Some(m)) => commands::geojson_cmd::run(m),
        ("gh", Some(m)) => commands::geohash::run(m),
        ("map", Some(_)) => commands::map::run(),
        ("snip", Some(_)) => commands::snip::run(),
        ("filter", Some(m)) => commands::filter::run(m),
        ("json", Some(m)) => commands::json::run(m),
        ("centroid", Some(_)) => commands::centroid::run(),
        ("whereami", Some(_)) => commands::whereami::run(),
        ("simplify", Some(m)) => commands::simplify::run(m),
        ("measure", Some(m)) => commands::measure::run(m),
        ("bbox", Some(m)) => commands::bbox::run(m),
        ("shp", Some(m)) => commands::shp::run(m),
        _ => Err(Error::UnknownCommand),
    }
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let geojson = SubCommand::with_name("gj")
        .about("Output features as GeoJSON")
        .subcommand(SubCommand::with_name("geom").about("Output entity as a GeoJSON geometry"))
        .subcommand(SubCommand::with_name("f").about("Output entity as a GeoJSON Feature"))
        .subcommand(
            SubCommand::with_name("fc")
                .about("Collect all given entities into a GeoJSON Feature Collection"),
        );

    let geohash = SubCommand::with_name("gh")
        .about("Work with geohashes")
        .subcommand(
            SubCommand::with_name("point")
                .about("Output base 32 Geohash for a given Lat,Lon")
                .arg(
                    Arg::with_name("level")
                        .help("Characters of geohash precision")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("covering")
                .about("Output the set of geohashes at the given level which covers the given entity.")
                .arg(
                    Arg::with_name("level")
                        .help("Characters of geohash precision")
                        .required(true)
                        .index(1),
                ).arg(Arg::with_name("original")
                      .long("original")
                      .short("o")
                      .help("Also print the query entity in the output.\nUseful for mapping a geometry along with its covering Geohashes.")),
        )
        .subcommand(SubCommand::with_name("children").about("Get children for the given geohash"))
        .subcommand(SubCommand::with_name("roots").about("List the Base32 Geohash root characters"))
        .subcommand(SubCommand::with_name("encode-long").about("Convert a 64 bit geohash from Base 10 numeric representation to Base 32."))
        .subcommand(SubCommand::with_name("neighbors")
                    .about("Get neighbors of the given Geohash")
                    .arg(Arg::with_name("exclude")
                         .long("exclude")
                         .short("e")
                         .help("Exclude the given geohash from its neighbors.\nBy default it will be included in the output,\ngiving a 3x3 grid centered on the provided geohash.")));

    let filter = SubCommand::with_name("filter")
        .about("Select features based on geospatial predicates")
        .after_help(text::FILTER_AFTER_HELP)
        .arg(Arg::with_name("query-file")
             .help("Input file for reading query feature(s).")
             .takes_value(true)
             .global(true)
             .long("query-file")
             .short("q"))
        .arg(Arg::with_name("negate")
             .help("Negate the filter, so intersects becomes 'not intersects', etc.")
             .global(true)
             .long("negate")
             .short("n"))
        .subcommand(
            SubCommand::with_name("intersects")
                .about("Output only entities (from STDIN) which intersect a QUERY entity (as command-line ARG)")
                .arg(Arg::with_name("query")
                     .help("Entity to check intersections.\nMust be Lat/Lon, Geohash, WKT, or GeoJSON.")
                     .index(1))
        )
        .subcommand(
            SubCommand::with_name("contains")
                .about("Output only entities (from STDIN) which fall within a QUERY entity (as command-line ARG)")
                .arg(
                    Arg::with_name("query")
                        .help("Entity to check intersections.\nMust be Geohash, WKT, or GeoJSON.\nMust be a POLYGON or MULTIPOLYGON.")
                        .index(1)
                )
        );

    let json = SubCommand::with_name("json")
        .about("Best-guess conversions from geo-oriented JSON to GeoJSON")
        .subcommand(
            SubCommand::with_name("munge")
                .about("Attempt to convert arbitrary JSON to a GeoJSON Feature.")
                .after_help(text::JSON_MUNGE_AFTER_HELP),
        );

    let read = SubCommand::with_name("read")
        .about("Information about reading inputs with geoq")
        .after_help(text::READ_AFTER_HELP);

    let centroid = SubCommand::with_name("centroid")
        .about(text::CENTROID_ABOUT)
        .after_help(text::CENTROID_AFTER_HELP);

    let whereami = SubCommand::with_name("whereami")
        .about(text::WHEREAMI_ABOUT)
        .after_help(text::WHEREAMI_AFTER_HELP);

    let measure = SubCommand::with_name("measure")
        .about(text::MEASURE_ABOUT)
        .subcommand(
            SubCommand::with_name("distance")
                .about(text::DISTANCE_ABOUT)
                .after_help(text::DISTANCE_AFTER_HELP)
                .arg(
                    Arg::with_name("query")
                        .help(text::DISTANCE_QUERY_ARG_HELP)
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("coord-count")
                .about(text::MEASURE_COORDS_ABOUT)
                .arg(
                    Arg::with_name("geojson")
                        .long("geojson")
                        .required(false)
                        .takes_value(false)
                        .help(text::MEASURE_COORDS_GEOJSON_ARG_HELP),
                ),
        );

    let simplify = SubCommand::with_name("simplify")
        .about(text::SIMPLIFY_ABOUT)
        .after_help(text::SIMPLIFY_AFTER_HELP)
        .arg(
            Arg::with_name("epsilon")
                .help(text::SIMPLIFY_EPSILON_ARG_HELP)
                .required(true)
                .index(1),
        ).arg(
            Arg::with_name("to_coord_count")
                .long("to-coord-count")
                .required(false)
                .takes_value(true)
                .help(text::SIMPLIFY_TO_COORD_COUNT_ARG_HELP),
        );

    let bbox = SubCommand::with_name("bbox")
        .about("Generate bounding boxes for geometries")
        .arg(Arg::with_name("embed")
             .long("embed")
             .short("e")
             .help("Print inputs as GeoJSON features and include the bbox in the GeoJSON bbox field"))
        .arg(Arg::with_name("all")
             .long("all")
             .short("a")
             .help("Give a single bbox for all input geometries rather than 1 bbox per input"))
        .after_help(text::BBOX_AFTER_HELP);

    let shp = SubCommand::with_name("shp")
        .about("Read a shapefile and convert to GeoJSON")
        .arg(
            Arg::with_name("path")
                .help("path to the .shp file -- expects .dbf file to be adjacent.")
                .required(true)
                .index(1),
        );

    let matches = App::new("geoq")
        .version(VERSION)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("geoq - GeoSpatial utility belt")
        .after_help(text::MAIN_AFTER_HELP)
        .subcommand(SubCommand::with_name("wkt").about("Output features as Well-Known Text"))
        .subcommand(SubCommand::with_name("map").about("View features on a map using geojson.io"))
        .subcommand(read)
        .subcommand(geohash)
        .subcommand(geojson)
        .subcommand(json)
        .subcommand(filter)
        .subcommand(centroid)
        .subcommand(whereami)
        .subcommand(measure)
        .subcommand(simplify)
        .subcommand(bbox)
        .subcommand(shp)
        .get_matches();

    if let Err(e) = run(matches) {
        eprintln!("Application error: {:?}", e);
        process::exit(1);
    }
}
