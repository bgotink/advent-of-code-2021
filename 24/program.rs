use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;
use std::collections::HashSet;
use std::borrow::Borrow;
use std::hash;
use std::cmp;

#[derive(PartialEq,Eq,Clone,Copy,Hash)]
enum Operation {
  Add,
  Mul,
  Div,
  Mod,
  Eql,
}

impl std::fmt::Display for Operation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Operation::Add => write!(f, "+"),
      Operation::Mul => write!(f, "*"),
      Operation::Div => write!(f, "/"),
      Operation::Mod => write!(f, "%"),
      Operation::Eql => write!(f, "=="),
    }
  }
}

enum Value {
  Input(usize, HashSet<Condition>),
  Constant(i64, HashSet<Condition>),
  Operation(Operation, Rc<Value>, Rc<Value>, HashSet<Condition>),
}

impl Clone for Value {
  fn clone(&self) -> Self {
    match self {
      Value::Input(i, conditions) => Value::Input(*i, conditions.clone()),
      Value::Constant(v, conditions) => Value::Constant(*v, conditions.clone()),
      Value::Operation(op, left, right, conditions) => Value::Operation(*op, Rc::clone(left), Rc::clone(right), conditions.clone()),
    }
  }
}

impl hash::Hash for Value {
  fn hash<H>(&self, state: &mut H)
  where
      H: hash::Hasher {
    match self {
      Value::Input(i, _) => {
        state.write_i64(0);
        i.hash(state);
      },
      Value::Constant(i, _) => {
        state.write_i64(1);
        i.hash(state);
      },
      Value::Operation(op, a, b, _) => {
        state.write_i64(2);
        op.hash(state);
        a.hash(state);
        b.hash(state);
      }
    }
  }
}

impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Input(v, _) => write!(f, "input_{}", v),
      Value::Constant(c, _) => write!(f, "{}", c),
      Value::Operation(op, a, b, _) => write!(f, "({} {} {})", a, op, b),
    }
  }
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Value::Input(i, _) => match other {
        Value::Input(j, _) => i == j,
        _ => false,
      },
      Value::Constant(c, _) => match other {
        Value::Constant(d, _) => c == d,
        _ => false,
      },
      Value::Operation(op, a, b, _) => match other {
        Value::Operation(op2, a2, b2, _) => op == op2 && a == a2 && b == b2,
        _ => false,
      }
    }
  }
}

impl Eq for Value {}

