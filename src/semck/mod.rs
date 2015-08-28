use driver::ctxt::Context;
use error::msg::Msg;

use parser::ast::{Function, Type};
use parser::ast::Type::*;
use parser::ast::visit;
use parser::ast::visit::Visitor;

use sym::Sym::*;
use sym::BuiltinType;

pub fn check(ctxt: &Context) {
    add_builtin_types(ctxt);

    SemCheck::new(ctxt).visit_ast(ctxt.ast);
}

fn add_builtin_types(ctxt: &Context) {
    let mut sym = ctxt.sym.borrow_mut();

    let name = ctxt.interner.intern("int".into());
    assert!(sym.insert(name, SymType(BuiltinType::Int)).is_none());

    let name = ctxt.interner.intern("bool".into());
    assert!(sym.insert(name, SymType(BuiltinType::Bool)).is_none());

    let name = ctxt.interner.intern("str".into());
    assert!(sym.insert(name, SymType(BuiltinType::Str)).is_none());
}

struct SemCheck<'a, 'ast: 'a> {
    ctxt: &'a Context<'a, 'ast>
}

impl<'a, 'ast> SemCheck<'a, 'ast> {
    fn new(ctxt: &'a Context<'a, 'ast>) -> SemCheck<'a, 'ast> {
        SemCheck {
            ctxt: ctxt
        }
    }
}

impl<'a, 'ast> Visitor<'ast> for SemCheck<'a, 'ast> {
    fn visit_fct(&mut self, f: &'ast Function) {
        let found = self.ctxt.sym.borrow().get(f.name).is_some();

        if found {
            let fname = self.ctxt.interner.str(f.name).clone_string();
            let msg = Msg::IdentifierExists(fname);

            self.ctxt.diag.borrow_mut().report(f.pos, msg);
        } else {
            self.ctxt.sym.borrow_mut().insert(f.name, SymFunction(f.id));
        }

        visit::walk_fct(self, f);
    }

    fn visit_type(&mut self, t: &'ast Type) {
        match *t {
            TypeBasic(ref basic) => {
                if let Some(builtin) = self.ctxt.sym.borrow().get_type(basic.name) {
                    self.ctxt.types.borrow_mut().insert(basic.id, builtin);
                } else {
                    let tyname = self.ctxt.interner.str(basic.name).clone_string();
                    let msg = Msg::UnknownType(tyname);
                    self.ctxt.diag.borrow_mut().report(basic.pos, msg);
                }
            }

            _ => self.ctxt.diag.borrow_mut().report_unimplemented(t.pos())
        }
    }
}
