// Classes and interfaces support generic type params.
// You can define them at different scopes, the
// whole class/interface or just single methods:
class MyMap<K, V> { // Class-scoped generics
  get(key: K): V { // Class-scoped

  }

  set(key: K, value: V): void { // Class-scoped

  }

  // Method-scoped K1 and V1
  merge<K1, V1>(map: MyMap<K1, V1>): MyMap<K | K1, V | V1> {

  }

  // Static methods don't have access to class-scope generic
  // type params (makes sense, since they are like a classes
  // instance variables)
  static of<K, V>(k: K, v: V): MyMap<K, V> {
  }
}