impl Value {
  fn r#const(val: i64) -> Rc<Value> {
    Rc::new(Value::Constant(val, HashSet::new()))
  }

  fn input(val: usize) -> Rc<Value> {
    Rc::new(Value::Input(val, HashSet::new()))
  }

  fn op(operation: Operation, left: Rc<Value>, right: Rc<Value>) -> Rc<Value> {
    let conditions = merge_sets(left.conditions(), right.conditions());
    Rc::new(Value::Operation(operation, left, right, conditions))
  }

  fn conditions(&self) -> &HashSet<Condition> {
    match self {
      Value::Input(_, c) => c,
      Value::Constant(_, c) => c,
      Value::Operation(_, _, _, c) => c,
    }
  }

  fn min_value(&self) -> i64 {
    match self {
      Value::Input(_, _) => 1,
      Value::Constant(c, _) => *c,
      Value::Operation(op, a, b, _) => {
        match op {
          Operation::Add => a.min_value() + b.min_value(),
          Operation::Mul => a.min_value() * b.min_value(),
          Operation::Div => a.min_value() / b.max_value(),
          Operation::Mod => if a.min_value() < 0 { -(b.max_value() - 1) } else { 0 },
          Operation::Eql => 0
        }
      }
    }
  }

  fn max_value(&self) -> i64 {
    match self {
      Value::Input(_, _) => 9,
      Value::Constant(c, _) => *c,
      Value::Operation(op, a, b, _) => {
        match op {
          Operation::Add => a.max_value() + b.max_value(),
          Operation::Mul => a.max_value() * b.max_value(),
          Operation::Div => a.max_value() / b.min_value(),
          Operation::Mod => if a.max_value() > 0 { b.max_value() - 1 } else { 0 },
          Operation::Eql => 1
        }
      }
    }
  }

  fn static_value(&self) -> Option<i64> {
    if let Value::Constant(v, _) = self {
      Some(*v)
    } else {
      None
    }
  }

  fn is_multiple_of(&self, val: i64) -> Option<bool> {
    if val == 1 {
      return Some(true);
    }
    
    if self.max_value() < val && self.min_value() > 0 {
      return Some(false);
    }

    match self {
      Value::Input(_, _) => None,
      Value::Constant(c, _) => Some(c % val == 0),
      Value::Operation(op, a, b, _) => {
        match op {
          Operation::Add => {
            let isa = a.is_multiple_of(val);
            let isb = b.is_multiple_of(val);

            if isa == Some(true) && isb == Some(true) {
              Some(true)
            } else if isa == Some(false) && isb == Some(false) {
              Some(false)
            } else {
              None
            }
          },
          Operation::Mul => {
            let isa = a.is_multiple_of(val);
            let isb = b.is_multiple_of(val);

            if isa == Some(true) || isb == Some(true) {
              Some(true)
            } else if isa == Some(false) && isb == Some(false) {
              Some(false)
            } else {
              None
            }
          },
          Operation::Div => None,
          Operation::Mod => {
            if b.max_value() < val {
              Some(false)
            } else {
              a.is_multiple_of(val)
            }
          },
          Operation::Eql => Some(val == 1),
        }
      }
    }
  }

  fn clone_with_extra_conditions(&self, conditions: &HashSet<Condition>) -> Self {
    let mut clone = self.clone();

    match clone {
      Value::Input(_, ref mut c) => *c = merge_sets(c, conditions),
      Value::Constant(_, ref mut c) => *c = merge_sets(c, conditions),
      Value::Operation(_, _, _, ref mut c) => *c = merge_sets(c, conditions),
    }

    clone
  }
}

enum Condition {
  Eq(Rc<Value>, Rc<Value>),
  Ne(Rc<Value>, Rc<Value>),
}

impl Clone for Condition {
  fn clone(&self) -> Self {
    match self {
      Condition::Eq(a, b) => Condition::Eq(Rc::clone(a), Rc::clone(b)),
      Condition::Ne(a, b) => Condition::Ne(Rc::clone(a), Rc::clone(b)),
    }
  }
}

impl hash::Hash for Condition {
  fn hash<H>(&self, state: &mut H)
  where
      H: hash::Hasher {
    match self {
      Condition::Eq(a, b) => {
        state.write_i64(0);
        a.hash(state);
        b.hash(state);
      },
      Condition::Ne(a, b) => {
        state.write_i64(1);
        a.hash(state);
        b.hash(state);
      },
    }
  }
}

impl PartialEq for Condition {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Condition::Eq(a, b) => {
        match other {
          Condition::Eq(a2, b2) => a == a2 && b == b2,
          _ => false,
        }
      },
      Condition::Ne(a, b) => {
        match other {
          Condition::Ne(a2, b2) => a == a2 && b == b2,
          _ => false,
        }
      },
    }
  }
}

impl Eq for Condition {}

impl std::fmt::Display for Condition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Condition::Eq(a, b) => write!(f, "{} == {}", a, b),
      Condition::Ne(a, b) => write!(f, "{} != {}", a, b),
    }
  }
}

fn merge_sets<T: hash::Hash + cmp::Eq + Clone>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T> {
  a.union(b).cloned().collect()
}

fn set_with<T: hash::Hash + cmp::Eq + Clone>(a: &HashSet<T>, b: T) -> HashSet<T> {
  a.iter().cloned().chain([b]).collect()
}

