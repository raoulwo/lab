// TS supports enums that can map from strings to numbers/strings.
enum Language {
  English, // 0 is inferred
  Spanish,
  German
};

// You can split enum declarations, TS can only infer values for 
// one declaration however.
enum Foo {
  A
};

enum Foo {
  B = 1 // Error when omitting the 1
};


// Enums can have strings for values (can mix strings and numbers)
enum Color {
  Red = "#ff0000",
  Blue = 0x0000ff
};

// WARNING: Enums let you access values by value and key, it doesn't
// prevent you from accessing non-existent values.
console.log(Color['Red']);
console.log(Color[5]);

// A solution against this is the `const enum` type:
const enum ConstColor {
  Red = "#ff0000",
  Blue = 0x0000ff
};

console.log(ConstColor['Red']);
// v const enum members can't be accessed using []
// console.log(ConstColor[5]);

// WARNING: There is a flag `preserveConstEnums` that when set to false
// will cause `const enums` to be inlined which can cause problems in
// scenarios where you're using const enums from third party code. If
// they update their enum members while you haven't compiled your TS code
// since the different version might point to different values at runtime.
// So avoid inlining const enums for programs that you plan to publish on npm.
