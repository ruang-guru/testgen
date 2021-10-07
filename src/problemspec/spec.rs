use thiserror::Error;

#[derive(Debug, Error)]
#[error("Expected: {messages:?}")]
pub struct ConstraintsError {
    pub messages: Vec<String>,
}

pub trait ProblemSpec {
    fn input_format(&self) -> IOFormat;
    // TODO: check output format
    // fn output_format(&self) -> IOFormat;
    fn constraints(&self) -> Result<(), ConstraintsError>;
    fn multiple_test_case_config() -> Option<MultipleTestcaseConfig> {
        None
    }
}

pub type Size = usize;

pub struct MultipleTestcaseConfig {
    pub constraints: fn(usize) -> Result<(), ConstraintsError>,
    pub output_prefix: Option<String>,
}

pub type IOFormat = Vec<IOElement>;

#[derive(Debug, Clone)]
pub enum IOElement {
    Line(Vec<LineElement>),
    RawLine(String),
    EmptyLine,
    LinesBounded(Vec<Vec<Scalar>>, Size),
    LinesUnbounded(Vec<Vec<Scalar>>),
    RawLinesBounded(Vec<String>, Size),
    RawLinesUnbounded(Vec<String>),
    Grid(Vec<Vec<Scalar>>, Size, Size),
}

#[derive(Debug, Clone)]
pub enum Scalar {
    UInt(u64),
    Float(f64),
    String(String),
    Char(char),
    Int(i64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum LineElement {
    Scalar(Scalar),
    BoundedVec(Vec<Scalar>, Size),
    UnboundedVec(Vec<Scalar>),
}

#[macro_export]
macro_rules! cons {
    ($($x:expr),*) => {
        {
            let mut errors = Vec::new();
            $(
                if !($x) {
                    errors.push(String::from(stringify!($x)));
                }
            )*
            if errors.is_empty() {
                Ok(())
            } else {
                Err(ConstraintsError { messages: errors })
            }
        }

        // let mut errors = Vec::new();
        // if !(self.a > 0) {
        //     errors.push("a > 0".to_string());
        // }
        // if !(self.b > 0) {
        //     errors.push("b > 0".to_string());
        // }
        // if errors.is_empty() {
        //     Ok(())
        // } else {
        //     Err(ConstraintsError { messages: errors })
        // }
    };
}