/// Try to simplify the given value
///
/// The goal isn't to apply all possible simplifications, this specifically tries
/// to simplify operations that actually come up in the input program.
///
/// This simplification yields one or more values, depending on the value.
fn simplify(val: Rc<Value>) -> Vec<Rc<Value>> {
  if let Value::Operation(op, a, b, _) = val.borrow() {
    match op {
      // Simplify the `eql` operator to a constant 0 and/or 1
      //
      // If we can tell for sure that the values are (not) equal, we can simplify to
      // the single value it the eql operator ends up with.
      // If we can't, e.g. because of the dynamic nature of the two operands, we
      // create two new values (0 and 1) and attach a new condition to these values
      // to handle the (in)equality.
      Operation::Eql => {
        let merged_conditions = val.conditions().clone();

        if let Value::Constant(aval, _) = a.borrow() {
          if let Value::Constant(bval, _) = b.borrow() {
            return vec![Rc::new(
              Value::Constant(
                if aval == bval { 1 } else { 0 },
                merged_conditions,
              )
            )];
          }
        }

        if a.max_value() < b.min_value() || a.min_value() > b.max_value() {
          return vec![Rc::new(
            Value::Constant(
              0,
              merged_conditions,
            )
          )];
        }

        vec![
          Rc::new(Value::Constant(1, set_with(&merged_conditions, Condition::Eq(Rc::clone(a), Rc::clone(b))))),
          Rc::new(Value::Constant(0, set_with(&merged_conditions, Condition::Ne(Rc::clone(a), Rc::clone(b))))),
        ]
      },

      // Simplify the `add` operator by collapsing constant additions
      //
      // If both operands are constants, resolve them into a constant.
      // If one operand is zero, replace the sum by the other operand.
      // If one operand is constant and the other is a sum with a constant, collapse
      // this into a single addition with a constant part consisting of the sum of
      // both constants.
      Operation::Add => {
        if let Value::Constant(0, _) = b.borrow() {
          return vec![Rc::new(
            a.clone_with_extra_conditions(val.conditions())
          )];
        }

        if let Value::Constant(aval, aconditions) = a.borrow() {
          if let Value::Constant(bval, _) = b.borrow() {
            return vec![Rc::new(
              Value::Constant(aval + bval, val.conditions().clone())
            )];
          }

          if *aval == 0 {
            return vec![Rc::new(
              b.clone_with_extra_conditions(val.conditions())
            )];
          }

          if let Value::Operation(Operation::Add, left, right, _) = b.borrow() {
            if let Value::Constant(c, cond) = left.borrow() {
              let new_left = Rc::new(Value::Constant(aval + c, merge_sets(cond, aconditions)));
              let new_conditions = merge_sets(new_left.conditions(), val.conditions());
              return vec![Rc::new(
                Value::Operation(Operation::Add, new_left, Rc::clone(right), new_conditions)
              )];
            } else if let Value::Constant(c, cond) = right.borrow() {
              let new_right = Rc::new(Value::Constant(aval + c, merge_sets(cond, aconditions)));
              let new_conditions = merge_sets(val.conditions(), new_right.conditions());
              return vec![Rc::new(
                Value::Operation(Operation::Add, Rc::clone(left), new_right, new_conditions)
              )];
            }
          }
        }

        if let Value::Constant(bval, bconditions) = b.borrow() {
          if let Value::Operation(Operation::Add, left, right, _) = a.borrow() {
            if let Value::Constant(c, cond) = left.borrow() {
              let new_left = Rc::new(Value::Constant(bval + c, merge_sets(cond, bconditions)));
              let new_conditions = merge_sets(new_left.conditions(), val.conditions());
              return vec![Rc::new(
                Value::Operation(Operation::Add, new_left, Rc::clone(right), new_conditions)
              )];
            } else if let Value::Constant(c, cond) = right.borrow() {
              let new_right = Rc::new(Value::Constant(bval + c, merge_sets(cond, bconditions)));
              let new_conditions = merge_sets(val.conditions(), new_right.conditions());
              return vec![Rc::new(
                Value::Operation(Operation::Add, Rc::clone(left), new_right, new_conditions)
              )];
            }
          }
        }

        vec![Rc::new(
          Value::Operation(Operation::Add, Rc::clone(&a), Rc::clone(b), merge_sets(a.conditions(), b.conditions()))
        )]
      },

      // Simplify the `mul` operator if statically knowable
      //
      // If one operand is zero, replace the multiplication with the constant zero.
      // If one operand is one, replace the multiplication with the other operand.
      // If both operands are constants, replace the multiplication with a constant
      // containing the product of both operands.
      Operation::Mul => {
        if let Value::Constant(0, _) = b.borrow() {
          return vec![Rc::clone(b)];
        } else if let Value::Constant(0, _) = a.borrow() {
          return vec![Rc::clone(a)];
        }

        if let Value::Constant(1, _) = b.borrow() {
          return vec![Rc::new(
            a.clone_with_extra_conditions(b.conditions())
          )]
        }

        if let Value::Constant(aval, _) = a.borrow() {
          if *aval == 1 {
            return vec![Rc::new(
              b.clone_with_extra_conditions(a.conditions())
            )];
          }

          if let Value::Constant(bval, _) = b.borrow() {
            return vec![Rc::new(
              Value::Constant(aval + bval, merge_sets(a.conditions(), b.conditions()))
            )];
          }
        }

        vec![Rc::new(
          Value::Operation(Operation::Mul, Rc::clone(a), Rc::clone(b), merge_sets(a.conditions(), b.conditions()))
        )]
      },

      // Simplify the `div` operator
      //
      // If the dividend is zero, replace the value with a zero constant.
      // If the divisor is one, replace the value with the dividend.
      // If the dividend is smaller than the divisor, replace the value
      // with a zero constant.
      // If the dividend consists of a sum where one part is smaller than
      // divisor and the other part is a multiple of the divisor, simplify
      // `((x * y) + z) / y` to `x`.
      // If the dividend consists of (operand * divisor), replace the value
      // with operand. In other words, simplify `(x * y) / y` to `x`.
      Operation::Div => {
        if let Value::Constant(0, _) = a.borrow() {
          return vec![Rc::clone(a)];
        }

        if let Value::Constant(1, bconditions) = b.borrow() {
          return vec![Rc::new(a.clone_with_extra_conditions(bconditions))];
        }

        if a == b {
          return vec![Rc::new(
            Value::Constant(1, merge_sets(a.conditions(), b.conditions()))
          )];
        }

        if a.max_value() < b.min_value() {
          return vec![Rc::new(
            Value::Constant(0, merge_sets(a.conditions(), b.conditions()))
          )];
        }

        if let Value::Constant(bval, _) = b.borrow() {
          if let Value::Constant(aval, _) = a.borrow() {
            return vec![Rc::new(
              Value::Constant(aval / bval, merge_sets(a.conditions(), b.conditions()))
            )];
          }

          fn simplify_div(bval: i64, b: &Rc<Value>, val: &Rc<Value>) -> Rc<Value> {
            if val.max_value() < bval {
              // println!("simplifying 0 / {} to 0", bval);
              return Rc::new(
                Value::Constant(0, merge_sets(val.conditions(), b.conditions()))
              );
            }

            match val.borrow() {
              Value::Constant(c, conditions) => {
                // println!("simplifying {} / {} to {}", c, bval, c / bval);
                Rc::new(Value::Constant(c / bval, merge_sets(conditions, b.conditions())))
              },
              Value::Operation(Operation::Add, left, right, _) => {
                if left.is_multiple_of(bval) == Some(true) && right.is_multiple_of(bval) == Some(true) {
                  let new_left = simplify_div(bval, b, left);
                  let new_right = simplify_div(bval, b, right);
                  let new_conditions = merge_sets(new_left.conditions(), new_right.conditions());
                  // println!("simplifying ({} + {}) / {} to {} + {}", left, right, bval, new_left, new_right);
                  Rc::new(
                    Value::Operation(
                      Operation::Add,
                      new_left,
                      new_right,
                      new_conditions,
                    )
                  )
                } else if left.min_value() > 0 && left.max_value() < bval && right.is_multiple_of(bval) == Some(true) {
                  let res = simplify_div(bval, b, &Rc::new(right.clone_with_extra_conditions(&merge_sets(b.conditions(), left.conditions()))));
                  // println!("simplifying ({} + {}) / {} to {}", left, right, bval, res);
                  res
                } else if right.min_value() > 0 && right.max_value() < bval && left.is_multiple_of(bval) == Some(true) {
                  let res = simplify_div(bval, b, &Rc::new(left.clone_with_extra_conditions(&merge_sets(b.conditions(), right.conditions()))));
                  // println!("simplifying ({} + {}) / {} to {}", left, right, bval, res);
                  res
                } else {
                  Rc::new(Value::Operation(Operation::Div, Rc::clone(val), Rc::clone(b), merge_sets(val.conditions(), b.conditions())))
                }
              },

              Value::Operation(Operation::Mul, left, right, _) => {
                if left.static_value() == Some(bval) {
                  // println!("simplifying ({} * {}) / {} to {}", left, right, bval, right);
                  Rc::new(right.clone_with_extra_conditions(&merge_sets(left.conditions(), b.conditions())))
                } else if right.static_value() == Some(bval) {
                  // println!("simplifying ({} * {}) / {} to {}", left, right, bval, left);
                  Rc::new(left.clone_with_extra_conditions(&merge_sets(right.conditions(), b.conditions())))
                } else if left.is_multiple_of(bval) == Some(true) {
                  let new_left = simplify_div(bval, b, left);
                  let new_conditions = merge_sets(new_left.conditions(), right.conditions());
                  // println!("simplifying ({} * {}) / {} to {} * {}", left, right, bval, new_left, right);
                  Rc::new(
                    Value::Operation(
                      Operation::Mul,
                      new_left,
                      Rc::clone(right),
                      new_conditions,
                    )
                  )
                } else if right.is_multiple_of(bval) == Some(true) {
                  let new_right = simplify_div(bval, b, right);
                  let new_conditions = merge_sets(left.conditions(), new_right.conditions());
                  // println!("simplifying ({} * {}) / {} to {} * {}", left, right, bval, left, new_right);
                  Rc::new(
                    Value::Operation(
                      Operation::Mul,
                      Rc::clone(left),
                      new_right,
                      new_conditions,
                    )
                  )
                } else {
                  Rc::new(Value::Operation(Operation::Div, Rc::clone(val), Rc::clone(b), merge_sets(val.conditions(), b.conditions())))
                }
              },

              _ => Rc::new(Value::Operation(Operation::Div, Rc::clone(val), Rc::clone(b), merge_sets(val.conditions(), b.conditions()))),
            }
          }

          return vec![simplify_div(*bval, b, a)];
        }
        
        vec![Rc::new(
          Value::Operation(
            Operation::Div,
            Rc::clone(&a),
            Rc::clone(b),
            merge_sets(a.conditions(), b.conditions()),
          )
        )]
      },

      // Simplify the `mod` operator
      //
      // If the dividend is smaller than the modulus, replace the value
      // with the dividend.
      // If the dividend consists of a sum where one part is smaller than
      // divisor and the other part is a multiple of the divisor, simplify
      // `((x * y) + z) % y` to `z`.
      Operation::Mod => {
        if let Value::Constant(0, _) = a.borrow() {
          return vec![Rc::clone(a)];
        }

        if a.max_value() < b.min_value() {
          return vec![Rc::new(
            a.clone_with_extra_conditions(b.conditions())
          )];
        }

        if let Value::Constant(bval, bconditions) = b.borrow() {
          if let Value::Constant(aval, aconditions) = a.borrow() {
            return vec![Rc::new(
              Value::Constant(aval % bval, merge_sets(aconditions, bconditions))
            )];
          }

          fn simplify_mod(bval: i64, b: &Rc<Value>, val: &Rc<Value>) -> Rc<Value> {
            if val.max_value() < bval && val.min_value() > -bval {
              // println!("simplifying {} % {} to {}", val, bval, val);
              return Rc::new(val.clone_with_extra_conditions(b.conditions()));
            }

            match val.borrow() {
              Value::Constant(c, conditions) => {
                // println!("simplifying {} % {} to {}", c, bval, c % bval);
                Rc::new(Value::Constant(c % bval, merge_sets(conditions, b.conditions())))
              },
              Value::Operation(Operation::Add, left, right, conditions) => {
                if left.is_multiple_of(bval) == Some(true) {
                  let res = simplify_mod(bval, b, right).clone_with_extra_conditions(conditions);
                  // println!("simplifying ({} + {}) % {} to {}", left, right, bval, res);
                  Rc::new(res)
                } else if right.is_multiple_of(bval) == Some(true) {
                  let res = simplify_mod(bval, b, left).clone_with_extra_conditions(conditions);
                  println!("simplifying ({} + {}) % {} to {}", left, right, bval, res);
                  Rc::new(res)
                } else {
                  Rc::new(Value::Operation(Operation::Mod, Rc::clone(val), Rc::clone(b), merge_sets(val.conditions(), b.conditions())))
                }
              },

              Value::Operation(Operation::Mul, left, right, _) => {
                if left.is_multiple_of(bval) == Some(true) {
                  // println!("simplifying ({} * {}) % {} to 0", left, right, bval);
                  Rc::new(Value::Constant(0, merge_sets(left.conditions(), b.conditions())))
                } else if right.is_multiple_of(bval) == Some(true) {
                  // println!("simplifying ({} * {}) % {} to 0", left, right, bval);
                  Rc::new(Value::Constant(0, merge_sets(right.conditions(), b.conditions())))
                } else {
                  Rc::new(Value::Operation(Operation::Mod, Rc::clone(val), Rc::clone(b), merge_sets(val.conditions(), b.conditions())))
                }
              },

              _ => Rc::new(Value::Operation(Operation::Mod, Rc::clone(val), Rc::clone(b), merge_sets(val.conditions(), b.conditions()))),
            }
          }

          return vec![simplify_mod(*bval, b, a)];
        }

        vec![Rc::new(
          Value::Operation(
            Operation::Mod,
            Rc::clone(&a),
            Rc::clone(b),
            merge_sets(a.conditions(), b.conditions()),
          ),
        )]
      }
    }
  } else {
    vec![val]
  }
}

