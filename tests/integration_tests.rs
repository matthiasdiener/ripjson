#[cfg(test)]
mod tests {
    // use super::main::search_string;
    use assert_cmd::Command;
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn calling_ripjson_without_arguments() {
        let mut cmd = Command::cargo_bin(env!("CARGO_BIN_EXE_rj") ).unwrap();
        cmd.assert().failure();
    }

    #[test]
    fn find_simple_file() {
        let mut file = File::create("test.json").unwrap();
        file.write_all(
            b"{\"name\": \"John Doe\",\"age\": 43,\"address\":
        {\"street\": \"10 Downing Street\",\"city\": \"London\"
        },\"phones\": [\"+44 1234567\",\"+44 2345678\"]}",
        )
        .unwrap();
        let mut cmd = Command::cargo_bin(env!("CARGO_BIN_EXE_rj") ).unwrap();
        let assert = cmd
            .arg(".*es.*/cit")
            .arg("test.json")
            .assert();

        assert
            .success()
            .stdout("address/city = \"London\"\n");
        fs::remove_file("test.json").unwrap();
    }
}
