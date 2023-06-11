fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["protos/reservation.proto"], &["protos"])
        .unwrap();

    //fs::remove_file("src/pb/reservation_service_server.rs").unwrap();")

    println!("cargo:rerun-if-changed=protos/reservation.proto");
}