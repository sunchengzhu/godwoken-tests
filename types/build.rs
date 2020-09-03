use std::process::Command;

const MOLECULE: &'static str = "moleculec";
const REQUIRED_MOLECULE_VERSION: &'static str = "0.6.0";
const SCHEMAS_DIR: &'static str = "schemas";
const OUTPUT_DIR: &'static str = "src/generated";

fn check_molecule_version() {
    let output = Command::new(MOLECULE)
        .arg("--version")
        .output()
        .expect("failed to execute process");
    assert!(output.status.success(), "process success");
    let out_str = String::from_utf8(output.stdout).expect("parse output");
    let mut iter = out_str.split_whitespace();
    iter.next();
    let ver_str = iter.next().expect("version");
    assert_eq!(
        ver_str, REQUIRED_MOLECULE_VERSION,
        "unsatisfied molecule version"
    );
}

fn generate_molecule(files: &[&str]) {
    for f in files {
        println!("cargo:rerun-if-changed={}/{}.mol", SCHEMAS_DIR, f);
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("{molc} --language rust --schema-file {schema_dir}/{file}.mol > {output_dir}/{file}.rs", molc=MOLECULE, file=f, schema_dir=SCHEMAS_DIR, output_dir=OUTPUT_DIR))
            .output()
            .expect("failed to execute process");
        assert!(output.status.success(), "run moleculec");
        let output = Command::new("rustfmt")
            .arg(format!(
                "{output_dir}/{file}.rs",
                file = f,
                output_dir = OUTPUT_DIR
            ))
            .output()
            .expect("failed to execute process");
        assert!(output.status.success(), "run rustfmt");
    }
}

fn main() {
    check_molecule_version();
    generate_molecule(&["blockchain", "godwoken"]);
}