fn main() {
    let input = r#"variable "instance_type" {
      type        = string
      description = "The size of the EC2 instance"
      default     = "t3.micro" # Optional: Used if no other value is given
    }"#;

    match hcl::parse(input) {
        Ok(parsed) => {
            println!("Successfully parsed HCL:\n{:#?}", parsed);
        }
        Err(e) => {
            eprintln!("Failed to parse HCL: {}", e);
        }
    }
}
