use crate::{Arena, Block, Expression, Function, Handle, Module, Statement};
use alloc::{vec, vec::Vec};

fn visit_calls(block: &mut Block, callback: &mut impl FnMut(&mut Handle<Function>)) {
    for statement in block.iter_mut() {
        match *statement {
            Statement::Call {
                ref mut function, ..
            } => callback(function),
            Statement::Block(ref mut inner) => visit_calls(inner, callback),
            Statement::If {
                ref mut accept,
                ref mut reject,
                ..
            } => {
                visit_calls(accept, callback);
                visit_calls(reject, callback);
            }
            Statement::Switch { ref mut cases, .. } => {
                for case in cases {
                    visit_calls(&mut case.body, callback);
                }
            }
            Statement::Loop {
                ref mut body,
                ref mut continuing,
                ..
            } => {
                visit_calls(body, callback);
                visit_calls(continuing, callback);
            }
            _ => {}
        }
    }
}

#[derive(Clone, Copy)]
enum VisitState {
    Unvisited,
    Visiting,
    Visited,
}

enum Visit {
    Enter(Handle<Function>),
    Exit(Handle<Function>),
}

pub(super) fn order_functions(module: &mut Module) -> super::Result<()> {
    let order = dependency_order(module)?;
    let mut original_functions: Vec<_> = module
        .functions
        .drain()
        .map(|(_, function, span)| Some((function, span)))
        .collect();
    let mut ordered_functions = Arena::new();
    let mut new_handles = vec![None; original_functions.len()];
    for handle in order {
        let (function, span) = original_functions[handle.index()]
            .take()
            .expect("each function is visited once");
        new_handles[handle.index()] = Some(ordered_functions.append(function, span));
    }
    for (_, function) in ordered_functions.iter_mut() {
        remap_calls(function, &new_handles);
    }
    for entry_point in &mut module.entry_points {
        remap_calls(&mut entry_point.function, &new_handles);
    }
    module.functions = ordered_functions;
    Ok(())
}

fn remap_calls(function: &mut Function, new_handles: &[Option<Handle<Function>>]) {
    let mut remap_handle = |handle: &mut Handle<Function>| {
        *handle = new_handles[handle.index()].expect("all functions have been reordered");
    };
    for (_, expression) in function.expressions.iter_mut() {
        if let Expression::CallResult(ref mut handle) = *expression {
            remap_handle(handle);
        }
    }
    visit_calls(&mut function.body, &mut remap_handle);
}

fn dependency_order(module: &mut Module) -> super::Result<Vec<Handle<Function>>> {
    let callees: Vec<_> = module
        .functions
        .iter_mut()
        .map(|(_, function)| {
            let mut callees = Vec::new();
            visit_calls(&mut function.body, &mut |callee| callees.push(*callee));
            callees
        })
        .collect();
    let mut states = vec![VisitState::Unvisited; callees.len()];
    let mut order = Vec::with_capacity(callees.len());
    let mut pending = Vec::new();
    for (root, _) in module.functions.iter() {
        pending.push(Visit::Enter(root));
        while let Some(visit) = pending.pop() {
            let handle = match visit {
                Visit::Enter(handle) => handle,
                Visit::Exit(handle) => {
                    states[handle.index()] = VisitState::Visited;
                    order.push(handle);
                    continue;
                }
            };
            match states[handle.index()] {
                VisitState::Visited => continue,
                VisitState::Visiting => {
                    return Err(super::Error {
                        kind: super::ErrorKind::SemanticError("recursive function graph".into()),
                        meta: module.functions.get_span(handle),
                    });
                }
                VisitState::Unvisited => {}
            }
            states[handle.index()] = VisitState::Visiting;
            pending.push(Visit::Exit(handle));
            pending.extend(
                callees[handle.index()]
                    .iter()
                    .rev()
                    .map(|&callee| Visit::Enter(callee)),
            );
        }
    }
    Ok(order)
}
