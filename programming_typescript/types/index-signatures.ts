// The syntax `[key: T]: U` is called an *index signature*
// and is used to specify multiple keys of an object. The
// index signature specifies that an object must only contain
// properties with keys of type `T` and values of type `U`.
const englishToGerman: {
  [englishWord: string]: string
} = {
  apple: "Apfel",
  banana: "Banane",
  cherry: "Kirsche",
};

// INFO: The type `T` of the keys must be assignable to
// either number or string (since keys of objects are strings
// with the exceptions of arrays which are special objects).

// INFO: You can use whatever name for the key you want.
