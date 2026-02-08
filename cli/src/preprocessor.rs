
#[derive(Copy, Clone)]
pub enum HostLanguage {
    C, Cpp, Java, Rust, Python,
}

impl HostLanguage {
    pub fn to_extension(&self) -> &str {
        match self {
            HostLanguage::C => "c",
            HostLanguage::Cpp => "cpp",
            HostLanguage::Java => "java",
            HostLanguage::Rust => "rs",
            HostLanguage::Python => "py",
        }
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "c" => Some(HostLanguage::C),
            "cpp" => Some(HostLanguage::Cpp),
            "java" => Some(HostLanguage::Java),
            "rs" => Some(HostLanguage::Rust),
            "py" => Some(HostLanguage::Python),
            _ => None,
        }
    }
}

pub struct EmbeddedStateqSource {
    host_language: HostLanguage,
    source: String,
    label_loc: usize,
    token_begin: usize,
    token_end: usize,
}

impl EmbeddedStateqSource {

    pub fn new(host_language: HostLanguage, source: String) -> Self {
        let label_loc = source.find("@stateq").expect("No stateq source found");
        let token_begin = source[label_loc..].find("{").expect("No opening brace found") + label_loc;
        let mut brace_count = 1;
        for i in token_begin + 1 .. source.len() {
            match source.as_bytes()[i] {
                b'{' => brace_count += 1,
                b'}' => brace_count -= 1,
                _ => (),
            }
            if brace_count == 0 {
                return Self {
                    host_language,
                    source,
                    label_loc,
                    token_begin,
                    token_end: i,
                }
            }
        }
        panic!("Not enough closing brace found");
    }

    pub fn get_embedded_source(&self) -> String {
        self.source[self.token_begin + 1 .. self.token_end].to_string()
    }

    pub fn replace_embedded_source(&self, new_source: &str) -> String {
        vec![
            self.source[.. self.label_loc].to_string(),
            new_source.to_string(),
            self.source[self.token_end + 1 ..].to_string()
        ].join("")
    }
}