enum Register {
  W,
  X,
  Y,
  Z,
  Const(Rc<Value>),
}

impl Register {
  fn parse(val: &str) -> Register {
    match val {
      "w" => Register::W,
      "x" => Register::X,
      "y" => Register::Y,
      "z" => Register::Z,
      _ => Register::Const(Value::r#const(val.parse::<i64>().unwrap())),
    }
  }
}

impl std::fmt::Display for Register {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Register::W => write!(f, "w"),
      Register::X => write!(f, "x"),
      Register::Y => write!(f, "y"),
      Register::Z => write!(f, "z"),
      Register::Const(value) => write!(f, "{}", value.static_value().unwrap()),
    }
  }
}

struct Alu {
  next_input: usize,

  w: Rc<Value>,
  x: Rc<Value>,
  y: Rc<Value>,
  z: Rc<Value>,
}

impl Clone for Alu {
  fn clone(&self) -> Self {
    Alu {
      next_input: self.next_input,

      w: Rc::clone(&self.w),
      x: Rc::clone(&self.x),
      y: Rc::clone(&self.y),
      z: Rc::clone(&self.z),
    }
  }
}

impl Alu {
  fn get(&self, register: &Register) -> Rc<Value> {
    match register {
      Register::W => Rc::clone(&self.w),
      Register::X => Rc::clone(&self.x),
      Register::Y => Rc::clone(&self.y),
      Register::Z => Rc::clone(&self.z),
      Register::Const(v) => Rc::clone(v),
    }
  }

