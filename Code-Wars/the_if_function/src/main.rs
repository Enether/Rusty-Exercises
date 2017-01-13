/*
Who likes keywords? Nobody likes keywords, so why use them?

You know what keyword I use too much? if! We should make a function called _if, with its arguments as a logical test and two functions/lambdas where the first function is executed if the boolean is true, and the second if it's false, like an if/else statement, so that we don't have to mess around with those nasty keywords! Even so, It should support truthy/falsy types just like the keyword.
*/
fn _if<T, F1, F2>(cond: bool, mut then: F1, mut els: F2) -> T
  where F1: FnMut() -> T, F2: FnMut() -> T
{
    if cond {
        then()
    }
    else {
        els()
    }
}

#[test]
fn should_support_true() {
  assert_eq!(_if(true, || 1, || 2), 1);
}

#[test]
fn should_support_false() {
  assert_eq!(_if(false, || 1, || 2), 2);
}

#[test]
fn should_support_false_other_type() {
  assert_eq!(_if(false, || 'a', || 'b'), 'b');
  assert_eq!(_if(true, || "True", || "False"), "True");
}

#[test]
fn should_support_closures() {
  let mut true_was_set = false;
  let result = _if(true, || true_was_set = true, || panic!("Fail"));
  assert!(true_was_set);
  assert_eq!(result, ())
}