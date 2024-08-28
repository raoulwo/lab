// INFO: Use the flag `strictBindCallApply` (part of `strict` suite)

// `this` in JS is *unbound*, this means it's defined for every function
// not just for object methods. The value of `this` is **always** the
// *receiver* of the function/method, the receiver is the object on the
// lhs of the `.` member access syntax.

function fancyDate() {
  return `${this.getDate()}/${this.getMonth()}/${this.getFullYear()}`;
}

// The function above is intended to be used this way:
fancyDate.call(new Date);
// However, when you forget to bind `this` to a `Date` object, you'll
// get a runtime exception.

// To prevent this, specify a param with name `this` as the first params
// of the function, that way TS can enforce the type of `this` any time
// the function is called. TS treats the `this` param differently from
// normal params, it is a reserved word with special meaning:
function fancyDateTypeSafe(this: Date) {
  return `${this.getDate()}/${this.getMonth()}/${this.getFullYear()}`;
}
