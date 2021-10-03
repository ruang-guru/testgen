use super::spec::*;

pub trait Generator {
    fn generate(&self) -> String;
}

impl Generator for IOFormat {
    fn generate(&self) -> String {
        self.iter()
            .map(|e| e.generate())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Generator for IOElement {
    fn generate(&self) -> String {
        match self {
            IOElement::Line(line) => line
                .iter()
                .map(|element| element.generate())
                .collect::<Vec<String>>()
                .join(" "),
            IOElement::LinesBounded(lines, size) => {
                let mut result = String::new();
                for i in 0..*size {
                    for (pos, line) in lines.iter().enumerate() {
                        result.push_str(line[i].generate().as_str());
                        if pos != lines.len() - 1 {
                            result.push_str(" ");
                        }
                    }
                    if i != *size - 1 {
                        result.push_str("\n");
                    }
                }
                result
            }
            IOElement::LinesUnbounded(lines) => {
                let mut result = String::new();
                for i in 0..lines[0].len() {
                    for (pos, line) in lines.iter().enumerate() {
                        result.push_str(line[i].generate().as_str());
                        if pos != lines.len() - 1 {
                            result.push_str(" ");
                        }
                    }
                    if i != lines[0].len() - 1 {
                        result.push('\n');
                    }
                }
                result
            }
            IOElement::RawLine(line) => line.to_string(),
            IOElement::EmptyLine => String::new(),
            IOElement::RawLinesBounded(lines, size) => {
                let mut result = String::new();
                for i in 0..*size {
                    result.push_str(lines[i].as_str());
                    if i != *size - 1 {
                        result.push('\n');
                    }
                }
                result
            }
            IOElement::RawLinesUnbounded(lines) => lines.join("\n"),
        }
    }
}

impl Generator for Scalar {
    fn generate(&self) -> String {
        match self {
            Scalar::UInt(u) => u.to_string(),
            Scalar::Int(i) => i.to_string(),
            Scalar::Float(f) => f.to_string(),
            Scalar::String(s) => s.to_string(),
            Scalar::Char(c) => c.to_string(),
            Scalar::Bool(b) => b.to_string(),
        }
    }
}

impl Generator for LineElement {
    fn generate(&self) -> String {
        match self {
            LineElement::Scalar(s) => s.generate(),
            LineElement::BoundedVec(v, size) => {
                let mut result = String::new();
                for i in 0..*size {
                    result.push_str(&v[i].generate());
                    if i != *size - 1 {
                        result.push_str(" ");
                    }
                }
                result
            }
            LineElement::UnboundedVec(v) => v
                .iter()
                .map(|s| s.generate())
                .collect::<Vec<String>>()
                .join(" "),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_line() {
        let line = IOElement::Line(vec![
            LineElement::Scalar(Scalar::Int(1)),
            LineElement::BoundedVec(vec![Scalar::Int(2), Scalar::Int(3)], 2),
            LineElement::UnboundedVec(vec![Scalar::Int(4), Scalar::Int(5)]),
            LineElement::Scalar(Scalar::Int(6)),
        ]);
        let result = line.generate();
        assert_eq!(result, "1 2 3 4 5 6");
    }

    #[test]
    fn test_generate_lines_bounded() {
        let a = vec![1, 2, 3]
            .into_iter()
            .map(Scalar::Int)
            .collect::<Vec<_>>();
        let b = vec![4, 5, 6]
            .into_iter()
            .map(Scalar::Int)
            .collect::<Vec<_>>();
        let lines = vec![a, b];
        let result = IOElement::LinesBounded(lines.clone(), 3).generate();
        assert_eq!(result, "1 4\n2 5\n3 6");

        let result = IOElement::LinesBounded(lines.clone(), 1).generate();
        assert_eq!(result, "1 4");
    }

    #[test]
    fn test_generate_lines_unbounded() {
        let a = vec![1, 2, 3]
            .into_iter()
            .map(Scalar::Int)
            .collect::<Vec<_>>();
        let b = vec![4, 5, 6]
            .into_iter()
            .map(Scalar::Int)
            .collect::<Vec<_>>();
        let lines = vec![a, b];
        let result = IOElement::LinesUnbounded(lines.clone()).generate();
        assert_eq!(result, "1 4\n2 5\n3 6");
    }

    #[test]
    fn test_generate_raw_line() {
        let line = IOElement::RawLine("Hello World".to_string());
        let result = line.generate();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_generate_empty_line() {
        let line = IOElement::EmptyLine;
        let result = line.generate();
        assert_eq!(result, "");
    }

    #[test]
    fn test_generate_raw_lines_bounded() {
        let lines = vec![
            "Hello World".to_string(),
            "Hello World".to_string(),
            "Hello World".to_string(),
        ];
        let line = IOElement::RawLinesBounded(lines.clone(), 3);
        let result = line.generate();
        assert_eq!(result, "Hello World\nHello World\nHello World");

        let line = IOElement::RawLinesBounded(lines.clone(), 2);
        let result = line.generate();
        assert_eq!(result, "Hello World\nHello World");
    }

    #[test]
    fn test_generate_raw_lines_unbounded() {
        let lines = vec![
            "Hello World".to_string(),
            "Hello World".to_string(),
            "Hello World".to_string(),
        ];
        let line = IOElement::RawLinesUnbounded(lines);
        let result = line.generate();
        assert_eq!(result, "Hello World\nHello World\nHello World");
    }

    #[test]
    fn test_generate_scalar() {
        let scalar = Scalar::UInt(42);
        assert_eq!(scalar.generate(), "42");

        let scalar = Scalar::Int(-42);
        assert_eq!(scalar.generate(), "-42");

        let scalar = Scalar::Float(3.14);
        assert_eq!(scalar.generate(), "3.14");

        let scalar = Scalar::String("Hello World".to_string());
        assert_eq!(scalar.generate(), "Hello World");

        let scalar = Scalar::Char('a');
        assert_eq!(scalar.generate(), "a");

        let scalar = Scalar::Bool(true);
        assert_eq!(scalar.generate(), "true");
    }

    #[test]
    fn test_generate_line_element() {
        let element = LineElement::Scalar(Scalar::Int(1));
        assert_eq!(element.generate(), "1");
        let element = LineElement::BoundedVec(vec![Scalar::Int(1), Scalar::Int(2)], 2);
        assert_eq!(element.generate(), "1 2");
        let element = LineElement::BoundedVec(vec![Scalar::Int(1), Scalar::Int(2)], 1);
        assert_eq!(element.generate(), "1");
        let element = LineElement::UnboundedVec(vec![Scalar::Int(1), Scalar::Int(2)]);
        assert_eq!(element.generate(), "1 2");
    }
}
