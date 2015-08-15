use ast;
use ast::Ast;
use ast::Function;
use ast::Elem::ElemFunction;

use error::ErrorCode;
use error::ParseError;

use parser::Parser;

use sym;
use sym::SymTable;
use sym::Sym::SymFunction;
use sym::SymFunctionType;

macro_rules! err {
    ( $errs: expr, $x: expr ) => {
        {
            match $x {
                Ok(val) => val,
                Err(_) => { return Err($errs); }
            }
        }
    };
}

// Only do semantic analysis until some amount of
// errors found
static max_errors: usize = 5;

pub struct SemCheck<'a> {
    ast: &'a Ast,
    errors: Vec<ParseError>,
}

impl<'a> SemCheck<'a> {
    pub fn new(ast: &Ast) -> SemCheck {
        SemCheck {
            ast: ast,
            errors: Vec::new(),
        }
    }

    pub fn check(mut self) -> Result<SymTable, Vec<ParseError>> {
        let mut globals = SymTable::new();

        for elem in &self.ast.elements {
            match *elem {
                ElemFunction(ref fct) => {
                    err!(self.errors, self.add_function_header(&mut globals, fct));
                }

                _ => unreachable!()
            }
        }

        for fct in globals.functions_mut() {
            err!(self.errors, self.check_function_body(fct));
        }

        if self.errors.len() == 0 {
            Ok(globals)
        } else {
            Err(self.errors)
        }
    }

    fn add_function_header(&mut self, globals: &mut SymTable, fct: &Function) -> Result<(), ()> {
        let params = fct.params.iter().map(|p| param_header(p));
        let symfct = SymFunction(SymFunctionType {
            name: fct.name,
            return_type: fct.return_type.builtin,
            params: params.collect(),
            body: fct.block.id()
        });

        if let Err(_) = globals.insert(fct.name, symfct) {
            try!(self.error(ParseError {
                code: ErrorCode::IdentifierAlreadyExists,
                message: format!("identifier {} already exists", self.ast.str(fct.name)),
                position: fct.pos
            }));
        }

        Ok(())
    }

    fn check_function_body(&mut self, fct: &SymFunctionType) -> Result<(), ()> {
        Ok(())
    }

    fn error(&mut self, error: ParseError) -> Result<(), ()> {
        self.errors.push(error);

        if self.errors.len() <= max_errors {
            Ok(())
        } else {
            Err(())
        }
    }

    fn errors(self) -> Vec<ParseError> {
        self.errors
    }

    fn check_function(&mut self, fct: &Function) {
        println!("check_function");
    }
}

fn param_header(param: &ast::Param) -> sym::Param {
    sym::Param {
        name: param.name,
        data_type: param.data_type.builtin
    }
}

#[test]
fn test_empty_file() {
    let prog = Parser::from_str("").parse().unwrap();

    SemCheck::new(&prog).check().unwrap();
}

#[test]
fn test_function_multiple_times() {
    let prog = Parser::from_str("fn main() {} fn main() {}").parse().unwrap();
    let errors = SemCheck::new(&prog).check().unwrap_err();

    assert_eq!(1, errors.len());
    assert_eq!(ErrorCode::IdentifierAlreadyExists, errors[0].code);
}
