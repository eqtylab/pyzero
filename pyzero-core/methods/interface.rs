pub mod interface {
    #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
    pub struct PythonCodeManifest {
        pub code: Vec<PythonCodeLine>,
        pub args: Vec<PythonArg>,
    }

    #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
    pub struct PythonCodeLine {
        pub line: String,
        pub redaction: LineRedaction,
    }

    #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
    pub struct PythonArg {
        pub arg: String,
        pub redaction: ArgRedaction,
    }

    #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
    pub enum LineRedaction {
        None,
        FullLine,
    }

    #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
    pub enum ArgRedaction {
        None,
        FullString,
    }

    #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
    pub struct PythonCodeResult {
        pub public_code: Vec<Option<String>>,
        pub public_args: Vec<Option<String>>,
        pub stdout: String,
    }
}