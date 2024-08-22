// `|` can be used for the *union* of types
// `&` can be used for the *intersection* of types

type Cat = { name: string, purrs: boolean };
type Dog = { name: string, barks: boolean };

type CatOrDogOrBoth = Cat | Dog;
type CatAndDog = Cat & Dog;

// The type CatOrDogOrBoth is satisfied as long as the value
// lies in the union of the types Cat and Dog. This means the value
// must be of type Cat or Dog or both.
let a: CatOrDogOrBoth = {
  name: 'cat',
  purrs: true
};

a = {
  name: 'dog',
  barks: true
};

a = {
  name: 'cat',
  purrs: true,
  barks: true
};

// The type CatAndDog is satisfied as long as the value
// lies in the intersection of the types Cat and Dog. This means the value
// must be of type Cat and Dog.
let b: CatAndDog = {
  name: 'cat',
  purrs: true,
};

// INFO: You have to think of the set-theoretic intersection in terms
// of the intersection of types, meaning both Cat **and** Dog must be
// satisfied. Don't think about it as the intersection of the object members.