  fn with(&self, register: &Register, value: Rc<Value>) -> Alu {
    let mut clone = self.clone();

    match register {
      Register::W => {
        clone.w = value;
      },
      Register::X => {
        clone.x = value;
      },
      Register::Y => {
        clone.y = value;
      },
      Register::Z => {
        clone.z = value;
      },
      Register::Const(_) => panic!("can't assign to constant value"),
    }

    clone
  }

  fn execute(&self, instruction: &Instruction) -> Vec<Alu> {
    let target_register: &Register;
    let new_value: Rc<Value>;
    let mut next_input = self.next_input;

    match instruction {
      Instruction::Inp(a) => {
        target_register = a;
        new_value = Value::input(next_input);
        next_input += 1;
      },
      Instruction::Add(a, b) => {
        target_register = a;
        new_value = Value::op(Operation::Add, self.get(a), self.get(b));
      },
      Instruction::Mul(a, b) => {
        target_register = a;
        new_value = Value::op(Operation::Mul, self.get(a), self.get(b));
      },
      Instruction::Div(a, b) => {
        target_register = a;
        new_value = Value::op(Operation::Div, self.get(a), self.get(b));
      },
      Instruction::Mod(a, b) => {
        target_register = a;
        new_value = Value::op(Operation::Mod, self.get(a), self.get(b));
      },
      Instruction::Eql(a, b) => {
        target_register = a;
        new_value = Value::op(Operation::Eql, self.get(a), self.get(b));
      },
    };

    simplify(new_value).into_iter().map(|val| {
      let mut new_alu = self.with(target_register, val);
      new_alu.next_input = next_input;
      new_alu
    }).collect()
  }
}

enum Instruction {
  Inp(Register),
  Add(Register, Register),
  Mul(Register, Register),
  Div(Register, Register),
  Mod(Register, Register),
  Eql(Register, Register),
}

impl std::fmt::Display for Instruction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Instruction::Inp(a) => write!(f, "inp {}", a),
      Instruction::Add(a, b) => write!(f, "add {} {}", a, b),
      Instruction::Mul(a, b) => write!(f, "mul {} {}", a, b),
      Instruction::Div(a, b) => write!(f, "div {} {}", a, b),
      Instruction::Mod(a, b) => write!(f, "mod {} {}", a, b),
      Instruction::Eql(a, b) => write!(f, "eql {} {}", a, b),
    }
  }
}

impl Instruction {
  fn parse(line: String) -> Instruction {
    let parts = line.split(' ').collect::<Vec<_>>();

    match parts[0] {
      "inp" => {
        assert_eq!(parts.len(), 2);
        Instruction::Inp(Register::parse(parts[1]))
      },
      "add" => {
        assert_eq!(parts.len(), 3);
        Instruction::Add(Register::parse(parts[1]), Register::parse(parts[2]))
      },
      "mul" => {
        assert_eq!(parts.len(), 3);
        Instruction::Mul(Register::parse(parts[1]), Register::parse(parts[2]))
      },
      "div" => {
        assert_eq!(parts.len(), 3);
        Instruction::Div(Register::parse(parts[1]), Register::parse(parts[2]))
      },
      "mod" => {
        assert_eq!(parts.len(), 3);
        Instruction::Mod(Register::parse(parts[1]), Register::parse(parts[2]))
      },
      "eql" => {
        assert_eq!(parts.len(), 3);
        Instruction::Eql(Register::parse(parts[1]), Register::parse(parts[2]))
      },
      v => panic!("Unknown instruction \"{}\"", v),
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut instructions: Vec<Instruction> = lines.map(|line| {
      Instruction::parse(line.unwrap())
    }).collect();

    let empty = &Value::r#const(0);
    
    instructions.push(Instruction::Eql(Register::Z, Register::Const(Rc::clone(empty))));

    let mut alus = vec![Alu {
      next_input: 0,
      w: Rc::clone(empty),
      x: Rc::clone(empty),
      y: Rc::clone(empty),
      z: Rc::clone(empty),
    }];

    for instruction in instructions {
      alus = alus.into_iter().flat_map(|alu| alu.execute(&instruction)).collect();
    }

    alus = alus.into_iter().filter(|alu| {
      if let Value::Constant(1, _) = alu.z.borrow() {
        true
      } else {
        false
      }
    }).collect();

    println!("Ended up with {} alus\n", alus.len());

    for (i, alu) in alus.into_iter().enumerate() {
      println!("no. {}", i);
      println!("Conditions:");
      for cond in alu.z.conditions() {
        println!("  -> {}", cond);
      }
      println!("");

      let mut ranges = vec![(1, 9); 14];

      for cond in alu.z.conditions() {
        let mut idx1: usize = 0;
        let mut idx2: usize = 0;
        let mut plus: i64 = 0;
        
        if let Condition::Eq(left, right) = cond {
          if let Value::Input(i, _) = left.borrow() {
            idx1 = *i;

            if let Value::Operation(Operation::Add, sub1, sub2, _) = right.borrow() {
              if let Value::Input(j, _) = sub1.borrow() {
                idx2 = *j;
              } else {
                println!("Unexpected complex condition: {}", cond);
                break;
              }
              if let Value::Constant(c, _) = sub2.borrow() {
                plus = *c;
              } else {
                println!("Unexpected != condition: {}", cond);
                break;
              }
            } else {
              println!("Unexpected complex condition: {}", cond);
              break;
            }
          } else if let Value::Input(i, _) = right.borrow() {
            idx1 = *i;

            if let Value::Operation(Operation::Add, sub1, sub2, _) = left.borrow() {
              if let Value::Input(j, _) = sub1.borrow() {
                idx2 = *j;
              } else {
                println!("Unexpected complex condition: {}", cond);
                break;
              }
              if let Value::Constant(c, _) = sub2.borrow() {
                plus = *c;
              } else {
                println!("Unexpected != condition: {}", cond);
                break;
              }
            } else {
              println!("Unexpected complex condition: {}", cond);
              break;
            }
          }
        } else {
          println!("Unexpected != condition: {}", cond);
          break;
        }

        if plus < 0 {
          let tmp = idx1;
          idx1 = idx2;
          idx2 = tmp;

          plus = plus.abs();
        }

        // number[idx1] = number[idx2] + plus
        // -> max value for idx2 is max of idx1 - plus
        // -> min value for idx1 is min of idx2 - plus

        let (_, max1) = ranges[idx1];
        let (min2, _) = ranges[idx2];

        let (ref mut min1, _) = ranges.get_mut(idx1).unwrap();
        *min1 = cmp::max(*min1, min2 + plus);

        let (_, ref mut max2) = ranges.get_mut(idx2).unwrap();
        *max2 = cmp::min(*max2, max1 - plus);
      }

      println!(
        "max: {}",
        ranges.iter().map(|(_, max)| max.to_string()).collect::<String>()
      );
      println!(
        "min: {}",
        ranges.iter().map(|(min, _)| min.to_string()).collect::<String>()
      );
      println!("");
    }
  } else {
    panic!("Failed to read file");
  }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}